use serde::{Deserialize, Serialize};

/// ABI item representing a function, event, or other contract element.
#[derive(Serialize, Deserialize, Clone)]
pub struct AbiItem {
    pub r#type: String,
    pub name: Option<String>,
    pub inputs: Option<Vec<AbiInput>>,
    pub outputs: Option<Vec<AbiOutput>>,
    pub state_mutability: Option<String>,
    pub anonymous: Option<bool>,
    pub constant: Option<bool>,
    pub payable: Option<bool>,
}

/// ABI input parameter definition.
#[derive(Serialize, Deserialize, Clone)]
pub struct AbiInput {
    pub name: String,
    pub r#type: String,
    pub indexed: Option<bool>,
    pub components: Option<Vec<AbiComponent>>,
}

/// ABI output parameter definition.
#[derive(Serialize, Deserialize, Clone)]
pub struct AbiOutput {
    pub name: String,
    pub r#type: String,
    pub components: Option<Vec<AbiComponent>>,
}

/// ABI component for tuple types.
#[derive(Serialize, Deserialize, Clone)]
pub struct AbiComponent {
    pub name: String,
    pub r#type: String,
    pub components: Option<Vec<AbiComponent>>,
}

/// Parse a JSON ABI string into a vector of AbiItems.
pub fn parse_abi(abi_json: &str) -> Result<Vec<AbiItem>, serde_json::Error> {
    serde_json::from_str(abi_json)
}

/// Get the function signature for a given function name and input types.
pub fn get_function_signature(name: &str, input_types: &[String]) -> String {
    let mut signature = name.to_string();
    signature.push('(');
    
    for (i, input_type) in input_types.iter().enumerate() {
        if i > 0 {
            signature.push(',');
        }
        signature.push_str(input_type);
    }
    
    signature.push(')');
    signature
}

/// Get the event signature for a given event name and input types.
pub fn get_event_signature(name: &str, input_types: &[String]) -> String {
    let mut signature = name.to_string();
    signature.push('(');
    
    for (i, input_type) in input_types.iter().enumerate() {
        if i > 0 {
            signature.push(',');
        }
        signature.push_str(input_type);
    }
    
    signature.push(')');
    signature
}

/// Determine if a function is a read-only function (view/pure).
pub fn is_read_only(abi_item: &AbiItem) -> bool {
    if let Some(state_mutability) = &abi_item.state_mutability {
        state_mutability == "view" || state_mutability == "pure"
    } else if let Some(constant) = abi_item.constant {
        constant
    } else {
        false
    }
}

/// Determine if a function is payable.
pub fn is_payable(abi_item: &AbiItem) -> bool {
    if let Some(state_mutability) = &abi_item.state_mutability {
        state_mutability == "payable"
    } else if let Some(payable) = abi_item.payable {
        payable
    } else {
        false
    }
}

/// A simplified list of Ethereum value types.
pub enum EthereumType {
    Address,
    Uint(usize),
    Int(usize),
    Bool,
    String,
    Bytes,
    FixedBytes(usize),
    Array(Box<EthereumType>),
    FixedArray(Box<EthereumType>, usize),
    Tuple(Vec<EthereumType>),
}

/// Parse a type string into an EthereumType.
pub fn parse_type(type_str: &str) -> Option<EthereumType> {
    if type_str == "address" {
        Some(EthereumType::Address)
    } else if type_str == "bool" {
        Some(EthereumType::Bool)
    } else if type_str == "string" {
        Some(EthereumType::String)
    } else if type_str == "bytes" {
        Some(EthereumType::Bytes)
    } else if type_str.starts_with("uint") {
        let size_str = &type_str[4..];
        if let Ok(size) = size_str.parse::<usize>() {
            if size % 8 == 0 && size <= 256 {
                return Some(EthereumType::Uint(size));
            }
        }
        None
    } else if type_str.starts_with("int") {
        let size_str = &type_str[3..];
        if let Ok(size) = size_str.parse::<usize>() {
            if size % 8 == 0 && size <= 256 {
                return Some(EthereumType::Int(size));
            }
        }
        None
    } else if type_str.starts_with("bytes") {
        let size_str = &type_str[5..];
        if let Ok(size) = size_str.parse::<usize>() {
            if size >= 1 && size <= 32 {
                return Some(EthereumType::FixedBytes(size));
            }
        }
        None
    } else if type_str.ends_with("[]") {
        let element_type_str = &type_str[..type_str.len() - 2];
        if let Some(element_type) = parse_type(element_type_str) {
            return Some(EthereumType::Array(Box::new(element_type)));
        }
        None
    } else if let Some(start_pos) = type_str.find('[') {
        if let Some(end_pos) = type_str.find(']') {
            let element_type_str = &type_str[..start_pos];
            let size_str = &type_str[start_pos + 1..end_pos];
            
            if let (Some(element_type), Ok(size)) = (parse_type(element_type_str), size_str.parse::<usize>()) {
                return Some(EthereumType::FixedArray(Box::new(element_type), size));
            }
        }
        None
    } else if type_str == "tuple" {
        Some(EthereumType::Tuple(Vec::new()))
    } else {
        None
    }
} 