use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::str;

const TESTDIR: &str = "tests/bf93/";
const SUFFIX: &str = ".bf";

fn testcase(name: &str) -> String {
    TESTDIR.to_owned() + name + &SUFFIX.to_owned()
}

#[test]
fn test_no_file() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.assert()
        .failure()
        .stderr("Error: No Befunge 98 program file was provided\n");
    Ok(())
}

#[test]
fn test_wrong_path() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("sengelebengele")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "Error: Failed to open Befunge source file",
        ));
    Ok(())
}

#[test]
fn test_nan_in_stdin() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let filename = testcase("factorial");
    cmd.arg(filename)
        .write_stdin("x")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Failed while parsing input from stdin as integer",
        ));
    Ok(())
}
