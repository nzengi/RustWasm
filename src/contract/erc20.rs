use wasm_bindgen::prelude::*;
use super::Contract;

/// ERC-20 token standard implementation.
/// This is a specialized interface for interacting with ERC-20 token contracts.
#[wasm_bindgen]
pub struct ERC20Token {
    contract: Contract,
}

const ERC20_ABI: &str = r#"[
    {
        "constant": true,
        "inputs": [],
        "name": "name",
        "outputs": [{"name": "", "type": "string"}],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [],
        "name": "symbol",
        "outputs": [{"name": "", "type": "string"}],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [],
        "name": "decimals",
        "outputs": [{"name": "", "type": "uint8"}],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [],
        "name": "totalSupply",
        "outputs": [{"name": "", "type": "uint256"}],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [{"name": "owner", "type": "address"}],
        "name": "balanceOf",
        "outputs": [{"name": "", "type": "uint256"}],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": false,
        "inputs": [
            {"name": "to", "type": "address"},
            {"name": "value", "type": "uint256"}
        ],
        "name": "transfer",
        "outputs": [{"name": "", "type": "bool"}],
        "payable": false,
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [
            {"name": "owner", "type": "address"},
            {"name": "spender", "type": "address"}
        ],
        "name": "allowance",
        "outputs": [{"name": "", "type": "uint256"}],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": false,
        "inputs": [
            {"name": "spender", "type": "address"},
            {"name": "value", "type": "uint256"}
        ],
        "name": "approve",
        "outputs": [{"name": "", "type": "bool"}],
        "payable": false,
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "constant": false,
        "inputs": [
            {"name": "from", "type": "address"},
            {"name": "to", "type": "address"},
            {"name": "value", "type": "uint256"}
        ],
        "name": "transferFrom",
        "outputs": [{"name": "", "type": "bool"}],
        "payable": false,
        "stateMutability": "nonpayable",
        "type": "function"
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
        "anonymous": false,
        "inputs": [
            {"indexed": true, "name": "owner", "type": "address"},
            {"indexed": true, "name": "spender", "type": "address"},
            {"indexed": false, "name": "value", "type": "uint256"}
        ],
        "name": "Approval",
        "type": "event"
    }
]"#;

#[wasm_bindgen]
impl ERC20Token {
    /// Creates a new ERC20Token instance for the given token contract address.
    #[wasm_bindgen(constructor)]
    pub fn new(token_address: String) -> Result<ERC20Token, JsValue> {
        let contract = Contract::new(token_address, ERC20_ABI.to_string())?;
        Ok(ERC20Token { contract })
    }

    /// Gets the token contract address.
    #[wasm_bindgen(getter)]
    pub fn address(&self) -> String {
        self.contract.address()
    }

    /// Gets the token name.
    #[wasm_bindgen]
    pub async fn name(&self) -> Result<String, JsValue> {
        let result = self.contract.call("name", JsValue::from(js_sys::Array::new())).await?;
        
        match result.as_string() {
            Some(name) => Ok(name),
            None => Err(JsValue::from_str("Failed to parse token name"))
        }
    }

    /// Gets the token symbol.
    #[wasm_bindgen]
    pub async fn symbol(&self) -> Result<String, JsValue> {
        let result = self.contract.call("symbol", JsValue::from(js_sys::Array::new())).await?;
        
        match result.as_string() {
            Some(symbol) => Ok(symbol),
            None => Err(JsValue::from_str("Failed to parse token symbol"))
        }
    }

    /// Gets the token decimals.
    #[wasm_bindgen]
    pub async fn decimals(&self) -> Result<u8, JsValue> {
        let result = self.contract.call("decimals", JsValue::from(js_sys::Array::new())).await?;
        
        match result.as_string() {
            Some(decimals_str) => {
                // Convert hexadecimal string to decimal
                if let Ok(decimals) = u8::from_str_radix(decimals_str.trim_start_matches("0x"), 16) {
                    Ok(decimals)
                } else {
                    Err(JsValue::from_str("Failed to parse token decimals"))
                }
            },
            None => Err(JsValue::from_str("Failed to get token decimals"))
        }
    }

    /// Gets the total supply of the token.
    #[wasm_bindgen]
    pub async fn total_supply(&self) -> Result<String, JsValue> {
        let result = self.contract.call("totalSupply", JsValue::from(js_sys::Array::new())).await?;
        
        match result.as_string() {
            Some(supply) => Ok(supply),
            None => Err(JsValue::from_str("Failed to parse token supply"))
        }
    }

    /// Gets the balance of the given address.
    #[wasm_bindgen]
    pub async fn balance_of(&self, owner: &str) -> Result<String, JsValue> {
        let args = js_sys::Array::new();
        args.push(&JsValue::from_str(owner));
        
        let result = self.contract.call("balanceOf", args.into()).await?;
        
        match result.as_string() {
            Some(balance) => Ok(balance),
            None => Err(JsValue::from_str("Failed to parse token balance"))
        }
    }

    /// Gets the allowance for a spender from an owner.
    #[wasm_bindgen]
    pub async fn allowance(&self, owner: &str, spender: &str) -> Result<String, JsValue> {
        let args = js_sys::Array::new();
        args.push(&JsValue::from_str(owner));
        args.push(&JsValue::from_str(spender));
        
        let result = self.contract.call("allowance", args.into()).await?;
        
        match result.as_string() {
            Some(allowance) => Ok(allowance),
            None => Err(JsValue::from_str("Failed to parse token allowance"))
        }
    }

