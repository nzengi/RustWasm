use wasm_bindgen::prelude::*;
use serde_json::{Value, Error as JsonError};

#[cfg(target_arch = "wasm32")]
use web_sys::console;

// Convert hex format value to decimal format
#[wasm_bindgen]
pub fn hex_to_decimal(hex_value: &str) -> Result<String, JsValue> {
    if !hex_value.starts_with("0x") {
        return Err(JsValue::from_str("Not a valid hex string, must start with 0x"));
    }

    let hex_str = &hex_value[2..]; // Remove "0x" prefix
    match u128::from_str_radix(hex_str, 16) {
        Ok(num) => Ok(num.to_string()),
        Err(_) => Err(JsValue::from_str("Failed to convert hex to decimal"))
    }
}

// Convert decimal value to hex format
#[wasm_bindgen]
pub fn decimal_to_hex(decimal_value: &str) -> Result<String, JsValue> {
    match decimal_value.parse::<u128>() {
        Ok(num) => Ok(format!("0x{:x}", num)),
        Err(_) => Err(JsValue::from_str("Failed to convert decimal to hex"))
    }
}

// Convert Wei to Ether (1 Ether = 10^18 Wei)
#[wasm_bindgen]
pub fn wei_to_ether(wei_value: &str) -> Result<String, JsValue> {
    match wei_value.parse::<u128>() {
        Ok(wei) => {
            // 1 Ether = 10^18 Wei
            let ether = wei as f64 / 1_000_000_000_000_000_000.0;
            Ok(ether.to_string())
        },
        Err(_) => Err(JsValue::from_str("Failed to convert wei to ether"))
    }
}

// Check if an Ethereum address is valid
#[wasm_bindgen]
pub fn is_valid_eth_address(address: &str) -> bool {
    // Ethereum address should start with 0x and be 42 characters total
    if !address.starts_with("0x") || address.len() != 42 {
        return false;
    }

    // Characters after 0x should be valid hex characters
    let hex_part = &address[2..];
    hex_part.chars().all(|c| c.is_digit(16))
}

// Parse JSON string
pub fn parse_json(json_str: &str) -> Result<Value, JsonError> {
    serde_json::from_str(json_str)
}

// Logging functions (for debugging)
#[wasm_bindgen]
pub fn log_info(message: &str) {
    #[cfg(target_arch = "wasm32")]
    console::log_1(&JsValue::from_str(message));
    
    #[cfg(not(target_arch = "wasm32"))]
    println!("INFO: {}", message);
}

#[wasm_bindgen]
pub fn log_warning(message: &str) {
    #[cfg(target_arch = "wasm32")]
    console::warn_1(&JsValue::from_str(message));
    
    #[cfg(not(target_arch = "wasm32"))]
    println!("WARNING: {}", message);
}

// Calculate gas limit (example function)
#[wasm_bindgen]
pub fn estimate_gas_limit(data_size: u32) -> u64 {
    // Simple example: calculate gas limit based on data size
    // In real scenarios, more complex calculations may be required
    let base_gas = 21000; // Base transaction gas
    let data_gas = data_size as u64 * 68; // Gas per data byte
    
    base_gas + data_gas
}
