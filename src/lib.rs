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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<PackageMeta>,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_types: Option<BTreeMap<String, ContractType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<BTreeMap<String, BTreeMap<String, ContractInstance>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_dependencies: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PackageMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    authors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<BTreeMap<String, String>>,
}

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
    state_mutability: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    compiler: Option<CompilerInformation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    methods: Option<BTreeMap<String, Method>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_property: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dev: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Method {
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notice: Option<String>,
    #[serde(rename = "return")]
    #[serde(skip_serializing_if = "Option::is_none")]
    return_property: Option<EthValueType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dev: Option<String>,
}

impl Package {
    pub fn from_str(contents: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(contents)
    }

    pub fn to_string(&self) -> String {
        // Safe to unwrap because it should always be well-formed
        serde_json::to_string(&self).unwrap()
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

    #[test]
    fn serialize_to_string() {
        let p = Package {
            manifest_version: "2".to_string(),
            package_name: "This is only a test!".to_string(),
            meta: None,
            version: "1.2.3".to_string(),
            sources: None,
            contract_types: None,
            deployments: None,
            build_dependencies: None,
        };
        assert_eq!(
            p.to_string(),
            r#"{"manifest_version":"2","package_name":"This is only a test!","version":"1.2.3"}"#,
        );
    }
}
