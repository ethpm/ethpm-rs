#[cfg(all(feature = "no-std", not(test)))]
#[no_std]

#[cfg(any(feature = "default", feature = "std", test))]
#[macro_use]
extern crate std;

#[cfg(all(feature = "no-std", not(test)))]
#[macro_use]
extern crate core as std;

#[cfg(all(feature = "no-std", not(test)))]
extern crate alloc;

#[cfg(all(feature = "no-std", not(test)))]
use alloc::vec::Vec;

#[cfg(all(feature = "no-std", not(test)))]
use alloc::collections::btree_map::BTreeMap;
#[cfg(any(feature = "default", feature = "std", test))]
use std::collections::BTreeMap;

extern crate serde;
#[cfg(any(feature = "default", feature = "std", test))]
extern crate serde_json;
#[cfg(all(feature = "no-std", not(test)))]
extern crate serde_json_core;

use serde::{Deserialize, Serialize};

//Rust-based transcription of Package standard as specified at https://github.com/ethpm/ethpm-spec/blob/master/spec/package.spec.json
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Package {
    pub manifest_version: String,
    pub package_name: String,
    pub meta: Option<PackageMeta>,
    pub version: String,
    pub sources: Option<BTreeMap<String, String>>,
    pub contract_types: Option<BTreeMap<String, ContractType>>,
    pub deployments: Option<BTreeMap<String, BTreeMap<String, ContractInstance>>>,
    pub build_dependencies: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PackageMeta {
    authors: Option<Vec<String>>,
    license: Option<String>,
    description: Option<String>,
    keywords: Option<Vec<String>>,
    links: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ContractType {
    contract_name: Option<String>,
    deployment_bytecode: Option<BytecodeObject>,
    runtime_bytecode: Option<BytecodeObject>,
    abi: Option<Vec<ABI>>,
    natspec: Option<NatSpec>,
    compiler: Option<CompilerInformation>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ContractInstance {
    contract_type: String,
    address: String,
    transaction: Option<String>,
    block: Option<String>,
    deployment_bytecode: Option<BytecodeObject>,
    runtime_bytecode: Option<BytecodeObject>,
    compiler: Option<CompilerInformation>,
    link_dependencies: Option<Vec<LinkValue>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BytecodeObject {
    bytecode: Option<String>,
    link_references: Option<Vec<LinkReference>>,
    link_dependencies: Option<Vec<LinkValue>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ABI {
    constant: Option<bool>,
    anonymous: Option<bool>,
    inputs: Option<Vec<EthTypes>>,
    name: Option<String>,
    outputs: Option<Vec<EthTypes>>,
    payable: Option<bool>,
    #[serde(rename = "stateMutability")]
    state_mutability: Option<String>,
    #[serde(rename = "type")]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LinkReference {
    offsets: Vec<i64>,
    length: u64,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
enum LinkValueType {
    #[serde(rename = "literal")]
    Literal,
    #[serde(rename = "reference")]
    Reference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LinkValue {
    offsets: Vec<i64>,
    #[serde(rename = "type")]
    type_property: LinkValueType,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CompilerInformation {
    name: CompilerType,
    version: String,
    settings: Option<CompilerSettings>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum CompilerType {
    #[serde(rename = "solc")]
    Solc,
    #[serde(rename = "vyper")]
    Vyper,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CompilerSettings {
    optimize: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct NatSpec {
    compiler: Option<CompilerInformation>,
    author: Option<String>,
    methods: Option<BTreeMap<String, Method>>,
    title: Option<String>,
    description: Option<String>,
    #[serde(rename = "type")]
    type_property: Option<String>,
    dev: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Method {
    details: Option<String>,
    params: Option<BTreeMap<String, String>>,
    notice: Option<String>,
    #[serde(rename = "return")]
    return_property: Option<EthValueType>,
    dev: Option<String>,
}

impl Package {
    pub fn from_str(contents: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_package_parses() {
        let example = r#"
        {
            "manifest_version": "2",
            "package_name": "This is only a test!",
            "version": "1.2.3"
        }
        "#;
        let package = Package::from_str(&example).unwrap();
        assert_eq!(package.manifest_version, "2");
        assert_eq!(package.package_name, "This is only a test!");
        assert_eq!(package.version, "1.2.3");
    }
}
