pub mod args;
pub mod constants;
pub mod kakarot;

use args::DeployArgs;
use clap::Parser;
use dotenv::dotenv;
use eyre::Result;
use kakarot::{deploy_account, fund_address};
use starknet::accounts::SingleOwnerAccount;
use starknet::core::types::{FieldElement, StarknetError};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider, ProviderError};
use starknet::signers::{LocalWallet, SigningKey};
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let args = DeployArgs::parse();

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()?;
    tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;

    let url = Url::parse("http://127.0.0.1:9944")?; // url to Madara
    let provider = JsonRpcClient::new(HttpTransport::new(url.clone()));
    let account_provider = JsonRpcClient::new(HttpTransport::new(url));

    let pk = FieldElement::from_hex_be("0x00c1cf1490de1352865301bb8705143f3ef938f97fdf892f1090dcb5ac7bcd1d")?;
    let signer = LocalWallet::from_signing_key(SigningKey::from_secret_scalar(pk));
    let wallet_address = FieldElement::from_hex_be("0x3")?;
    let account = SingleOwnerAccount::new(
        account_provider,
        signer,
        wallet_address,
        FieldElement::from_hex_be("0x534e5f474f45524c49")?,
    );

    log::info!("Deploying EOA account at evm address {:?}", args.evm_address);
    let result = deploy_account(&provider, &account, &args).await?;

    if result.transaction_hash != FieldElement::ZERO {
        log::info!("Waiting on tx with hash {:?}", &result.transaction_hash);
        loop {
            let res = provider.get_transaction_by_hash(result.transaction_hash).await;
            match res {
                Ok(_) => break,
                Err(err) => match err {
                    ProviderError::StarknetError(StarknetError::TransactionHashNotFound) => {
                        std::thread::sleep(std::time::Duration::from_secs(1))
                    }
                    _ => panic!("error while deploying eoa {:?}", err),
                },
            }
        }
    }
    log::info!("Deployed EOA");

    log::info!("Funding EOA account at evm address {:?} with one ETH", args.evm_address);
    fund_address(&provider, &account, &args).await?;

    Ok(())
}
