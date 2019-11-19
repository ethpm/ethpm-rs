use ethpm;

use proptest::prelude::*;

// Ensure Package structure conforms to the spec
// present in ethpm-spec/spec/package.spec.json
proptest! {
    #[test]
    fn package_struct_meets_spec(
        manifest_version in "2",
        package_name in "[a-z][-a-z0-9]{0,255}",
        version in "v[0.9]\\.[0-9](\\.0-9])?",
    ) {
        let data = format!(r#"{{
            "manifest_version": "{}",
            "package_name": "{}",
            "version": "{}"
        }}"#, manifest_version, package_name, version);
        println!("{}", data);
        let package: ethpm::Package = serde_json::from_str(&data).unwrap();

        assert_eq!(package.manifest_version, manifest_version);
        assert_eq!(package.package_name, package_name);
        assert_eq!(package.version, version);
    }
}
