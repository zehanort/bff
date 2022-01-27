use anyhow::Result;
use assert_cmd::Command;
use std::str;

const TESTFILE: &str = "tests/bf93/factorial.bf";

#[test]
fn test_file_all_types() -> Result<()> {
    for (t, s, f) in [
        ("1", ("5", "120 "), "6"),
        ("2", ("7", "5040 "), "8"),
        ("4", ("12", "479001600 "), "13"),
        ("8", ("20", "2432902008176640000 "), "21"),
        ("16", ("33", "8683317618811886495518194401280000000 "), "34"),
    ] {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
        cmd.arg(TESTFILE)
            .arg("-s")
            .arg(t)
            .write_stdin(s.0)
            .assert()
            .success()
            .stdout(s.1);

        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
        cmd.arg(TESTFILE)
            .arg("-s")
            .arg(t)
            .write_stdin(f)
            .assert()
            .success()
            .stderr(predicates::str::contains(
                "A multiplication resulted in overflow.",
            ));
    }

    Ok(())
}

// TODO: test repl with something like factorial,
//       but in Unefunge
//
// #[test]
// fn test_repl_all_types() -> Result<()> {
//     for (t, s, f) in [
//         ("1", ("5", "120 "), "6"),
//         ("2", ("7", "5040 "), "8"),
//         ("4", ("12", "479001600 "), "13"),
//         ("8", ("20", "2432902008176640000 "), "21"),
//         ("16", ("33", "8683317618811886495518194401280000000 "), "34"),
//     ] {
//         let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
//         cmd.arg("-s")
//             .arg(t)
//             .write_stdin(s.0)
//             .assert()
//             .success()
//             .stdout(predicates::str::ends_with(format!(
//                 "{}{} \n{}",
//                 PROMPT, s.1, PROMPT
//             )));

//         let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
//         cmd.arg("-s")
//             .arg(t)
//             .write_stdin(f)
//             .assert()
//             .success()
//             .stderr(predicates::str::contains(
//                 "A multiplication resulted in overflow.",
//             ));
//     }

//     Ok(())
// }
