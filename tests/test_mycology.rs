use anyhow::Result;
use assert_cmd::Command;
use predicates::{boolean::PredicateBooleanExt, prelude::predicate};
use std::str;

const TESTDIR: &str = "tests/mycology/";
const SUFFIX98: &str = ".b98";
const SUFFIX93: &str = ".bf";

fn testcase93(name: &str) -> String {
    TESTDIR.to_owned() + name + &SUFFIX93.to_owned()
}

fn testcase98(name: &str) -> String {
    TESTDIR.to_owned() + name + &SUFFIX98.to_owned()
}

#[test]
fn test_mycorand() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let filename = testcase93("mycorand");
    let predicate_fn = predicate::str::is_match(
        "The directions were generated in the order [<>^v]{4}\n\\? was met [0-9]+ times\n",
    )
    .unwrap();
    cmd.arg(filename).assert().success().stdout(predicate_fn);

    Ok(())
}

#[test]
fn test_mycology() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(testcase98("mycology"))
        .assert()
        .failure()
        .code(predicate::eq(15))
        .stdout(predicate::str::contains("BAD:").not());

    Ok(())
}
