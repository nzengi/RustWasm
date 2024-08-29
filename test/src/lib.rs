use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use url::Url;
use std::sync::Arc;
use log::info;

#[wasm_bindgen]
pub fn get_eth_balance(address: &str) -> Result<String, JsValue> {
    let address = address.to_string();
    let (tx, rx) = std::sync::mpsc::channel();

    spawn_local(async move {
        info!("Attempting to connect to the Ethereum network...");

        let url = Url::parse("https://mainnet.infura.io/v3/Your_Api_key_Infura")
            .expect("Invalid URL");
        
        let provider = Provider::new(Http::new(url));
        let client = Arc::new(provider);

        info!("Parsing Ethereum address: {}", address);
        let address_result: Result<Address, _> = address.parse();
        match address_result {
            Ok(address) => {
                info!("Address parsed successfully: {:?}", address);
                let balance_result = client.get_balance(address, None).await;
                match balance_result {
                    Ok(balance) => {
                        let balance_in_eth = ethers::utils::format_units(balance, "ether").unwrap_or_else(|_| "0".into());
                        info!("Balance fetched successfully: {} ETH", balance_in_eth);
                        tx.send(Ok(balance_in_eth)).expect("Failed to send balance");
                    }
                    Err(e) => {
                        info!("Failed to fetch balance: {:?}", e);
                        tx.send(Err(JsValue::from_str(&format!("Failed to fetch balance: {:?}", e)))).expect("Failed to send error");
                    }
                }
            }
            Err(e) => {
                info!("Failed to parse address: {:?}", e);
                tx.send(Err(JsValue::from_str("Invalid Ethereum address format"))).expect("Failed to send error");
            }
        }
    });

    rx.recv().unwrap_or_else(|_| Err(JsValue::from_str("Failed to receive balance from async task")))
}
