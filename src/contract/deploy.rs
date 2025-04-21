use wasm_bindgen::prelude::*;
use super::Contract;
use crate::eth_integration::get_provider;
use js_sys::{Object, Reflect, Promise, Array};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;
use std::collections::HashMap;
use std::time::Duration;
use std::thread::sleep;

#[cfg(target_arch = "wasm32")]
use web_sys;

use crate::contract::abi::AbiItem;
use crate::utils;

/// Contract deployment module that handles deploying new smart contracts to the Ethereum network.
#[wasm_bindgen]
pub struct ContractDeployer {
    bytecode: String,
    abi: String,
    eth_provider: JsValue,
    constructor_args: Vec<JsValue>,
}

#[wasm_bindgen]
impl ContractDeployer {
    /// Creates a new ContractDeployer with the provided bytecode and ABI.
    #[wasm_bindgen(constructor)]
    pub fn new(bytecode: String, abi: String) -> Result<ContractDeployer, JsValue> {
        // Get window object
        #[cfg(target_arch = "wasm32")]
        let eth_provider = {
            let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window object found"))?;
            
            // Check for ethereum provider
            if !js_sys::Reflect::has(&window, &JsValue::from_str("ethereum")).unwrap_or(false) {
                return Err(JsValue::from_str("Ethereum provider not found in window object"));
            }
            
            js_sys::Reflect::get(&window, &JsValue::from_str("ethereum"))?
        };
        
        #[cfg(not(target_arch = "wasm32"))]
        let eth_provider = JsValue::null();
        
        Ok(ContractDeployer {
            bytecode,
            abi,
            eth_provider,
            constructor_args: Vec::new(),
        })
    }

    /// Set constructor arguments for the contract deployment
    #[wasm_bindgen]
    pub fn set_constructor_args(&mut self, args: js_sys::Array) -> Result<(), JsValue> {
        self.constructor_args = args.to_vec();
        Ok(())
    }

