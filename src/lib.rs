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
    // Ensure Package structure conforms to the spec
    // present in ethpm-spec/spec/package.spec.json
    #[test]
    fn package_struct_meets_spec() {
    }

    // Try to parse packages located in
    // ethpm-spec/examples/**/*.json
    #[test]
    fn can_parse_examples() {
    }
}
