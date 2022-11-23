extern crate log;
extern crate pretty_env_logger;
extern crate rand;

use std::env;
use std::str::FromStr;

use dotenv::dotenv;
use fuel_tx::Address;
use fuels::prelude::{Bech32ContractId, Contract, ContractId, Provider, TxParameters, WalletUnlocked};
use fuels::signers::fuel_crypto::SecretKey;
use fuels_abigen_macro::abigen;
use fuels_core::Identity;
use fuels_core::parameters::StorageConfiguration;
use fuels_core::types::SizedAsciiString;

abigen!(
    Greeting,
    "../contracts/out/debug/contracts-abi.json"
);

async fn get_contract_id(wallet: &WalletUnlocked) -> Bech32ContractId {
    let contract_address = env::var("CONTRACT_ADDRESS");
    if !contract_address.is_err() {
        println!("Using contract address from .env");
        return Bech32ContractId::from(ContractId::from_str(&*contract_address.unwrap()).unwrap());
    }
    let bin_path = "../contracts/out/debug/contracts.bin".to_string();
    let contract_id = Contract::deploy(
        &bin_path,
        wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "../contracts/out/debug/contracts-storage_slots.json".to_string()
        )),
    )
        .await
        .unwrap();

    contract_id
}

async fn setup_provider_and_wallet() -> (Provider, WalletUnlocked) {
    let address = match env::var("NODE_URL") {
        Ok(val) => val,
        Err(_) => "127.0.0.1:4000".to_string(),
    };
    println!("Connected to node on url : {}", address);
    let provider = Provider::connect(&address).await.unwrap();

    let primary_private_key = env::var("PRIVATE_KEY").unwrap();
    let secret = SecretKey::from_str(&primary_private_key).unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret, Some(provider.clone()));

    (provider, wallet)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let (_provider, wallet) = setup_provider_and_wallet().await;
    let contract_id: Bech32ContractId = get_contract_id(&wallet).await;
    let contract: Greeting = Greeting::new(contract_id.clone(), wallet.clone());

    println!("Contract address: 0x{}", contract_id.hash);
    println!("Address of the owner: 0x{}", wallet.address().hash);

    let mut tx_params = TxParameters::default();
    tx_params.gas_price = 1;

    let contract_methods = contract.methods();

    let greetings = SizedAsciiString::try_from("Hello atul").unwrap();
    let greet_result = contract_methods.greet(1, greetings.clone()).tx_params(tx_params).call().await;
    assert_eq!(greet_result.is_err(), false, "Greet function call failed");
    greet_result.as_ref().unwrap();

    let _ = contract_methods.greet(2, greetings.clone()).tx_params(tx_params).call().await;

    Ok(())
}
