extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    // Required Members
    manifest_version: String,
    package_name: String,
    version: String,
    // Other members here...
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::prelude::*;

    extern crate glob;
    use glob::glob;

    fn get_examples() -> Vec<String> {
        glob("./ethpm-spec/examples/**/*.json")
            .expect("Make sure to load git submodules")
            .map(|file| file.unwrap())
            .map(|file| {
                let mut file = fs::File::open(file).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                contents
            })
            .collect()
    }

    use super::*;

    // Ensure Package structure conforms to the spec
    // present in ethpm-spec/spec/package.spec.json
    // Note: fuzzing based on this spec?
    #[test]
    fn package_struct_meets_spec() {
    }

    // Try to parse packages located in
    // ethpm-spec/examples/**/*.json
    // TODO: Generate 1 case per example
    #[test]
    fn can_parse_examples() {
        for example in get_examples() {
            // Try to parse without failure
            let package: Package = serde_json::from_str(&example).unwrap();
            let raw_object: serde_json::Value = serde_json::from_str(&example).unwrap();
            // TODO iterate through `raw_object` and assert field match to `package`
        }
    }
}
