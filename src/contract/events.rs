use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

/// Event data returned from Ethereum logs
#[derive(Serialize, Deserialize, Clone)]
pub struct EventData {
    pub event_name: String,
    pub args: Vec<LogParam>,
    pub raw_log: EventLog,
}

/// Log parameter with name and value
#[derive(Serialize, Deserialize, Clone)]
pub struct LogParam {
    pub name: String,
    pub value: String,
    pub r#type: ParamType,
}

/// Ethereum event log structure
#[derive(Serialize, Deserialize, Clone)]
pub struct EventLog {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub block_number: Option<u64>,
    pub transaction_hash: Option<String>,
    pub transaction_index: Option<u64>,
    pub block_hash: Option<String>,
    pub log_index: Option<u64>,
    pub removed: Option<bool>,
}

/// Solidity parameter types
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum ParamType {
    Address,
    Bytes,
    Int(usize),
    Uint(usize),
    Bool,
    String,
    Array(Box<ParamType>),
    FixedBytes(usize),
    FixedArray(Box<ParamType>, usize),
    Tuple(Vec<ParamType>),
}

impl ParamType {
    /// Convert a string type to a ParamType
    pub fn from_str(type_str: &str) -> Option<Self> {
        if type_str == "address" {
            Some(ParamType::Address)
        } else if type_str == "bytes" {
            Some(ParamType::Bytes)
        } else if type_str == "bool" {
            Some(ParamType::Bool)
        } else if type_str == "string" {
            Some(ParamType::String)
        } else if type_str.starts_with("uint") {
            let size_str = &type_str[4..];
            if let Ok(size) = size_str.parse::<usize>() {
                if size % 8 == 0 && size <= 256 {
                    return Some(ParamType::Uint(size));
                }
            }
            None
        } else if type_str.starts_with("int") {
            let size_str = &type_str[3..];
            if let Ok(size) = size_str.parse::<usize>() {
                if size % 8 == 0 && size <= 256 {
                    return Some(ParamType::Int(size));
                }
            }
            None
        } else if type_str.starts_with("bytes") {
            let size_str = &type_str[5..];
            if let Ok(size) = size_str.parse::<usize>() {
                if size >= 1 && size <= 32 {
                    return Some(ParamType::FixedBytes(size));
                }
            }
            None
        } else {
            None
        }
    }
}

/// Represents a filter for Ethereum events
#[wasm_bindgen]
pub struct ContractEventFilter {
    event_signature: String,
    contract_address: String,
    topics: Vec<String>,
}

#[wasm_bindgen]
impl ContractEventFilter {
    /// Creates a new event filter for the given event signature and contract address
    #[wasm_bindgen(constructor)]
    pub fn new(event_signature: String, contract_address: String) -> ContractEventFilter {
        let mut topics = Vec::new();
        topics.push(event_signature.clone());
        
        ContractEventFilter {
            event_signature,
            contract_address,
            topics,
        }
    }
    
    /// Adds a topic (indexed parameter) to the filter
    #[wasm_bindgen]
    pub fn add_topic(&mut self, topic: String) -> Result<(), JsValue> {
        if self.topics.len() >= 4 {
            return Err(JsValue::from_str("Maximum 4 topics allowed"));
        }
        self.topics.push(topic);
        Ok(())
    }
    
    /// Converts the filter to a JS object that can be used with eth_getLogs or eth_subscribe
    #[wasm_bindgen]
    pub fn to_filter_object(&self) -> Result<JsValue, JsValue> {
        let filter = js_sys::Object::new();
        
        js_sys::Reflect::set(&filter, &JsValue::from_str("address"), &JsValue::from_str(&self.contract_address))?;
        
        let topics_array = js_sys::Array::new();
        for topic in &self.topics {
            topics_array.push(&JsValue::from_str(topic));
        }
        
        js_sys::Reflect::set(&filter, &JsValue::from_str("topics"), &topics_array)?;
        
        Ok(filter.into())
    }
    
    /// Subscribes to events matching this filter
    #[wasm_bindgen]
    pub async fn subscribe(&self, callback: &js_sys::Function) -> Result<JsValue, JsValue> {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().ok_or_else(|| JsValue::from_str("Could not access window"))?;
            
            if !js_sys::Reflect::has(&window, &JsValue::from_str("ethereum")).unwrap_or(false) {
                return Err(JsValue::from_str("Ethereum provider not found"));
            }

            let ethereum = js_sys::Reflect::get(&window, &JsValue::from_str("ethereum"))?;
            
            // Not all providers support eth_subscribe, so we'll use eth_getLogs with a polling mechanism
            let filter_obj = self.to_filter_object()?;
            
            // Set up an interval to poll for logs
            let closure = js_sys::Function::new_with_args(
                "filter, ethereum, callback",
                r#"
                async function pollLogs() {
                    try {
                        const logs = await ethereum.request({
                            method: 'eth_getLogs',
                            params: [filter]
                        });
                        
                        if (logs && logs.length > 0) {
                            for (const log of logs) {
                                callback(null, log);
                            }
                        }
                    } catch (error) {
                        callback(error, null);
                    }
                }
                
                // Poll every 10 seconds
                const intervalId = setInterval(pollLogs, 10000);
                
                // Initial poll
                pollLogs();
                
                // Return the interval ID so it can be cleared later
                return intervalId;
                "#
            );
            
            let result = closure.call3(
                &JsValue::null(),
                &filter_obj,
                &ethereum,
                callback
            )?;
            
            Ok(result)
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Mock subscription ID for testing
            Ok(JsValue::from_str("0x1"))
        }
    }
    
    /// Unsubscribes from an event subscription
    #[wasm_bindgen]
    pub fn unsubscribe(&self, subscription_id: JsValue) -> Result<(), JsValue> {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().ok_or_else(|| JsValue::from_str("Could not access window"))?;
            
            // Clear the interval if it's valid
            if !subscription_id.is_null() && !subscription_id.is_undefined() {
                let clear_interval = js_sys::Reflect::get(&window, &JsValue::from_str("clearInterval"))?;
                let clear_interval_fn = js_sys::Function::from(clear_interval);
                clear_interval_fn.call1(&JsValue::null(), &subscription_id)?;
            }
        }
        
        Ok(())
    }
} 