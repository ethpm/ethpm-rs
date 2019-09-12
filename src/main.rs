extern crate ethpm;
use ethpm::Package;

use std::env;
use std::fs;
use std::process;

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguemnts: {}", err);
        process::exit(1);
    });
    let file_contents = fs::read_to_string(config.filename).unwrap_or_else(|err| {
        println!("Problem reading file: {}", err);
        process::exit(1);
    });
    let package: Package = serde_json::from_str(&file_contents).unwrap_or_else(|err| {
        println!("Problem parsing file: {}", err);
        process::exit(1);
    });

    // TODO: Actually do stuff here...
    // (see https://github.com/ethpm/ethpm-cli/blob/master/ethpm_cli/parser.py)
    println!("{:?}", package);
}
