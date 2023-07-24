use lazy_static::lazy_static;
use starknet::core::types::FieldElement;

lazy_static! {
    pub static ref ETH_ADDRESS: FieldElement =
        FieldElement::from_hex_be("0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7").unwrap();
    pub static ref ONE_ETH: FieldElement = FieldElement::from(1000000000000000000u64);
}
