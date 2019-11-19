use std::fs;
use std::io::prelude::*;

use ::ethpm::Package;

macro_rules! test_case {
    ($name:ident, $file_name:literal) => {
        paste::item! {
            #[test]
            fn  [<test_ $name _v1>]() {
                let mut file = fs::File::open($file_name).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                // Try to parse without failure
                let package = Package::from_str(&contents).unwrap();
                assert_eq!(package.manifest_version, "2");
                assert_eq!(package.version, "1.0.0");
                assert_eq!(package.package_name, String::from(stringify!($name)).replace("_", "-"));
            }
        }
    }
}

test_case!(escrow,              "./ethpm-spec/examples/escrow/1.0.0.json");
test_case!(owned,               "./ethpm-spec/examples/owned/1.0.0.json");
test_case!(piper_coin,          "./ethpm-spec/examples/piper-coin/1.0.0.json");
test_case!(safe_math_lib,       "./ethpm-spec/examples/safe-math-lib/1.0.0.json");
test_case!(standard_token,      "./ethpm-spec/examples/standard-token/1.0.0.json");
test_case!(transferable,        "./ethpm-spec/examples/transferable/1.0.0.json");
test_case!(wallet,              "./ethpm-spec/examples/wallet/1.0.0.json");
test_case!(wallet_with_send,    "./ethpm-spec/examples/wallet-with-send/1.0.0.json");
