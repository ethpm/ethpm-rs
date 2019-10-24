use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn missing_arguments() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.assert().failure();
}

#[test]
fn file_doesnt_exist() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.arg("test/file/doesnt/exist");
    cmd.assert().failure();
}

#[test]
fn file_not_json() {
    let mut tempfile = NamedTempFile::new().unwrap();

    writeln!(tempfile, "This is not a JSON file.").unwrap();

    let mut cmd = Command::main_binary().unwrap();
    cmd.arg(tempfile.path());
    cmd.assert().failure();
}

#[test]
fn file_not_ethpm() {
    let example = r#"
    {
        "manifest_version": "2",
        "package_name": "This is only a test!"
    }
    "#;

    let mut tempfile = NamedTempFile::new().unwrap();

    tempfile.write_all(example.as_bytes()).unwrap();

    let mut cmd = Command::main_binary().unwrap();
    cmd.arg(tempfile.path());
    cmd.assert().failure();
}

#[test]
fn file_correct() {
    let example = r#"
    {
        "manifest_version": "2",
        "package_name": "This is only a test!",
        "version": "1.2.3"
    }
    "#;

    let mut tempfile = NamedTempFile::new().unwrap();

    tempfile.write_all(example.as_bytes()).unwrap();

    let mut cmd = Command::main_binary().unwrap();
    cmd.arg(tempfile.path());
    cmd.assert().success();
}
