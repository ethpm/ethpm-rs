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
pub struct Package<'a> {
    manifest_version: &'a str, //TODO: Rustify
    package_name: &'a str,
    meta: Option<PackageMeta<'a>>,
    version: &'a str, //TODO: Rustify
    sources: Option<BTreeMap<&'a str, &'a str>>, //TODO: Rustify
    contract_types: Option<BTreeMap<&'a str, ContractType<'a>>>, //TODO: Rustify
    deployments: Option<BTreeMap<&'a str, BTreeMap<&'a str, ContractInstance<'a>>>>, //TODO: Rustify
    build_dependencies: Option<BTreeMap<&'a str, &'a str>>, //TODO: Rustify
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PackageMeta<'a> {
    authors: Option<Vec<&'a str>>,
    license: Option<&'a str>,
    description: Option<&'a str>,
    keywords: Option<Vec<&'a str>>,
    links: Option<BTreeMap<&'a str, &'a str>>, //TODO: Rustify
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ContractType<'a> {
    contract_name: Option<&'a str>,
    deployment_bytecode: Option<BytecodeObject<'a>>,
    runtime_bytecode: Option<BytecodeObject<'a>>,
    abi: Option<Vec<ABI<'a>>>,
    natspec: Option<NatSpec<'a>>,
    compiler: Option<CompilerInformation<'a>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ContractInstance<'a> {
    contract_type: &'a str,
    address: &'a str,
    transaction: Option<&'a str>,
    block: Option<&'a str>,
    deployment_bytecode: Option<BytecodeObject<'a>>,
    runtime_bytecode: Option<BytecodeObject<'a>>,
    compiler: Option<CompilerInformation<'a>>,
    link_dependencies: Option<Vec<LinkValue<'a>>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BytecodeObject<'a> {
    bytecode: Option<&'a str>,
    link_references: Option<Vec<LinkReference<'a>>>,
    link_dependencies: Option<Vec<LinkValue<'a>>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ABI<'a> {
    constant: Option<bool>,
    anonymous: Option<bool>,
    inputs: Option<Vec<EthTypes<'a>>>,
    name: Option<&'a str>,
    outputs: Option<Vec<EthTypes<'a>>>,
    payable: Option<bool>,
    #[serde(rename = "stateMutability")]
    state_mutability: Option<&'a str>, //TODO: Rustify
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
pub struct EthTypes<'a> {
    indexed: Option<bool>,
    name: &'a str,
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
pub struct LinkReference<'a> {
    offsets: Vec<i64>,
    length: u64,
    name: &'a str,
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
pub struct LinkValue<'a> {
    offsets: Vec<i64>,
    #[serde(rename = "type")]
    type_property: LinkValueType,
    value: &'a str, //TODO: Rustify
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CompilerInformation<'a> {
    name: CompilerType,
    version: &'a str, //TODO: Rustify
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
pub struct NatSpec<'a> {
    compiler: Option<CompilerInformation<'a>>,
    author: Option<&'a str>,
    methods: Option<BTreeMap<&'a str, Method<'a>>>,
    title: Option<&'a str>,
    description: Option<&'a str>,
    #[serde(rename = "type")]
    type_property: Option<&'a str>, //TODO: Rustify
    dev: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Method<'a> {
    details: Option<&'a str>,
    params: Option<BTreeMap<&'a str, &'a str>>,
    notice: Option<&'a str>,
    #[serde(rename = "return")]
    return_property: Option<EthValueType>,
    dev: Option<&'a str>, //TODO: Rustify
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
        let package: Package = serde_json::from_str(&example).unwrap();
        assert_eq!(package.manifest_version, "2");
        assert_eq!(package.package_name, "This is only a test!");
        assert_eq!(package.version, "1.2.3");
    }
}
