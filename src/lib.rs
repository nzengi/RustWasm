use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// Import modules
mod eth_integration;
mod utils;
mod bindings;
mod contract;

// Export Ethereum integration
pub use eth_integration::*;
pub use crate::contract::{
    AbiItem, Contract, ERC20Token, 
    ContractEventFilter, StateMutability, ContractDeployer
};

// Basic web connection functions
#[cfg(target_arch = "wasm32")]
use web_sys::{console, window};

// Connect to Ethereum implemented function
#[wasm_bindgen]
pub async fn connect_to_ethereum() -> Result<String, JsValue> {
    match eth_integration::connect().await {
        Ok(accounts) => {
            if accounts.is_empty() {
                return Ok("Connected to Ethereum but no accounts available".to_string());
            }
            Ok("Connected to Ethereum!".to_string())
        },
        Err(e) => Err(e),
    }
}

// Get Ethereum accounts function
#[wasm_bindgen]
pub async fn get_ethereum_accounts() -> Result<JsValue, JsValue> {
    match eth_integration::connect().await {
        Ok(accounts) => {
            let accounts_array = js_sys::Array::new();
            for account in accounts {
                accounts_array.push(&JsValue::from_str(&account));
            }
            Ok(accounts_array.into())
        },
        Err(e) => Err(e),
    }
}

// This function will be called from JavaScript and display an alert
#[wasm_bindgen]
pub fn greet() {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(win) = window() {
            win.alert_with_message("Hello, WebAssembly!").unwrap();
        } else {
            console::error_1(&"Could not get window object".into());
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("Hello, Rust Native!");
    }
}

// Another example function that will be called from JavaScript
#[wasm_bindgen]
pub fn another_function() -> i32 {
    42
}

// Additional function that returns an error if dividing by zero
#[wasm_bindgen]
pub fn divide(x: i32, y: i32) -> Result<i32, JsValue> {
    if y == 0 {
        Err(JsValue::from_str("Division by zero error"))
    } else {
        Ok(x / y)
    }
}

// Data structures for Ethereum transactions
#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct TransactionData {
    from: String,
    to: String,
    value: String,
    gas: u64,
    data: String,
}

// Structure to receive data from JavaScript
#[wasm_bindgen]
impl TransactionData {
    #[wasm_bindgen(constructor)]
    pub fn new(from: String, to: String, value: String, gas: u64, data: String) -> TransactionData {
        TransactionData {
            from,
            to,
            value,
            gas,
            data,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn from(&self) -> String {
        self.from.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn to(&self) -> String {
        self.to.clone()
    }
}

// Helper function to pass error messages to JavaScript
#[wasm_bindgen]
pub fn log_error(error_msg: &str) {
    #[cfg(target_arch = "wasm32")]
    console::error_1(&JsValue::from_str(error_msg));
    
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("Error: {}", error_msg);
}
