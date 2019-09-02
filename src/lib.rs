#[cfg(all(feature = "no-std", not(test)))]
#[no_std]

// We always pull in `std` during tests, because it's just easier
// to write tests when you can assume you're on a capable platform
#[cfg(any(feature = "default", feature = "std", test))]
#[macro_use]
extern crate std;

// When we're building for a no-std target, we pull in `core`, but alias
// it as `std` so the `use` statements are the same between `std` and `core`.
#[cfg(all(feature = "no-std", not(test)))]
#[macro_use]
extern crate core as std;

#[cfg(all(feature = "no-std", not(test)))]
extern crate alloc;

#[cfg(all(feature = "no-std", not(test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "default", feature = "std", test))]
use std::collections::BTreeMap;
#[cfg(all(feature = "no-std", not(test)))]
use alloc::collections::btree_map::BTreeMap;

extern crate serde;
#[cfg(any(feature = "default", feature = "std", test))]
extern crate serde_json;
#[cfg(all(feature = "no-std", not(test)))]
extern crate serde_json_core;

use serde::{Serialize, Deserialize};

//Rust-based transcription of Package standard as specified at https://github.com/ethpm/ethpm-spec/blob/master/spec/package.spec.json
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)] 
pub struct Package<'a> {
    manifest_version: &'a str,
    package_name: &'a str,
    meta: Option<PackageMeta<'a>>,
    version: &'a str,
    sources: Option<BTreeMap<&'a str,&'a str>>,
    contract_types: Option<ContractType<'a>>,
    deployments: Option<BTreeMap<&'a str,BTreeMap<&'a str, ContractInstance<'a>>>>,
    build_dependencies: Option<BTreeMap<&'a str,&'a str>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageMeta<'a> {
    authors: Option<Vec<&'a str>>,
    license: Option<&'a str>,
    description: Option<&'a str>,
    keywords: Option<Vec<&'a str>>,
    links: Option<BTreeMap<&'a str,&'a str>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractType<'a> {
    contract_name: Option<&'a str>,
    deployment_bytecode: Option<BytecodeObject<'a>>,
    runtime_bytecode: Option<BytecodeObject<'a>>,
    abi: Option<ABI<'a>>,
    natspec: Option<NatSpec<'a>>,
    compiler: Option<CompilerInformation<'a>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractInstance<'a> {
    contract_type: &'a str,
    address: &'a str,
    transaction: Option<&'a str>,
    block: Option<&'a str>,
    runtime_bytecode: Option<BytecodeObject<'a>>,
    compiler: Option<CompilerInformation<'a>>,
    link_dependencies: Option<Vec<LinkValue<'a>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BytecodeObject<'a> {
    bytecode: Option<&'a str>,
    link_references: Option<Vec<LinkReference<'a>>>,
    link_dependencies: Option<Vec<LinkValue<'a>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ABI<'a> {
    constant: Option<bool>,
    inputs: Option<BTreeMap<&'a str,&'a str>>,
    name: &'a str,
    outputs: Option<BTreeMap<&'a str,&'a str>>,
    payable: Option<bool>,
    #[serde(rename = "stateMutability")]
    state_mutability: Option<bool>,
    #[serde(rename = "type")]
    type_property: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EthTypes<'a> {
    name: &'a str,
    #[serde(rename = "type")]
    type_property: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkReference<'a> {
    offsets: Vec<i64>,
    length: u64,
    name: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
enum LinkValueType {
    #[serde(rename = "literal")]
    Literal(),
    #[serde(rename = "reference")]
    Reference,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkValue<'a> {
    offsets: Vec<i64>,
    #[serde(rename = "type")]
    type_property: LinkValueType,
    value: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompilerInformation<'a> {
    name: &'a str,
    version: &'a str,
    settings: Option<BTreeMap<&'a str,serde_json::Value>>,//TODO: Implement this as Rust structure
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NatSpec<'a> {
    compiler: Option<CompilerInformation<'a>>,
    author: Option<&'a str>,
    methods: Option<BTreeMap<&'a str,Method<'a>>>,
    title: Option<&'a str>,
    description: Option<&'a str>,
    #[serde(rename = "type")]
    type_property: Option<&'a str>,
    dev: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Method<'a> {
    details: Option<&'a str>,
    params: Option<BTreeMap<&'a str,&'a str>>,
    notice: Option<&'a str>,
    #[serde(rename = "return")]
    return_property: Option<serde_json::Value>,//TODO: Implement this as Rust structure
    dev: Option<&'a str>,
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
