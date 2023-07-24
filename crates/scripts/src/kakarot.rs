use eyre::Result;
use kakarot_rpc_core::contracts::kakarot::KakarotContract;
use starknet::accounts::{Call, Execution, SingleOwnerAccount};
use starknet::core::types::{BlockId, BlockTag, FieldElement, InvokeTransactionResult};
use starknet::macros::selector;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::LocalWallet;

use crate::args::DeployArgs;
use crate::constants::{ETH_ADDRESS, ONE_ETH};

pub async fn deploy_account(
    provider: &JsonRpcClient<HttpTransport>,
    account: &SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    args: &DeployArgs,
) -> Result<InvokeTransactionResult> {
    let kakarot = init_kakarot(args)?;

    let evm_address = FieldElement::from_hex_be(&args.evm_address)?;
    if is_deployed(provider, kakarot, evm_address).await.is_ok() {
        return Ok(InvokeTransactionResult { transaction_hash: FieldElement::ZERO });
    }

    let kakarot_address = FieldElement::from_hex_be(&args.kakarot_address)?;
    let execution = Execution::new(
        vec![Call {
            to: kakarot_address,
            selector: selector!("deploy_externally_owned_account"),
            calldata: vec![evm_address],
        }],
        &account,
    );

    Ok(execution.send().await?)
}

fn init_kakarot(args: &DeployArgs) -> Result<KakarotContract<JsonRpcClient<HttpTransport>>> {
    let kakarot_address = FieldElement::from_hex_be(&args.kakarot_address)?;
    let proxy_account_class_hash = FieldElement::from_hex_be(&args.proxy_account_class_hash)?;
    Ok(KakarotContract::new(kakarot_address, proxy_account_class_hash))
}

async fn is_deployed(
    provider: &JsonRpcClient<HttpTransport>,
    kakarot: KakarotContract<JsonRpcClient<HttpTransport>>,
    evm_address: FieldElement,
) -> Result<()> {
    let block_id = BlockId::Tag(BlockTag::Latest);

    let starknet_address = kakarot.compute_starknet_address(provider, &evm_address, &block_id).await.unwrap();
    provider.get_class_at(block_id, starknet_address).await?;
    Ok(())
}

pub async fn fund_address(
    provider: &JsonRpcClient<HttpTransport>,
    account: &SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    args: &DeployArgs,
) -> Result<()> {
    let block_id = BlockId::Tag(BlockTag::Latest);
    let evm_address = FieldElement::from_hex_be(&args.evm_address)?;

    let kakarot = init_kakarot(args)?;

    let starknet_address = kakarot.compute_starknet_address(provider, &evm_address, &block_id).await?;

    let execution = Execution::new(
        vec![Call {
            to: *ETH_ADDRESS,
            selector: selector!("transfer"),
            calldata: vec![starknet_address, *ONE_ETH, FieldElement::from(0u8)],
        }],
        &account,
    );

    execution.send().await?;

    Ok(())
}
