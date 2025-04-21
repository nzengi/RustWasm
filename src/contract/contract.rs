use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::utils::hex_to_decimal;
use std::collections::HashMap;

/// Contract implementation with function definitions.
/// This module contains the actual implementation of the Contract struct
/// but exports the Contract struct and related types via mod.rs.

#[derive(Clone)]
pub struct Contract {
    pub address: String,
    pub abi: String,
    pub functions: HashMap<String, Function>,
    pub events: HashMap<String, Event>,
}

/// Represents a function in a smart contract
#[derive(Serialize, Deserialize, Clone)]
pub struct Function {
    pub name: String,
    pub inputs: Vec<Parameter>,
    pub outputs: Vec<Parameter>,
    pub state_mutability: StateMutability,
}

/// Represents an event in a smart contract
#[derive(Serialize, Deserialize, Clone)]
pub struct Event {
    pub name: String,
    pub inputs: Vec<EventParameter>,
    pub anonymous: bool,
}

/// Parameter for a function
#[derive(Serialize, Deserialize, Clone)]
pub struct Parameter {
    pub name: String,
    pub r#type: String,
    pub components: Option<Vec<Parameter>>,
}

/// Parameter for an event
#[derive(Serialize, Deserialize, Clone)]
pub struct EventParameter {
    pub name: String,
    pub r#type: String,
    pub indexed: bool,
    pub components: Option<Vec<Parameter>>,
}

/// Function state mutability
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    Pure,
    View,
    Nonpayable,
    Payable,
} 