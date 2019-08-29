extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};
//use http::Uri;

//Rust-based transcription of Package standard as specified at https://github.com/ethpm/ethpm-spec/blob/master/spec/package.spec.json
#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    manifest_version: String,
    package_name: String,
    meta: Option<PackageMeta>,
    version: String,
    sources: Option<u8>,
    contract_types: Option<ContractType>,
    deployments: Option<Deployment>,
    build_dependencies: Option<ContentURI>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageMeta {
    authors: Option<Vec<String>>,
    license: Option<String>,
    description: Option<String>,
    keywords: Option<Vec<String>>,
    links: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractType {
    contract_name: Option<String>,
    deployment_bytecode: Option<BytecodeObject>,
    runtime_bytecode: Option<BytecodeObject>,
    abi: Option<Vec<u8>>,
    natspec: Option<String>,
    compiler: Option<CompilerInformation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractInstance {
    contract_type: String,
    address: Address,
    transaction: Option<TransactionHash>,
    block: Option<BlockHash>,
    runtime_bytecode: Option<BytecodeObject>,
    compiler: Option<CompilerInformation>,
    link_dependencies: Option<Vec<LinkValue>>,
}

//ByteString: not used explicitly

#[derive(Serialize, Deserialize, Debug)]
pub struct BytecodeObject {
    bytecode: Option<String>,//as ByteString
    link_references: Option<Vec<LinkReference>>,
    link_dependencies: Option<Vec<LinkValue>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkReference {
    offsets: Vec<i64>,
    length: u64,
    name: Identifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkValue {
    offsets: Vec<i64>,
    #[serde(rename = "type")]
    json_type: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identifier {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractInstanceName {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deployment {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageContractInstanceName {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompilerInformation {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionHash {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHash {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentURI {
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
