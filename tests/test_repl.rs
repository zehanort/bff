use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

const PROMPT: &str = "> ";

#[test]
fn test_repl_core_behavior() -> Result<()> {
    let line1 = format!("{} - Unefunge 98 REPL", env!("CARGO_PKG_NAME"));
    let line2 = format!("version {}", env!("CARGO_PKG_VERSION"));
    let line3 =
        format!("(type \"exit\" or \"quit\" and press <Enter> or press <Ctrl> + C to quit)");

    for stop in ["exit", "quit"] {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
        cmd.write_stdin(stop)
            .assert()
            .success()
            .stdout(format!("{}\n{}\n{}\n{}", line1, line2, line3, PROMPT));
    }

    Ok(())
}

#[test]
fn test_simple_program() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    // no stop checked here, too
    cmd.write_stdin("27*.\nexit")
        .assert()
        .success()
        .stdout(predicate::str::ends_with(format!("14 \n{}", PROMPT)));

    Ok(())
}

#[test]
fn test_empty_line() -> Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin("\nexit")
        .assert()
        .success()
        .stdout(predicate::str::ends_with(format!("{}{}", PROMPT, PROMPT)));

    Ok(())
}
