extern crate alloc;

use fuel_indexer_macros::indexer;
use fuel_indexer_plugin::types::*;
use fuels_core::*;

pub fn get_zero_address() -> Address {
    Address::new([0u8; 32])
}

pub fn get_zero_contract() -> ContractId {
    ContractId::new([0u8; 32])
}

pub fn get_u64_from_address(address: Address) -> u64 {
    let mut buff = [0u8; 8];
    buff.copy_from_slice(&address.to_string().as_bytes()[..8]);
    u64::from_le_bytes(buff)
}

pub fn get_address_from_identity(identity: Identity) -> Address {
    match identity {
        Identity::Address(address) => address,
        Identity::ContractId(_) => get_zero_address(),
    }
}

pub fn get_i64_from_u64(val: u64) -> i64 {
    val.try_into().unwrap()
}

pub fn get_u64_from_string_vec(values: Vec<String>) -> u64 {
    let result = values.join("_");
    let mut buff = [0u8; 8];
    buff.copy_from_slice(&result.as_bytes()[..8]);
    u64::from_le_bytes(buff)
}

// noinspection RsUnresolvedReference
#[indexer(manifest = "../fuel-indexers/greetings/indexer/manifest.yaml")]
pub mod nft_indexer_module {

    pub fn handle_greeting_event(event: NewGreeting) {

    }
}
