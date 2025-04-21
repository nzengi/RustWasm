use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::utils::hex_to_decimal;
use std::collections::HashMap;
use serde_wasm_bindgen;

// Re-export submodules
mod abi;
mod erc20;
mod events;
mod utils;
mod contract;
mod deploy;

// Re-export types and functions
pub use abi::*;
pub use erc20::*;
pub use events::*;
pub use utils::*;
pub use contract::{Function, Event, Parameter, EventParameter, StateMutability};
pub use deploy::ContractDeployer;
// Re-export the internal Contract as public Contract 
pub use self::contract::Contract as ContractImpl;

/// Represents a deployed smart contract on the Ethereum blockchain.
/// Provides methods for interacting with the contract functions and events.
#[wasm_bindgen]
pub struct Contract {
    address: String,
    abi: String,
    functions: HashMap<String, Function>,
    events: HashMap<String, Event>,
}

#[wasm_bindgen]
impl Contract {
    /// Creates a new Contract instance from an ABI and address
    #[wasm_bindgen(constructor)]
    pub fn new(address: String, abi: String) -> Result<Contract, JsValue> {
        // Parse ABI
        let abi_items: Vec<AbiItem> = match serde_json::from_str(&abi) {
            Ok(items) => items,
            Err(e) => return Err(JsValue::from_str(&format!("Failed to parse ABI: {}", e))),
        };

        // Build function and event maps
        let mut functions = HashMap::new();
        let mut events = HashMap::new();

        for item in abi_items {
            match item.r#type.as_str() {
                "function" => {
                    if let Some(name) = item.name {
                        let state_mutability = match item.state_mutability.as_deref() {
                            Some("view") => StateMutability::View,
                            Some("pure") => StateMutability::Pure,
                            Some("payable") => StateMutability::Payable,
                            _ => StateMutability::Nonpayable,
                        };

                        let inputs = item.inputs.unwrap_or_default().into_iter().map(|input| {
                            Parameter {
                                name: input.name,
                                r#type: input.r#type,
                                components: input.components.map(|comps| {
                                    comps.into_iter().map(|c| Parameter {
                                        name: c.name,
                                        r#type: c.r#type,
                                        components: None,
                                    }).collect()
                                }),
                            }
                        }).collect();

                        let outputs = item.outputs.unwrap_or_default().into_iter().map(|output| {
                            Parameter {
                                name: output.name,
                                r#type: output.r#type,
                                components: output.components.map(|comps| {
                                    comps.into_iter().map(|c| Parameter {
                                        name: c.name,
                                        r#type: c.r#type,
                                        components: None,
                                    }).collect()
                                }),
                            }
                        }).collect();

                        functions.insert(name.clone(), Function {
                            name,
                            inputs,
                            outputs,
                            state_mutability,
                        });
                    }
                },
                "event" => {
                    if let Some(name) = item.name {
                        let inputs = item.inputs.unwrap_or_default().into_iter().map(|input| {
                            EventParameter {
                                name: input.name,
                                r#type: input.r#type,
                                indexed: input.indexed.unwrap_or(false),
                                components: input.components.map(|comps| {
                                    comps.into_iter().map(|c| Parameter {
                                        name: c.name,
                                        r#type: c.r#type,
                                        components: None,
                                    }).collect()
                                }),
                            }
                        }).collect();

                        events.insert(name.clone(), Event {
                            name,
                            inputs,
                            anonymous: item.anonymous.unwrap_or(false),
                        });
                    }
                },
                _ => {}, // Ignore other ABI item types
            }
        }

