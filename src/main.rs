use anyhow::Result;
use clap::Parser;

#[macro_use]
extern crate colour;

mod args;
mod commands;
mod cursor;
mod delta;
mod program;
mod repl;

fn main() -> Result<()> {
    let args = args::Args::parse();
    if args.file.is_none() && args.ucode.is_none() {
        repl::start()
    } else {
        let mut program = match args.file {
            Some(filepath) => program::Program::try_from(filepath)?,
            None => {
                // unwrap is safe here
                // args.ucode is a "Some" for sure at this point
                let unefunge_code = args.ucode.unwrap();
                program::Program::from(vec![unefunge_code])
            }
        };

        program.run()
    }
}
