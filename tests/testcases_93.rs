use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::{fs, str};

const TESTDIR: &str = "tests/bf93/";
const SUFFIX: &str = ".bf";

fn testcase(name: &str) -> String {
    TESTDIR.to_owned() + name + &SUFFIX.to_owned()
}

#[test]
fn test_sanity() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(testcase("sanity"))
        .assert()
        .success()
        .stdout("0 1 2 3 4 5 6 7 8 9 ");

    Ok(())
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
fn test_factorial() -> Result<()> {
    let filename = testcase("factorial");
    for (query, answer) in [("4", "24 "), ("6", "720 "), ("10", "3628800 ")] {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
        cmd.arg(&filename)
            .write_stdin(query)
            .assert()
            .stdout(answer);
    }

    Ok(())
}

#[test]
fn test_quine() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let filename = testcase("quine");
    let contents = fs::read_to_string(&filename)?;
    cmd.arg(filename).assert().success().stdout(contents);

    Ok(())
}

#[test]
fn test_compare() -> Result<()> {
    let filename = testcase("compare");
    for (a, b, comp) in [("12", "135", "<"), ("53", "-123", ">"), ("42", "42", "=")] {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
        cmd.arg(&filename)
            .write_stdin(format!("{}\n{}", a, b))
            .assert()
            .success()
            .stdout(format!("A={}  B={} \nA {} B\n", a, b, comp));
    }

    Ok(())
}

#[test]
fn test_primecheck() -> Result<()> {
    let filename = testcase("primecheck");
    for p in ["5", "7", "139", "337", "1193", "4357", "15269", "108877"] {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
        cmd.arg(&filename)
            .write_stdin(p)
            .assert()
            .success()
            .stdout("Yes");
    }
    for np in ["6", "8", "140", "338", "1194", "4358", "15279", "108977"] {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
        cmd.arg(&filename)
            .write_stdin(np)
            .assert()
            .success()
            .stdout("No");
    }

    Ok(())
}

#[test]
fn test_echo() -> Result<()> {
    let filename = testcase("echo");
    let c: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1)
        .map(char::from)
        .collect();
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(filename)
        .write_stdin(format!("{}", c))
        .assert()
        .success()
        .stdout(format!("{}", c));
    Ok(())
}

#[test]
fn test_borders() -> Result<()> {
    let filename = testcase("borders");
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(filename).assert().success();

    Ok(())
}

#[test]
/// "get" from outside of the program borders should return " "
fn test_getout() -> Result<()> {
    let filename = testcase("getout");
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(filename).assert().success().stdout("  ");

    Ok(())
}

#[test]
fn test_div_by_zero() -> Result<()> {
    let filename = testcase("div_by_zero");
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(filename)
        .assert()
        .success()
        .stdout("0 ")
        .stderr(predicate::str::ends_with(
            "Division by 0 occured. Will return 0 as per the language specification.\n",
        ));

    Ok(())
}

#[test]
fn test_read_int_with_chars() -> Result<()> {
    let filename = testcase("echo_int");

    for (pre, suf) in [("", ""), ("abc", ""), ("", "def"), ("abc", "def")] {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
        cmd.arg(&filename)
            .write_stdin(format!("{}12345{}", pre, suf))
            .assert()
            .success()
            .stdout("12345 ");
    }

    for (pre, suf) in [("", ""), ("abc", ""), ("", "def"), ("abc", "def")] {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
        cmd.arg(&filename)
            .write_stdin(format!("{}-12345{}", pre, suf))
            .assert()
            .success()
            .stdout("-12345 ");
    }

    Ok(())
}

#[test]
fn test_read_int_limits() -> Result<()> {
    let filename = testcase("echo_int");
    for (n_in, n_out) in [
        ("2147483647", "2147483647 "),
        ("2147483648", "214748364 "),
        ("-2147483647", "-2147483647 "),
        ("-2147483648", "-214748364 "),
    ] {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
        cmd.arg(&filename)
            .write_stdin(n_in)
            .assert()
            .success()
            .stdout(n_out);
    }

    Ok(())
}
