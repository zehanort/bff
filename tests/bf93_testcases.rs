use assert_cmd::Command;
use std::fs;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

const TESTDIR: &str = "tests/bf93/";
const SUFFIX: &str = ".bf";

fn testcase(name: &str) -> String {
    TESTDIR.to_owned() + name + &SUFFIX.to_owned()
}

#[test]
fn test_sanity() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(testcase("sanity"));
    cmd.assert().success().stdout("0 1 2 3 4 5 6 7 8 9 ");

    Ok(())
}

#[test]
fn test_hello_world() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(testcase("hello_world"));
    cmd.assert().success().stdout("Hello world!");

    Ok(())
}

#[test]
fn test_factorial() -> Result<()> {
    let testcase_file = testcase("factorial");
    for (query, answer) in vec![("4", "24 "), ("6", "720 "), ("10", "3628800 ")] {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
        cmd.arg(&testcase_file).write_stdin(query);
        cmd.assert().stdout(answer);
    }

    Ok(())
}

#[test]
fn test_quine() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let filename = testcase("quine");
    let contents = fs::read_to_string(&filename)?;
    cmd.arg(filename);
    cmd.assert().success().stdout(contents);

    Ok(())
}
