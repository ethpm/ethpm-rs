extern crate ethpm;
use ethpm::Package;

extern crate clap;
use clap::{Arg, ArgMatches, App};

use std::fs;
use std::process;

struct Config {
    filename: String,
}

impl Config {
    pub fn parse(matches: ArgMatches) -> Self {
        // `.unwrap()` is safe here because FILENAME is required
        let filename = matches.value_of("FILENAME").unwrap().to_string();
        Config {
            filename
        }
    }
}

pub fn main() {
    let matches = App::new("")
                    .version("")
                    .author("")
                    .about("")
                    .arg(Arg::with_name("FILENAME")
                            .help("")
                            .required(true)
                        )
                    .get_matches();

    let config = Config::parse(matches);

    let file_contents = fs::read_to_string(config.filename).unwrap_or_else(|err| {
        println!("Problem reading file: {}", err);
        process::exit(1);
    });

    let package = Package::from_str(&file_contents).unwrap_or_else(|err| {
        println!("Problem parsing file: {}", err);
        process::exit(1);
    });

    // TODO: Actually do stuff here...
    // (see https://github.com/ethpm/ethpm-cli/blob/master/ethpm_cli/parser.py)
    println!("{:?}", package);
}
