use std::fs;
use std::io::prelude::*;

extern crate glob;
use glob::glob;

use ::ethpm::Package;

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

// Try to parse packages located in
// ethpm-spec/examples/**/*.json
// TODO: Generate 1 case per example
#[test]
fn can_parse_examples() {
    for example in get_examples() {
        // let raw_object: serde_json::Value = serde_json::from_str(&example).unwrap();
        // println!("JSON: {}", raw_object);

        // Try to parse without failure
        let package: Package = serde_json::from_str(&example).unwrap();

        println!("Package: {:?}", package);
    }
}
