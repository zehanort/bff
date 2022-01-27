use anyhow::{Context, Result};
use std::{io, io::prelude::*};

use crate::program::Program;

pub fn start(size: i32) -> Result<()> {
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
        if code.is_empty() {
            continue;
        }

        // step 5: add a "@" instruction at the end, for safety
        code.push('@');

        // step 6: evaluate code
        match size {
            1 => {
                let mut program = Program::<i8>::from(vec![code.into_bytes()]);
                program.run()?;
            }
            2 => {
                let mut program = Program::<i16>::from(vec![code.into_bytes()]);
                program.run()?;
            }
            8 => {
                let mut program = Program::<i64>::from(vec![code.into_bytes()]);
                program.run()?;
            }
            16 => {
                let mut program = Program::<i128>::from(vec![code.into_bytes()]);
                program.run()?;
            }
            _ => {
                // default is i32
                let mut program = Program::<i32>::from(vec![code.into_bytes()]);
                program.run()?;
            }
        }

        println!();
    }

    Ok(())
}