    /// Transfers tokens to the given address.
    #[wasm_bindgen]
    pub async fn transfer(&self, to: &str, amount: &str, options: JsValue) -> Result<String, JsValue> {
        let args = js_sys::Array::new();
        args.push(&JsValue::from_str(to));
        args.push(&JsValue::from_str(amount));
        
        self.contract.send_transaction("transfer", args.into(), options).await
    }

    /// Approves a spender to use tokens on behalf of the sender.
    #[wasm_bindgen]
    pub async fn approve(&self, spender: &str, amount: &str, options: JsValue) -> Result<String, JsValue> {
        let args = js_sys::Array::new();
        args.push(&JsValue::from_str(spender));
        args.push(&JsValue::from_str(amount));
        
        self.contract.send_transaction("approve", args.into(), options).await
    }

    /// Transfers tokens from one address to another, requires approval.
    #[wasm_bindgen]
    pub async fn transfer_from(&self, from: &str, to: &str, amount: &str, options: JsValue) -> Result<String, JsValue> {
        let args = js_sys::Array::new();
        args.push(&JsValue::from_str(from));
        args.push(&JsValue::from_str(to));
        args.push(&JsValue::from_str(amount));
        
        self.contract.send_transaction("transferFrom", args.into(), options).await
    }

    /// Creates a transfer event filter.
    #[wasm_bindgen]
    pub fn create_transfer_filter(&self, from: Option<String>, to: Option<String>) -> Result<JsValue, JsValue> {
        let indexed_params = js_sys::Object::new();
        
        if let Some(from_addr) = from {
            js_sys::Reflect::set(&indexed_params, &JsValue::from_str("from"), &JsValue::from_str(&from_addr))?;
        }
        
        if let Some(to_addr) = to {
            js_sys::Reflect::set(&indexed_params, &JsValue::from_str("to"), &JsValue::from_str(&to_addr))?;
        }
        
        let filter = self.contract.create_event_filter("Transfer", indexed_params.into())?;
        Ok(filter.into())
    }

    /// Creates an approval event filter.
    #[wasm_bindgen]
    pub fn create_approval_filter(&self, owner: Option<String>, spender: Option<String>) -> Result<JsValue, JsValue> {
        let indexed_params = js_sys::Object::new();
        
        if let Some(owner_addr) = owner {
            js_sys::Reflect::set(&indexed_params, &JsValue::from_str("owner"), &JsValue::from_str(&owner_addr))?;
        }
        
        if let Some(spender_addr) = spender {
            js_sys::Reflect::set(&indexed_params, &JsValue::from_str("spender"), &JsValue::from_str(&spender_addr))?;
        }
        
        let filter = self.contract.create_event_filter("Approval", indexed_params.into())?;
        Ok(filter.into())
    }

    /// Format a token amount with the correct number of decimal places.
    #[wasm_bindgen]
    pub async fn format_units(&self, amount: &str, decimals: Option<u8>) -> Result<String, JsValue> {
        let decimal_places = match decimals {
            Some(d) => d,
            None => self.decimals().await?,
        };
        
        // Convert from hex if needed
        let amount_str = if amount.starts_with("0x") {
            match u128::from_str_radix(amount.trim_start_matches("0x"), 16) {
                Ok(a) => a.to_string(),
                Err(_) => return Err(JsValue::from_str("Invalid amount format")),
            }
        } else {
            amount.to_string()
        };
        
        // Ensure the amount string has at least decimal_places + 1 characters
        let mut padded_amount = amount_str;
        while padded_amount.len() <= decimal_places as usize {
            padded_amount.insert(0, '0');
        }
        
        // Insert decimal point
        let len = padded_amount.len();
        let decimal_pos = len - decimal_places as usize;
        let formatted = format!(
            "{}.{}",
            &padded_amount[..decimal_pos],
            &padded_amount[decimal_pos..]
        );
        
        // Remove trailing zeros and decimal point if needed
        let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
        
        Ok(trimmed.to_string())
    }

    /// Parse a human-readable token amount to the raw amount.
    #[wasm_bindgen]
    pub async fn parse_units(&self, amount: &str, decimals: Option<u8>) -> Result<String, JsValue> {
        let decimal_places = match decimals {
            Some(d) => d,
            None => self.decimals().await?,
        };
        
        // Split the amount into whole and fractional parts
        let parts: Vec<&str> = amount.split('.').collect();
        let whole = parts[0].replace(',', "");
        let fraction = if parts.len() > 1 { parts[1] } else { "" };
        
        // Ensure the fraction is not longer than the token's decimal places
        if fraction.len() > decimal_places as usize {
            return Err(JsValue::from_str("Too many decimal places"));
        }
        
        // Construct the raw amount
        let mut raw_amount = whole;
        
        // Pad the fraction with zeros if needed
        let mut padded_fraction = fraction.to_string();
        while padded_fraction.len() < decimal_places as usize {
            padded_fraction.push('0');
        }
        
        raw_amount.push_str(&padded_fraction);
        
        // Remove leading zeros
        raw_amount = raw_amount.trim_start_matches('0').to_string();
        if raw_amount.is_empty() {
            raw_amount = "0".to_string();
        }
        
        Ok(raw_amount)
    }
} 