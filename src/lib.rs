
extern crate serde;
extern crate serde_json;

use std::hash::{BuildHasher, Hash, Hasher};

use std::fmt;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde::de;
use serde::de::{Visitor, MapAccess};
use std::marker::PhantomData;

//Rust-based transcription of Package standard as specified at https://github.com/ethpm/ethpm-spec/blob/master/spec/package.spec.json
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)] 
pub struct Package {
    manifest_version: String,
    package_name: String,
    meta: Option<PackageMeta>,
    version: String,
    sources: Option<HashMap<String,String>>,
    contract_types: Option<ContractType>,
    #[serde(skip_serializing)]
    #[serde(default)]
    deployments: Deployments<String,ContractInstances<String, ContractInstance>>,
    build_dependencies: Option<HashMap<String,String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageMeta {
    authors: Option<Vec<String>>,
    license: Option<String>,
    description: Option<String>,
    keywords: Option<Vec<String>>,
    links: Option<HashMap<String,String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractType {
    contract_name: Option<String>,
    deployment_bytecode: Option<BytecodeObject>,
    runtime_bytecode: Option<BytecodeObject>,
    abi: Option<ABI>,
    natspec: Option<serde_json::Value>,//TODO: Implement this as Rust structure
    compiler: Option<CompilerInformation>,
}

//Specially-derived JSON object
#[derive(Debug)]
pub struct Deployment {
    blockchain_hash: String,
    contract_instance_name: String,
    contract_instance: ContractInstance,
}

pub struct Deployments<K, V> {
    map : HashMap<K, V>,
}

impl<K: Hash + Eq, V> Deployments<K, V> {
    pub fn with_capacity(size: usize) -> Deployments<K, V> {
        Deployments { map: HashMap::new() }
    }
    pub fn insert(&self, key: K, value: V) {
        map.insert(key, value);
    }
}

pub struct ContractInstances<K, V> {
    contract_instance_name: K,
    contract_instance: V,
}

// pub fn deserialize_optional_deployments<'de, D>(d: D) -> Result<Option<Vec<Deployment>>, D::Error>
// where
//     D: de::Deserializer<'de>,
// {
//     d.deserialize_option(OptionalDeploymentsVisitor)
// }

// struct OptionalDeploymentsVisitor;

// impl<'de> de::Visitor<'de> for OptionalDeploymentsVisitor {
//     type Value = Option<Vec<Deployment>>;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         write!(formatter, "null or the deployed contract instances in this release")
//     }

//     fn visit_none<E>(self) -> Result<Self::Value, E>
//     where
//         E: de::Error,
//     {
//         Ok(None)
//     }

//     fn visit_some<D>(self, d: D) -> Result<Option<Vec<Deployment>>, D::Error>
//     where
//         D: de::Deserializer<'de>,
//     {
//         Ok(Some(d.deserialize_str(DeploymentsVisitor<String,ContractInstances<String, ContractInstance>>)?))
//     }
// }

// pub fn deserialize_deployments<'de, D>(d: D) -> Result<Vec<Deployment>, D::Error>
// where
//     D: de::Deserializer<'de>,
// {
//     d.deserialize_map(DeploymentsVisitor)
// }

struct DeploymentsVisitor<K, V> {
    marker: PhantomData<fn() -> Deployments<K, V>>
}

impl<K, V> DeploymentsVisitor<K, V> {
    fn new() -> Self {
        DeploymentsVisitor {
            marker: PhantomData
        }
    }
}

impl<'de, K, V> Visitor<'de> for DeploymentsVisitor<K, V>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    // The type that our Visitor is going to produce.
    type Value = Deployments<K, V>;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a very special map")
    }

    // Deserialize MyMap from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = Deployments::with_capacity(access.size_hint().unwrap_or(0));

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(map)
    }
}

// pub fn deserialize<'de, D>(d: D) -> Result<Vec<Deployment>, D::Error>
// where
//     D: de::Deserializer<'de>,
// {
//     d.deserialize_str(DeploymentsVisitor)
// }

// impl<'de> de::Visitor<'de> for DeploymentsVisitor {
//     type Value = Vec<Deployment>;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         write!(formatter, "the deployed contract instances in this release")
//     }

//     fn visit_map<V>(&mut self, mut visitor: V) -> Result<Vec<Deployment>, V::Error>
//         where V: MapAccess,
//             {
//         let deployments_object: serde_json::Value = serde_json::from_str(value).unwrap();
//         let deployments_map = deployments_object.as_object().unwrap();

//         let mut deployments = Vec::new();

//         for (chain, contract_instances) in deployments_map.iter() {
//             //TODO: Enforce conformance to regex "^blockchain\\://[0-9a-zA-Z]{64}/block/[0-9a-zA-Z]{64}$"
//             let blockchain_hash = chain;
//             let contract_instance_map = contract_instances.as_object().unwrap();

//             for (contract_instance_name, contract_instance) in contract_instance_map.iter() {
//                 let contract_instance: ContractInstance = serde_json::from_value(contract_instance.clone()).unwrap();

//                 deployments.push(
//                     Deployment{ blockchain_hash: blockchain_hash.clone(),
//                         contract_instance_name: contract_instance_name.clone(),
//                         contract_instance: contract_instance });
//             }
//         }

//         Ok(deployments)
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractInstance {
    contract_type: String,
    address: String,
    transaction: Option<String>,
    block: Option<String>,
    runtime_bytecode: Option<BytecodeObject>,
    compiler: Option<CompilerInformation>,
    link_dependencies: Option<Vec<LinkValue>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BytecodeObject {
    bytecode: Option<String>,
    link_references: Option<Vec<LinkReference>>,
    link_dependencies: Option<Vec<LinkValue>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ABI {
    constant: Option<bool>,
    inputs: Option<HashMap<String,String>>,
    name: String,
    outputs: Option<HashMap<String,String>>,
    payable: Option<bool>,
    #[serde(rename = "stateMutability")]
    state_mutability: Option<bool>,
    #[serde(rename = "type")]
    type_property: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EthTypes {
    name: String,
    #[serde(rename = "type")]
    type_property: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkReference {
    offsets: Vec<i64>,
    length: u64,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum LinkValueType {
    #[serde(rename = "literal")]
    Literal(String),
    #[serde(rename = "reference")]
    Reference(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkValue {
    offsets: Vec<i64>,
    #[serde(rename = "type")]
    type_property: LinkValueType,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompilerInformation {
    name: String,
    version: String,
    settings: Option<HashMap<String,String>>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct NatSpec {
//     name: String,
//     version: String,
//     settings: Option<HashMap<String,String>>,
// }

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
