//!
//! Types for working with EthPM Manifests
//!

#[cfg(all(feature = "no-std", not(test)))]
#[no_std]

#[cfg(all(feature = "no-std", not(test)))]
use alloc::vec::Vec;

#[cfg(all(feature = "no-std", not(test)))]
use alloc::collections::btree_map::BTreeMap;
#[cfg(any(feature = "default", feature = "std", test))]
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

mod contract;
use contract::{ContractType, ContractInstance};

mod documentation;

// TODO: Use this temporarily until web3::types implemented no-std
mod bytes;

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
