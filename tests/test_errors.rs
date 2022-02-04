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
fn test_file_and_une() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("sengelebengele")
        .arg("-u")
        .arg("sengelebengele")
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
    Ok(())
}

#[test]
fn test_wrong_path() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("sengelebengele")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "Error: Failed to read Befunge source file",
        ));
    Ok(())
}

#[test]
fn test_nan_in_stdin() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let filename = testcase("echo_int");
    cmd.arg(filename)
        .write_stdin("x")
        .assert()
        .success()
        .stdout("0 ");
    Ok(())
}
