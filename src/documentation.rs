//!
//! Types for documentation data structures
//!

#[cfg(all(feature = "no-std", not(test)))]
extern crate alloc;

#[cfg(all(feature = "no-std", not(test)))]
use alloc::vec::Vec;

#[cfg(all(feature = "no-std", not(test)))]
use alloc::collections::btree_map::BTreeMap;
#[cfg(any(feature = "default", feature = "std", test))]
use std::collections::BTreeMap;

extern crate serde;
use serde::{Deserialize, Serialize};

use crate::contract::EthValueType;

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
