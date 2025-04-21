use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// Struct for ABI definition
#[derive(Serialize, Deserialize)]
pub struct AbiItem {
    pub name: Option<String>,
    pub r#type: String,
    pub inputs: Option<Vec<AbiInput>>,
    pub outputs: Option<Vec<AbiOutput>>,
    pub stateMutability: Option<String>,
    pub constant: Option<bool>,
    pub payable: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct AbiInput {
    pub name: String,
    pub r#type: String,
    pub indexed: Option<bool>,
    pub components: Option<Vec<AbiComponent>>,
}

#[derive(Serialize, Deserialize)]
pub struct AbiOutput {
    pub name: String,
    pub r#type: String,
    pub components: Option<Vec<AbiComponent>>,
}

#[derive(Serialize, Deserialize)]
pub struct AbiComponent {
    pub name: String,
    pub r#type: String,
    pub components: Option<Vec<AbiComponent>>,
}

// Basic structure for smart contract interaction
#[wasm_bindgen]
pub struct SmartContract {
    abi: String,
    address: String,
}

#[wasm_bindgen]
impl SmartContract {
    #[wasm_bindgen(constructor)]
    pub fn new(abi: String, address: String) -> SmartContract {
        SmartContract { abi, address }
    }

    // Get contract address
    #[wasm_bindgen(getter)]
    pub fn address(&self) -> String {
        self.address.clone()
    }

    // Get contract ABI
    #[wasm_bindgen(getter)]
    pub fn abi(&self) -> String {
        self.abi.clone()
    }

    // Create ABI encoding for function call (simple simulation)
    pub fn encode_function_call(&self, function_name: &str, args: &JsValue) -> Result<String, JsValue> {
        // In a real implementation, we would parse the ABI, use the function signature and arguments
        // to create an ABI encoding. Here we're just doing a simple simulation.
        
        let args_str = args.as_string().unwrap_or_else(|| "[]".to_string());
        
        // Function selector (first 4 bytes) simulation
        let mut function_selector = format!("0x{:08x}", function_name.as_bytes().iter().fold(0u32, |acc, b| acc.wrapping_add(*b as u32)));
        
        // Represent arguments simply (not an actual encoding)
        if args_str != "[]" {
            function_selector.push_str(&format!("{}", args_str.replace([' ', '"', '[', ']'], "")));
        }
        
        Ok(function_selector)
    }
}

/// Event filter parameters
#[wasm_bindgen]
pub struct EventOptions {
    address: Option<String>,
    from_block: Option<u64>,
    to_block: Option<u64>,
    topics: Vec<String>,
}

#[wasm_bindgen]
impl EventOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> EventOptions {
        EventOptions {
            address: None,
            from_block: None,
            to_block: None,
            topics: Vec::new(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn address(&self) -> Option<String> {
        self.address.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_address(&mut self, address: String) {
        self.address = Some(address);
    }

    #[wasm_bindgen(getter)]
    pub fn from_block(&self) -> Option<u64> {
        self.from_block
    }

    #[wasm_bindgen(setter)]
    pub fn set_from_block(&mut self, from_block: u64) {
        self.from_block = Some(from_block);
    }

    #[wasm_bindgen(getter)]
    pub fn to_block(&self) -> Option<u64> {
        self.to_block
    }

    #[wasm_bindgen(setter)]
    pub fn set_to_block(&mut self, to_block: u64) {
        self.to_block = Some(to_block);
    }

    #[wasm_bindgen]
    pub fn add_topic(&mut self, topic: String) {
        self.topics.push(topic);
    }

    #[wasm_bindgen]
    pub fn get_topic(&self, index: usize) -> Option<String> {
        self.topics.get(index).cloned()
    }

    #[wasm_bindgen]
    pub fn clear_topics(&mut self) {
        self.topics.clear();
    }
}
