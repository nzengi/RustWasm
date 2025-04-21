use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::TransactionData;
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
use web_sys::{console, window};

/// Ethereum integration module for interacting with Ethereum networks.
/// This module provides functions to interact with Ethereum providers,
/// send transactions, and query blockchain data.

/// Get the current Ethereum provider (MetaMask or other web3 provider)
pub fn get_provider() -> Result<JsValue, JsValue> {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("Could not access window"))?;
        
        if !js_sys::Reflect::has(&window, &JsValue::from_str("ethereum")).unwrap_or(false) {
            return Err(JsValue::from_str("Ethereum provider not found"));
        }

        let ethereum = js_sys::Reflect::get(&window, &JsValue::from_str("ethereum"))?;
        Ok(ethereum)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // For non-WASM environments, return a mock provider for testing
        Err(JsValue::from_str("Provider only available in browser environment"))
    }
}

/// Get the connected accounts from the Ethereum provider
pub async fn get_accounts() -> Result<Vec<String>, JsValue> {
    let provider = get_provider()?;
    
    #[cfg(target_arch = "wasm32")]
    {
        let request_fn = js_sys::Reflect::get(&provider, &JsValue::from_str("request"))?
            .dyn_into::<js_sys::Function>()?;
        
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &JsValue::from_str("method"), &JsValue::from_str("eth_accounts"))?;
        
        let promise = request_fn.call1(&provider, &args)?;
        let promise = js_sys::Promise::from(promise);
        let accounts = wasm_bindgen_futures::JsFuture::from(promise).await?;
        
        if let Ok(accounts_array) = accounts.dyn_into::<js_sys::Array>() {
            let mut result = Vec::new();
            for i in 0..accounts_array.length() {
                if let Some(account) = accounts_array.get(i).as_string() {
                    result.push(account);
                }
            }
            Ok(result)
        } else {
            Err(JsValue::from_str("Failed to parse accounts"))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // For non-WASM environments, return mock accounts for testing
        Ok(vec!["0x0000000000000000000000000000000000000000".to_string()])
    }
}

/// Connect to the Ethereum provider and request access to accounts
pub async fn connect() -> Result<Vec<String>, JsValue> {
    let provider = get_provider()?;
    
    #[cfg(target_arch = "wasm32")]
    {
        let request_fn = js_sys::Reflect::get(&provider, &JsValue::from_str("request"))?
            .dyn_into::<js_sys::Function>()?;
        
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &JsValue::from_str("method"), &JsValue::from_str("eth_requestAccounts"))?;
        
        let promise = request_fn.call1(&provider, &args)?;
        let promise = js_sys::Promise::from(promise);
        let accounts = wasm_bindgen_futures::JsFuture::from(promise).await?;
        
        if let Ok(accounts_array) = accounts.dyn_into::<js_sys::Array>() {
            let mut result = Vec::new();
            for i in 0..accounts_array.length() {
                if let Some(account) = accounts_array.get(i).as_string() {
                    result.push(account);
                }
            }
            Ok(result)
        } else {
            Err(JsValue::from_str("Failed to parse accounts"))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // For non-WASM environments, return mock accounts for testing
        Ok(vec!["0x0000000000000000000000000000000000000000".to_string()])
    }
}

/// Get the current network ID from the Ethereum provider
pub async fn get_network_id() -> Result<u64, JsValue> {
    let provider = get_provider()?;
    
    #[cfg(target_arch = "wasm32")]
    {
        let request_fn = js_sys::Reflect::get(&provider, &JsValue::from_str("request"))?
            .dyn_into::<js_sys::Function>()?;
        
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &JsValue::from_str("method"), &JsValue::from_str("net_version"))?;
        
        let promise = request_fn.call1(&provider, &args)?;
        let promise = js_sys::Promise::from(promise);
        let network_id = wasm_bindgen_futures::JsFuture::from(promise).await?;
        
        if let Some(id_str) = network_id.as_string() {
            match id_str.parse::<u64>() {
                Ok(id) => Ok(id),
                Err(_) => Err(JsValue::from_str("Failed to parse network ID")),
            }
        } else {
            Err(JsValue::from_str("Failed to get network ID"))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // For non-WASM environments, return a mock network ID for testing
        Ok(1u64) // Ethereum mainnet
    }
}