        Ok(Contract {
            address,
            abi,
            functions,
            events,
        })
    }

    /// Returns the contract address
    #[wasm_bindgen(getter)]
    pub fn address(&self) -> String {
        self.address.clone()
    }

    /// Returns the contract ABI as a string
    #[wasm_bindgen(getter)]
    pub fn abi(&self) -> String {
        self.abi.clone()
    }

    /// Returns a list of function names in the contract
    #[wasm_bindgen]
    pub fn get_function_names(&self) -> Result<JsValue, JsValue> {
        let names: Vec<String> = self.functions.keys().cloned().collect();
        Ok(serde_wasm_bindgen::to_value(&names)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?)
    }

    /// Returns a list of event names in the contract
    #[wasm_bindgen]
    pub fn get_event_names(&self) -> Result<JsValue, JsValue> {
        let names: Vec<String> = self.events.keys().cloned().collect();
        Ok(serde_wasm_bindgen::to_value(&names)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?)
    }

    /// Encodes a function call for the given function name and arguments
    #[wasm_bindgen]
    pub fn encode_function_call(&self, function_name: &str, args: JsValue) -> Result<String, JsValue> {
        // Check if function exists
        let function = match self.functions.get(function_name) {
            Some(f) => f,
            None => return Err(JsValue::from_str(&format!("Function '{}' not found in ABI", function_name))),
        };

        // Parse arguments
        let args_vec: Vec<JsValue> = match js_sys::Array::from(&args).to_vec() {
            args if args.len() != function.inputs.len() => {
                return Err(JsValue::from_str(&format!(
                    "Expected {} arguments for function '{}', got {}",
                    function.inputs.len(), function_name, args.len()
                )));
            },
            args => args,
        };

        // For now, we're using a simplified encoding approach
        // In a real implementation, we would use proper ABI encoding
        let selector = compute_function_selector(function_name, &function.inputs);
        
        // Encode arguments (simplified for demo)
        let mut encoded_args = String::new();
        for (i, arg) in args_vec.iter().enumerate() {
            if let Some(arg_str) = arg.as_string() {
                encoded_args.push_str(&format!("_{}", arg_str.replace(" ", "")));
            } else {
                // Handle non-string arguments
                encoded_args.push_str(&format!("_{}", i));
            }
        }

        Ok(format!("{}{}", selector, encoded_args))
    }

    /// Calls a read-only (view/pure) function on the contract
    #[wasm_bindgen]
    pub async fn call(&self, function_name: &str, args: JsValue) -> Result<JsValue, JsValue> {
        // Check if function exists and is read-only
        let function = match self.functions.get(function_name) {
            Some(f) => {
                if f.state_mutability != StateMutability::View && f.state_mutability != StateMutability::Pure {
                    return Err(JsValue::from_str(
                        &format!("Function '{}' is not read-only (view/pure)", function_name)
                    ));
                }
                f
            },
            None => return Err(JsValue::from_str(&format!("Function '{}' not found in ABI", function_name))),
        };

        // Encode the function call
        let encoded_call = self.encode_function_call(function_name, args)?;

        // Perform the call using Web3
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().ok_or_else(|| JsValue::from_str("Could not access window"))?;
            
            if !js_sys::Reflect::has(&window, &JsValue::from_str("ethereum")).unwrap_or(false) {
                return Err(JsValue::from_str("Ethereum provider not found"));
            }

            let ethereum = js_sys::Reflect::get(&window, &JsValue::from_str("ethereum"))?;
            
            let request_fn = js_sys::Reflect::get(&ethereum, &JsValue::from_str("request"))?;
            let request_fn = js_sys::Function::from(request_fn);
            
            let params = js_sys::Object::new();
            js_sys::Reflect::set(&params, &JsValue::from_str("to"), &JsValue::from_str(&self.address))?;
            js_sys::Reflect::set(&params, &JsValue::from_str("data"), &JsValue::from_str(&encoded_call))?;
            
            let args = js_sys::Object::new();
            js_sys::Reflect::set(&args, &JsValue::from_str("method"), &JsValue::from_str("eth_call"))?;
            
            let params_array = js_sys::Array::new();
            params_array.push(&params);
            params_array.push(&JsValue::from_str("latest"));
            
            js_sys::Reflect::set(&args, &JsValue::from_str("params"), &params_array)?;
            
            let promise = request_fn.call1(&ethereum, &args)?;
            let promise = js_sys::Promise::from(promise);
            let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
            
            // Parse the result based on the function's output types
            return decode_function_result(function, result);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Mock response for testing
            let mock_result = JsValue::from_str("0x0000000000000000000000000000000000000000000000000000000000000020");
            return decode_function_result(function, mock_result);
        }
    }

    /// Sends a transaction to execute a state-changing (nonpayable/payable) function on the contract
    #[wasm_bindgen]
    pub async fn send_transaction(&self, function_name: &str, args: JsValue, options: JsValue) -> Result<String, JsValue> {
        // Check if function exists and can modify state
        let _function = match self.functions.get(function_name) {
            Some(f) => {
                if f.state_mutability == StateMutability::View || f.state_mutability == StateMutability::Pure {
                    return Err(JsValue::from_str(
                        &format!("Function '{}' is read-only and cannot be called with sendTransaction", function_name)
                    ));
                }
                f
            },
            None => return Err(JsValue::from_str(&format!("Function '{}' not found in ABI", function_name))),
        };

        // Encode the function call
        let encoded_call = self.encode_function_call(function_name, args)?;

        // Prepare transaction options
        let tx_options = js_sys::Object::from(options);
        js_sys::Reflect::set(&tx_options, &JsValue::from_str("to"), &JsValue::from_str(&self.address))?;
        js_sys::Reflect::set(&tx_options, &JsValue::from_str("data"), &JsValue::from_str(&encoded_call))?;

        // Send the transaction
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().ok_or_else(|| JsValue::from_str("Could not access window"))?;
            
            if !js_sys::Reflect::has(&window, &JsValue::from_str("ethereum")).unwrap_or(false) {
                return Err(JsValue::from_str("Ethereum provider not found"));
            }

            let ethereum = js_sys::Reflect::get(&window, &JsValue::from_str("ethereum"))?;
            
            let request_fn = js_sys::Reflect::get(&ethereum, &JsValue::from_str("request"))?;
            let request_fn = js_sys::Function::from(request_fn);
            
            let args = js_sys::Object::new();
            js_sys::Reflect::set(&args, &JsValue::from_str("method"), &JsValue::from_str("eth_sendTransaction"))?;
            
            let params_array = js_sys::Array::new();
            params_array.push(&tx_options);
            
            js_sys::Reflect::set(&args, &JsValue::from_str("params"), &params_array)?;
            
            let promise = request_fn.call1(&ethereum, &args)?;
            let promise = js_sys::Promise::from(promise);
            let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
            
            // Return the transaction hash
            if let Some(tx_hash) = result.as_string() {
                Ok(tx_hash)
            } else {
                Err(JsValue::from_str("Failed to get transaction hash"))
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Mock transaction hash for testing
            Ok("0x".to_string() + &"1234567890abcdef".repeat(4))
        }
    }

    /// Creates a new event subscription for the specified event
    #[wasm_bindgen]
    pub fn create_event_filter(&self, event_name: &str, indexed_params: JsValue) -> Result<ContractEventFilter, JsValue> {
        // Check if event exists
        let event = match self.events.get(event_name) {
            Some(e) => e,
            None => return Err(JsValue::from_str(&format!("Event '{}' not found in ABI", event_name))),
        };

        // Create the event signature
        let event_signature = compute_event_signature(event_name, &event.inputs);
        
        // Create a new event filter
        let mut filter = ContractEventFilter::new(event_signature, self.address.clone());
        
        // Handle indexed parameters if provided
        if !indexed_params.is_null() && !indexed_params.is_undefined() {
            let params_obj = js_sys::Object::from(indexed_params);
            
            // Add topics for indexed parameters
            for (_i, param) in event.inputs.iter().enumerate() {
                if param.indexed {
                    let param_name = &param.name;
                    if js_sys::Reflect::has(&params_obj, &JsValue::from_str(param_name)).unwrap_or(false) {
                        let value = js_sys::Reflect::get(&params_obj, &JsValue::from_str(param_name))?;
                        if let Some(value_str) = value.as_string() {
                            filter.add_topic(value_str)?;
                        }
                    }
                }
            }
        }
        
        Ok(filter)
    }
}

