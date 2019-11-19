//!
//! Types for working with contracts in manifests
//!

#[cfg(all(feature = "no-std", not(test)))]
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::documentation::{CompilerInformation, NatSpec, LinkValue, LinkReference}; 

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ContractType {
    #[serde(skip_serializing_if = "Option::is_none")]
    contract_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_bytecode: Option<BytecodeObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_bytecode: Option<BytecodeObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    abi: Option<Vec<ABI>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    natspec: Option<NatSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compiler: Option<CompilerInformation>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ContractInstance {
    contract_type: String,
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_bytecode: Option<BytecodeObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_bytecode: Option<BytecodeObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compiler: Option<CompilerInformation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_dependencies: Option<Vec<LinkValue>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BytecodeObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    bytecode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_references: Option<Vec<LinkReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_dependencies: Option<Vec<LinkValue>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ABI {
    #[serde(skip_serializing_if = "Option::is_none")]
    constant: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inputs: Option<Vec<EthTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outputs: Option<Vec<EthTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payable: Option<bool>,
    #[serde(rename = "stateMutability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    state_mutability: Option<StateMutability>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_property: Option<EthMethodType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum EthMethodType {
    #[serde(rename = "constructor")]
    Constructor,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "fallback")]
    Fallback,
    #[serde(rename = "event")]
    Event,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EthTypes {
    #[serde(skip_serializing_if = "Option::is_none")]
    indexed: Option<bool>,
    name: String,
    #[serde(rename = "type")]
    type_property: EthValueType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum EthValueType {
    #[serde(rename = "address")]
    Address,
    #[serde(rename = "uint256")]
    Uint256,
    #[serde(rename = "bool")]
    Bool,
    //TODO: Add complete list of value types
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum StateMutability {
    #[serde(rename = "nonpayable")]
    NonPayable,
    #[serde(rename = "view")]
    View,
    #[serde(rename = "pure")]
    Pure,
    #[serde(rename = "payable")]
    Payable,
}