    /// Encode constructor arguments with the contract bytecode
    fn encode_constructor_data(&self) -> Result<String, JsValue> {
        // Parse ABI to find constructor
        let abi_items: Vec<AbiItem> = match serde_json::from_str(&self.abi) {
            Ok(items) => items,
            Err(e) => return Err(JsValue::from_str(&format!("Failed to parse ABI: {}", e))),
        };
        
        // Find constructor in ABI
        let constructor = abi_items.iter().find(|item| item.r#type == "constructor");
        
        // If constructor has inputs, encode them
        // For simplicity, we're just appending args as strings
        // A real implementation would use proper ABI encoding
        let mut encoded_data = self.bytecode.clone();
        
        if let Some(constructor) = constructor {
            if let Some(inputs) = &constructor.inputs {
                if inputs.len() != self.constructor_args.len() {
                    return Err(JsValue::from_str(&format!(
                        "Expected {} constructor arguments, got {}",
                        inputs.len(), self.constructor_args.len()
                    )));
                }
                
                // Simple encoding for demo purposes
                for arg in &self.constructor_args {
                    if let Some(arg_str) = arg.as_string() {
                        // For addresses and bytes, remove 0x prefix if present
                        let processed_arg = if arg_str.starts_with("0x") {
                            arg_str[2..].to_string()
                        } else {
                            arg_str
                        };
                        
                        // Encode as hex and append
                        encoded_data.push_str(&processed_arg);
                    } else if let Some(arg_num) = arg.as_f64() {
                        // Convert numbers to hex
                        encoded_data.push_str(&format!("{:064x}", arg_num as u64));
                    } else {
                        return Err(JsValue::from_str("Unsupported argument type"));
                    }
                }
            }
        }
        
        // Ensure bytecode has 0x prefix
        if !encoded_data.starts_with("0x") {
            encoded_data = format!("0x{}", encoded_data);
        }
        
        Ok(encoded_data)
    }

    /// Estimates the gas required to deploy the contract with the given constructor arguments.
    #[wasm_bindgen]
    pub async fn estimate_gas(&self, from_address: String) -> Result<JsValue, JsValue> {
        let encoded_data = self.encode_constructor_data()?;
        
        // Create transaction object
        let tx_obj = Object::new();
        Reflect::set(&tx_obj, &JsValue::from_str("from"), &JsValue::from_str(&from_address))?;
        Reflect::set(&tx_obj, &JsValue::from_str("data"), &JsValue::from_str(&encoded_data))?;
        
        // Call estimateGas method on provider
        let request_obj = Object::new();
        Reflect::set(&request_obj, &JsValue::from_str("method"), &JsValue::from_str("eth_estimateGas"))?;
        
        let params = js_sys::Array::new();
        params.push(&tx_obj);
        Reflect::set(&request_obj, &JsValue::from_str("params"), &params)?;
        
        let request_fn = Reflect::get(&self.eth_provider, &JsValue::from_str("request"))?;
        let request_fn = js_sys::Function::from(request_fn);
        
        let promise = request_fn.call1(&self.eth_provider, &request_obj)?;
        let promise = Promise::from(promise);
        let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
        
        // Convert hex to decimal
        if let Some(gas_hex) = result.as_string() {
            // Remove 0x prefix if present
            let gas_hex = if gas_hex.starts_with("0x") { &gas_hex[2..] } else { &gas_hex };
            
            // Parse hex to decimal
            match u64::from_str_radix(gas_hex, 16) {
                Ok(gas) => Ok(JsValue::from_f64(gas as f64)),
                Err(_) => Err(JsValue::from_str("Failed to parse gas estimate")),
            }
        } else {
            Err(JsValue::from_str("Invalid gas estimate result"))
        }
    }

    /// Deploys the contract with the given constructor arguments and transaction options.
    #[wasm_bindgen]
    pub async fn deploy(&self, from_address: String, gas_limit: Option<u64>, value: Option<String>) -> Result<JsValue, JsValue> {
        let encoded_data = self.encode_constructor_data()?;
        
        // Create transaction object
        let tx_obj = Object::new();
        Reflect::set(&tx_obj, &JsValue::from_str("from"), &JsValue::from_str(&from_address))?;
        Reflect::set(&tx_obj, &JsValue::from_str("data"), &JsValue::from_str(&encoded_data))?;
        
        // Add gas limit if provided
        if let Some(gas) = gas_limit {
            Reflect::set(&tx_obj, &JsValue::from_str("gas"), &JsValue::from_f64(gas as f64))?;
        }
        
        // Add value if provided
        if let Some(val) = value {
            Reflect::set(&tx_obj, &JsValue::from_str("value"), &JsValue::from_str(&val))?;
        }
        
        // Send transaction
        let request_obj = Object::new();
        Reflect::set(&request_obj, &JsValue::from_str("method"), &JsValue::from_str("eth_sendTransaction"))?;
        
        let params = js_sys::Array::new();
        params.push(&tx_obj);
        Reflect::set(&request_obj, &JsValue::from_str("params"), &params)?;
        
        let request_fn = Reflect::get(&self.eth_provider, &JsValue::from_str("request"))?;
        let request_fn = js_sys::Function::from(request_fn);
        
        let promise = request_fn.call1(&self.eth_provider, &request_obj)?;
        let promise = Promise::from(promise);
        let tx_hash = wasm_bindgen_futures::JsFuture::from(promise).await?;
        
        // Wait for transaction receipt
        let receipt = self.wait_for_receipt(tx_hash.clone()).await?;
        
        // Create result object
        let result = Object::new();
        Reflect::set(&result, &JsValue::from_str("transactionHash"), &tx_hash)?;
        Reflect::set(&result, &JsValue::from_str("receipt"), &receipt)?;
        
        // Create contract instance
        let contract_address = Reflect::get(&receipt, &JsValue::from_str("contractAddress"))?;
        let contract = Contract::new(
            contract_address.as_string().ok_or_else(|| JsValue::from_str("Invalid contract address"))?,
            self.abi.clone()
        )?;
        
        // Convert contract to JsValue before setting it
        Reflect::set(&result, &JsValue::from_str("contract"), &JsValue::from(contract))?;
        
        // Convert result to JsValue before returning
        Ok(JsValue::from(result))
    }

    /// Wait for transaction receipt
    async fn wait_for_receipt(&self, tx_hash: JsValue) -> Result<JsValue, JsValue> {
        // Function to get transaction receipt
        async fn get_receipt(provider: &JsValue, tx_hash: &JsValue) -> Result<JsValue, JsValue> {
            let request_obj = Object::new();
            Reflect::set(&request_obj, &JsValue::from_str("method"), &JsValue::from_str("eth_getTransactionReceipt"))?;
            
            let params = js_sys::Array::new();
            params.push(tx_hash);
            Reflect::set(&request_obj, &JsValue::from_str("params"), &params)?;
            
            let request_fn = Reflect::get(provider, &JsValue::from_str("request"))?;
            let request_fn = js_sys::Function::from(request_fn);
            
            let promise = request_fn.call1(provider, &request_obj)?;
            let promise = Promise::from(promise);
            wasm_bindgen_futures::JsFuture::from(promise).await
        }
        
        // Poll for receipt with exponential backoff
        let mut attempts = 0;
        let max_attempts = 50;
        let mut delay_ms = 1000;
        
        while attempts < max_attempts {
            let receipt = get_receipt(&self.eth_provider, &tx_hash).await?;
            
            if !receipt.is_null() && !receipt.is_undefined() {
                return Ok(receipt);
            }
            
            // Wait with exponential backoff
            #[cfg(target_arch = "wasm32")]
            {
                let promise = Promise::new(&mut |resolve, _| {
                    let window = web_sys::window().unwrap();
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        &resolve, 
                        delay_ms
                    );
                });
                
                wasm_bindgen_futures::JsFuture::from(promise).await?;
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            {
                // Simulated delay in non-wasm environment
                sleep(Duration::from_millis(delay_ms));
            }
            
            attempts += 1;
            delay_ms = std::cmp::min(delay_ms * 2, 10000);
        }
        
        Err(JsValue::from_str("Transaction receipt not found after maximum attempts"))
    }

    /// Create a collection from an existing contract
    #[wasm_bindgen]
    pub fn create_collection(&self, contracts: js_sys::Array) -> Result<JsValue, JsValue> {
        let result = js_sys::Object::new();
        
        // Process each contract
        for i in 0..contracts.length() {
            let contract_data = contracts.get(i);
            
            // Clone before dyn_into to avoid moved value errors
            let contract_obj = contract_data.clone().dyn_into::<js_sys::Object>()?;
            
            let name = Reflect::get(&contract_obj, &JsValue::from_str("name"))?;
            let address = Reflect::get(&contract_obj, &JsValue::from_str("address"))?;
            
            // Convert to strings
            let name_str = name.as_string().ok_or_else(|| JsValue::from_str("Invalid contract name"))?;
            let addr_str = address.as_string().ok_or_else(|| JsValue::from_str("Invalid contract address"))?;
            
            // Create Contract instance
            let contract = Contract::new(addr_str, self.abi.clone())?;
            
            // Add to result object instead of HashMap (which requires Serialize)
            Reflect::set(&result, &name, &JsValue::from(contract))?;
        }
        
        Ok(JsValue::from(result))
    }

    /// Deploys an ERC-20 token contract with standard parameters.
    #[wasm_bindgen]
    pub async fn deploy_erc20(
        name: String, 
        symbol: String, 
        total_supply: String, 
        decimals: u8, 
        options: JsValue
    ) -> Result<JsValue, JsValue> {
        // ERC-20 contract bytecode (this is a placeholder - in a real implementation 
        // this would be the full compiled bytecode of a standard ERC-20 contract)
        let bytecode = "0x608060405234801561001057600080fd5b50610b0a806100206000396000f3fe608060405234801561001057600080fd5b50600436106100885760003560e01c806370a082311161005b57806370a0823114610149578063a457c2d71461019f578063a9059cbb146101ff578063dd62ed3e1461025f57610088565b8063095ea7b31461008d57806318160ddd146100ed57806323b872dd14610111578063395093511461018157600080fd5b3661008957005b005b6100d06100a736600461086f565b6001600160a01b03918216600090815260016020908152604080832093909416825291909152205490565b60405163ffffffff909116815260200160405180910390f35b6100ff60005481565b60405190815260200160405180910390f35b61017461011f36600461089b565b6001600160a01b038316600090815260208190526040902054821115610144575060006101e1565b506001600160a01b0383166000908152602081905260409020805483900390555b90565b61018f6101573660046108df565b6001600160a01b03166000908152602081905260409020549056b6100d06101ad36600461086f565b6001600160a01b0391821660009081526001602090815260408083209390941682529190915220549056b6001600160a01b0382166000908152602081905260409020548111156102325750600061027a565b506001600160a01b038216600090815260208190526040902080548301905561027a565b6100d061026d36600461086f565b6001600160a01b0391821660009081526001602090815260408083209390941682529190915220549056b6001600160a01b038216600090815260208190526040902054811115610364576001600160a01b03831660009081526020819052604090205482036103645750600061036e565b505060015b919050565b6000806000610383888a018a6108df565b909250905061039281836108fa565b9150509250929050565b600080600080600080600060e0888a0312156103b757600080fd5b87516103c2816109ee565b6020890151909750906103d4816109ee565b60408901519096506103e5816109ee565b979a969950949793969295929490936060810135925060808101359160a0820135916103748a01359061040f81610a03565b8091505092959891949750929550565b805163ffffffff81168114610a0357600080fd5b60006020828403121561044457600080fd5b81516104bf816109ee565b9392505050565b60008083601f84011261049b57600080fd5b50813567ffffffffffffffff8111156104b357600080fd5b6020830191508360208260051b85010111156104ce57600080fd5b9250929050565b60008060006040848603121561048a57600080fd5b83359250602084013567ffffffffffffffff8111156104fa57600080fd5b61050886828701610489565b949790965093945050565b600080604083850312156104fa57600080fd5b803567ffffffffffffffff81111561053c57600080fd5b61054a84828501610489565b9598949750955050565b60008060008060006080868803121561056e57600080fd5b85359450602086013567ffffffffffffffff81111561057d57600080fd5b61058b88828901610489565b90955093505060408601359150606086013567ffffffffffffffff8111156105b257600080fd5b6105c088828901610489565b9150509295509295909350565b600080600080600080600060c0888a0312156105e757600080fd5b87359650602088013595506040880135945060608801359350608088013567ffffffffffffffff81111561061a57600080fd5b61062a8a828b01610489565b989b9699509397509195939450505060a00135919050565b60008060008060006080868803121561065a57600080fd5b853567ffffffffffffffff81111561067157600080fd5b61067f88828901610489565b9096509450506020860135935060408601359150606086013567ffffffffffffffff8111156105b257600080fd5b600080602083850312156106be57600080fd5b823567ffffffffffffffff8111156106d557600080fd5b6106e385828601610489565b90969095509350505050565b600080600080600060a0868803121561070757600080fd5b85359450602086013593506040860135925060608601359150608086013567ffffffffffffffff8111156105b257600080fd5b6000806000806060858703121561074e57600080fd5b84359350602085013567ffffffffffffffff8111156106d557600080fd5b600080600060a0848603121561078257600080fd5b833567ffffffffffffffff81111561079957600080fd5b6107a786828701610489565b9450945050602084013592506040840135915060608401356107c8816109ee565b91505092959194509250565b6000806000604084860312156107e957600080fd5b83359250602084013567ffffffffffffffff8111156104fa57600080fd5b60008060006040848603121561081c57600080fd5b83359250602084013567ffffffffffffffff81111561053c57600080fd5b60008060006060848603121561084f57600080fd5b83359250602084013591506040840135610867816109ee565b809150509250925092565b6000806040838503121561088257600080fd5b823561088d816109ee565b9150602083013561089d816109ee565b809150509250929050565b6000806000606084860312156108b057600080fd5b83356108bb816109ee565b925060208401356108cb816109ee565b929592945050506040919091013590565b6000602082840312156108f157600080fd5b81356104bf816109ee565b60008083357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe184360301811261092f57600080fd5b83018035915067ffffffffffffffff82111561094a57600080fd5b60200191503681900382131561096057600080fd5b9250929050565b6000815180845260005b8181101561098d57602081850181015186830182015201610971565b8181111561099f576000602083870101525b50601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b6001600160a01b03811681146109ee57600080fd5b50565b8015158114610a0357600080fd5b5056fea26469706673582212206028cce15c3a8283aface19c2dd25d9c1dbb61ebb7f53f17fd7eee6f89eaebde64736f6c63430008090033";
        
        // ERC-20 contract ABI
        let abi = r#"[
            {
                "inputs": [
                    {"name": "name", "type": "string"},
                    {"name": "symbol", "type": "string"},
                    {"name": "initialSupply", "type": "uint256"},
                    {"name": "decimals", "type": "uint8"}
                ],
                "stateMutability": "nonpayable",
                "type": "constructor"
            },
            {
                "anonymous": false,
                "inputs": [
                    {"indexed": true, "name": "owner", "type": "address"},
                    {"indexed": true, "name": "spender", "type": "address"},
                    {"indexed": false, "name": "value", "type": "uint256"}
                ],
                "name": "Approval",
                "type": "event"
            },
            {
                "anonymous": false,
                "inputs": [
                    {"indexed": true, "name": "from", "type": "address"},
                    {"indexed": true, "name": "to", "type": "address"},
                    {"indexed": false, "name": "value", "type": "uint256"}
                ],
                "name": "Transfer",
                "type": "event"
            },
            {
                "inputs": [],
                "name": "name",
                "outputs": [{"name": "", "type": "string"}],
                "stateMutability": "view",
                "type": "function"
            }
        ]"#;
        
        // Create constructor arguments
        let args = js_sys::Array::new();
        args.push(&JsValue::from_str(&name));
        args.push(&JsValue::from_str(&symbol));
        args.push(&JsValue::from_str(&total_supply));
        args.push(&JsValue::from_str(&decimals.to_string()));
        
        // Deploy the contract
        let deployer = ContractDeployer::new(bytecode.to_string(), abi.to_string())?;
        let contract = deployer.deploy(
            options.as_string().ok_or_else(|| JsValue::from_str("From address required"))?,
            None,
            None
        ).await?;
        
        // Convert to JsValue to return
        let result = Object::new();
        
        // Get contract address from the contract object using Reflect
        let contract_obj = Reflect::get(&contract, &JsValue::from_str("contract"))?;
        let contract_addr = if let Some(contract_val) = contract_obj.dyn_ref::<JsValue>() {
            // Handle contract as JsValue by getting its address property through Reflect
            let contract_obj = contract_val.dyn_ref::<Object>()
                .ok_or_else(|| JsValue::from_str("Contract is not an object"))?;
            Reflect::get(contract_obj, &JsValue::from_str("address"))?
        } else {
            // Fallback - this is just to handle potential edge cases
            JsValue::from_str("unknown")
        };
        
        Reflect::set(&result, &JsValue::from_str("address"), &contract_addr)?;
        Reflect::set(&result, &JsValue::from_str("contract"), &contract_obj)?;
        
        Ok(JsValue::from(result))
    }
} 