// Helper functions for Contract implementation

/// Computes a function selector from the function name and input parameters
fn compute_function_selector(name: &str, inputs: &[Parameter]) -> String {
    // In a real implementation, we would compute the Keccak256 hash of the function signature
    // and take the first 4 bytes. For this demo, we'll use a simplified approach.
    let mut signature = name.to_string();
    signature.push('(');
    
    for (i, input) in inputs.iter().enumerate() {
        if i > 0 {
            signature.push(',');
        }
        signature.push_str(&input.r#type);
    }
    
    signature.push(')');
    
    // Use a simple hash function for demo purposes
    let hash = signature.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32));
    format!("0x{:08x}", hash)
}

/// Computes an event signature (topic0) from the event name and input parameters
fn compute_event_signature(name: &str, inputs: &[EventParameter]) -> String {
    // In a real implementation, we would compute the Keccak256 hash of the event signature
    // For this demo, we'll use a simplified approach.
    let mut signature = name.to_string();
    signature.push('(');
    
    for (i, input) in inputs.iter().enumerate() {
        if i > 0 {
            signature.push(',');
        }
        signature.push_str(&input.r#type);
    }
    
    signature.push(')');
    
    // Use a simple hash function for demo purposes
    let hash = signature.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32));
    format!("0x{:064x}", hash)
}

/// Decodes a function's result based on its output types
fn decode_function_result(function: &Function, result: JsValue) -> Result<JsValue, JsValue> {
    // In a real implementation, we would use proper ABI decoding
    // For this demo, we'll return the raw result
    
    // If the function has no outputs, return null
    if function.outputs.is_empty() {
        return Ok(JsValue::null());
    }
    
    // For functions with a single output, return the decoded value
    if function.outputs.len() == 1 {
        if let Some(result_str) = result.as_string() {
            // For numeric types, try to convert from hex
            if function.outputs[0].r#type.starts_with("uint") || 
               function.outputs[0].r#type.starts_with("int") {
                if let Ok(decimal) = hex_to_decimal(&result_str) {
                    return Ok(JsValue::from_str(&decimal));
                }
            }
            // For other types, return the raw value
            return Ok(result);
        }
    }
    
    // For functions with multiple outputs, return a JS object
    // This would require more sophisticated decoding in a real implementation
    Ok(result)
} 