// Get Ethereum network information
#[wasm_bindgen]
pub async fn get_network_info() -> Result<JsValue, JsValue> {
    #[cfg(target_arch = "wasm32")]
    {
        let window = window().ok_or_else(|| JsValue::from_str("Window not available"))?;
        
        // Check for the existence of Ethereum provider
        if !js_sys::Reflect::has(&window, &JsValue::from_str("ethereum")).unwrap_or(false) {
            return Err(JsValue::from_str("Ethereum provider not found"));
        }

        // Get the Ethereum object
        let ethereum = js_sys::Reflect::get(&window, &JsValue::from_str("ethereum"))?;
        
        // Call the request method to get chainId
        let request_fn = js_sys::Reflect::get(&ethereum, &JsValue::from_str("request"))?;
        let request_fn = js_sys::Function::from(request_fn);
        
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &JsValue::from_str("method"), &JsValue::from_str("eth_chainId"))?;
        
        // Create a Promise object and convert it to JsFuture
        let promise = request_fn.call1(&ethereum, &args)?;
        let promise = js_sys::Promise::from(promise);
        let chain_id_result = wasm_bindgen_futures::JsFuture::from(promise).await?;
        
        // Return directly as JsValue to be processed on JavaScript side
        let network_info = js_sys::Object::new();
        js_sys::Reflect::set(&network_info, &JsValue::from_str("chainId"), &chain_id_result)?;
        
        Ok(network_info.into())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Return fixed network info for testing (Ethereum mainnet)
        let network_info = js_sys::Object::new();
        js_sys::Reflect::set(&network_info, &JsValue::from_str("chainId"), &JsValue::from_str("0x1"))?;
        Ok(network_info.into())
    }
}

// Send transaction
#[wasm_bindgen]
pub async fn send_transaction(tx_data: TransactionData) -> Result<String, JsValue> {
    #[cfg(target_arch = "wasm32")]
    {
        let window = window().ok_or_else(|| JsValue::from_str("Window not available"))?;
        
        // Check for the existence of Ethereum provider
        if !js_sys::Reflect::has(&window, &JsValue::from_str("ethereum")).unwrap_or(false) {
            return Err(JsValue::from_str("Ethereum provider not found"));
        }

        // Get the Ethereum object
        let ethereum = js_sys::Reflect::get(&window, &JsValue::from_str("ethereum"))?;
        
        // Create transaction object
        let tx_object = js_sys::Object::new();
        js_sys::Reflect::set(&tx_object, &JsValue::from_str("from"), &JsValue::from_str(&tx_data.from))?;
        js_sys::Reflect::set(&tx_object, &JsValue::from_str("to"), &JsValue::from_str(&tx_data.to))?;
        js_sys::Reflect::set(&tx_object, &JsValue::from_str("value"), &JsValue::from_str(&tx_data.value))?;
        js_sys::Reflect::set(&tx_object, &JsValue::from_str("gas"), &JsValue::from_f64(tx_data.gas as f64))?;
        js_sys::Reflect::set(&tx_object, &JsValue::from_str("data"), &JsValue::from_str(&tx_data.data))?;
        
        // Send the transaction
        let request_fn = js_sys::Reflect::get(&ethereum, &JsValue::from_str("request"))?;
        let request_fn = js_sys::Function::from(request_fn);
        
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &JsValue::from_str("method"), &JsValue::from_str("eth_sendTransaction"))?;
        js_sys::Reflect::set(&args, &JsValue::from_str("params"), &js_sys::Array::of1(&tx_object))?;
        
        // Create a Promise object and convert it to JsFuture
        let promise = request_fn.call1(&ethereum, &args)?;
        let promise = js_sys::Promise::from(promise);
        let tx_result = wasm_bindgen_futures::JsFuture::from(promise).await?;
        
        // Return the transaction hash
        let tx_hash = tx_result.as_string().ok_or_else(|| JsValue::from_str("Failed to get transaction hash"))?;
        
        Ok(tx_hash)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Return a fixed transaction hash for testing
        Ok("0x".to_string() + &"1234567890abcdef".repeat(4))
    }
}
