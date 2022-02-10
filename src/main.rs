use anyhow::{Context, Result};
use clap::Parser;

use program::{fingerprints::FPManager, Program};

mod args;
mod program;
mod repl;

fn main() -> Result<()> {
    let args = args::Args::parse();
    let mut fpmanager = FPManager::<'static, i32>::new();
    if args.file.is_none() && args.ucode.is_none() {
        repl::start::<i32>(&mut fpmanager)
    } else {
        let mut program = match args.file {
            Some(filepath) => Program::<i32>::try_from(filepath)?,
            None => {
                // unwrap is safe here
                // args.ucode is a "Some" for sure at this point
                let unefunge_code = args.ucode.unwrap().bytes().collect();
                Program::<i32>::from(vec![unefunge_code])
            }
        };
        let exit_code = program.run(&mut fpmanager).context("Runtime error")?;
        std::process::exit(exit_code);
    }
}
