use anyhow::{Context, Result};
use std::{io, io::prelude::*};

use crate::program::Program;

pub fn start() -> Result<()> {
    println!("{} - Unefunge 98 REPL", env!("CARGO_PKG_NAME"));
    println!("version {}", env!("CARGO_PKG_VERSION"));
    println!("(type \"exit\" or \"quit\" and press <Enter> or press <Ctrl> + C to quit)");

    loop {
        // step 1: print prompt
        print!("> ");
        io::stdout()
            .flush()
            .context("Failed to write an REPL prompt to stdout")?;

        // step 2: read code
        let mut code = match io::stdin().lock().lines().next() {
            Some(line) => line.context("Failed to read line from stdin")?,
            None => String::from(""),
        };

        // step 3: check for break
        if code == "exit" || code == "quit" {
            break;
        }

        // step 4: check empty line
        if code == "" {
            continue;
        }

        // step 5: add a "@" instruction at the end, for safety
        code.push('@');

        // step 5: evaluate code
        let mut program = Program::from(vec![code]);
        program.run()?;
        println!();
    }

    Ok(())
}
