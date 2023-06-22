#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use kakarot_rpc_core::client::client_api::KakarotClient;
    use kakarot_rpc_core::mock::wiremock_utils::setup_mock_client_crate;
    use kakarot_rpc_core::models::convertible::ConvertibleStarknetBlock;
    use kakarot_rpc_core::models::BlockWithTxs;
    use reth_primitives::H256;
    use starknet::core::types::{BlockId, BlockTag, FieldElement};
    use starknet::providers::Provider;

    #[tokio::test]
    async fn test_starknet_block_to_eth_block() {
        let client = setup_mock_client_crate().await;
        let starknet_client = client.inner();
        let starknet_block = starknet_client.get_block_with_txs(BlockId::Tag(BlockTag::Latest)).await.unwrap();
        let eth_block = BlockWithTxs::new(starknet_block).to_eth_block(&client).await.unwrap();

        // TODO: Add more assertions & refactor into assert helpers
        // assert helpers should allow import of fixture file
        assert_eq!(
            eth_block.header.hash,
            Some(H256::from_slice(
                &FieldElement::from_hex_be("0x449aa33ad836b65b10fa60082de99e24ac876ee2fd93e723a99190a530af0a9")
                    .unwrap()
                    .to_bytes_be()
            ))
        )
    }

    #[tokio::test]
    async fn test_starknet_transaction_by_hash() {
        let client = setup_mock_client_crate().await;
        let starknet_tx = client
            .transaction_by_hash(
                H256::from_str("0x03204b4c0e379c3a5ccb80d08661d5a538e95e2960581c9faf7ebcf8ff5a7d3c").unwrap(),
            )
            .await;
        assert!(starknet_tx.is_ok());
    }
}