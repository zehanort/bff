use anyhow::Result;
use assert_cmd::Command;
use std::str;

const TESTDIR: &str = "tests/bf98/";
const SUFFIX: &str = ".b98";

fn testcase(name: &str) -> String {
    TESTDIR.to_owned() + name + &SUFFIX.to_owned()
}

#[test]
fn test_hello_world() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(testcase("hello_world"))
        .assert()
        .success()
        .stdout("Hello world!");

    Ok(())
}

#[test]
fn test_k1() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(testcase("k1")).assert().success().stdout("0 ");

    Ok(())
}

#[test]
fn test_k2() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(testcase("k2")).assert().success().stdout("0 ");

    Ok(())
}

#[test]
fn test_k3() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(testcase("k3")).assert().success().stdout("1 ");

    Ok(())
}

#[test]
fn test_k4() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(testcase("k4")).assert().success().stdout("Hello");

    Ok(())
}

#[test]
fn test_k5() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(testcase("k5")).assert().success().stdout("Hello");

    Ok(())
}

#[test]
#[ignore = "Negative Funge-Space is not implemented yet"]
fn test_roundabout() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(testcase("roundabout"))
        .assert()
        .success()
        .stdout("OK");

    Ok(())
}
