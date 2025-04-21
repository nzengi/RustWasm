use wasm_bindgen::prelude::*;

/// Utility functions for Ethereum contract operations

/// Convert a hex string to a decimal string
pub fn hex_to_decimal(hex: &str) -> Result<String, String> {
    // Remove 0x prefix if present
    let clean_hex = hex.trim_start_matches("0x");
    
    // Parse the hex string
    match u128::from_str_radix(clean_hex, 16) {
        Ok(value) => Ok(value.to_string()),
        Err(e) => Err(format!("Failed to parse hex value: {}", e)),
    }
}

/// Convert a decimal string to a hex string
pub fn decimal_to_hex(decimal: &str) -> Result<String, String> {
    // Parse the decimal string
    match decimal.parse::<u128>() {
        Ok(value) => Ok(format!("0x{:x}", value)),
        Err(e) => Err(format!("Failed to parse decimal value: {}", e)),
    }
}

/// Check if a string is a valid address
pub fn is_valid_address(address: &str) -> bool {
    if !address.starts_with("0x") {
        return false;
    }
    
    let address = address.trim_start_matches("0x");
    if address.len() != 40 {
        return false;
    }
    
    // Check if the address contains only hex characters
    address.chars().all(|c| c.is_digit(16))
}

/// Pad a hex string to a specific length
pub fn pad_hex(hex: &str, length: usize) -> String {
    let clean_hex = hex.trim_start_matches("0x");
    let padded = format!("{:0>width$}", clean_hex, width = length);
    format!("0x{}", padded)
}

/// Convert a value to Wei (smallest Ethereum unit)
pub fn to_wei(value: f64, unit: &str) -> Result<String, String> {
    let multiplier = match unit.to_lowercase().as_str() {
        "wei" => 1.0,
        "kwei" | "babbage" | "femtoether" => 1_000.0,
        "mwei" | "lovelace" | "picoether" => 1_000_000.0,
        "gwei" | "shannon" | "nanoether" | "nano" => 1_000_000_000.0,
        "microether" | "micro" => 1_000_000_000_000.0,
        "milliether" | "milli" => 1_000_000_000_000_000.0,
        "ether" | "eth" => 1_000_000_000_000_000_000.0,
        _ => return Err(format!("Unknown unit: {}", unit)),
    };
    
    // Calculate wei value
    let wei = value * multiplier;
    
    // Round to integer
    let wei_int = wei.round() as u128;
    
    Ok(wei_int.to_string())
}

/// Format a number with commas for thousands
pub fn format_with_commas(value: &str) -> String {
    let mut chars: Vec<char> = value.chars().collect();
    let mut i = chars.len();
    
    // Add commas from right to left
    while i > 3 {
        i -= 3;
        chars.insert(i, ',');
    }
    
    chars.into_iter().collect()
} 