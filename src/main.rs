use anyhow::Result;
use clap::Parser;

#[macro_use]
extern crate colour;

mod args;
mod cursor;
mod delta;
mod program;

macro_rules! warning {
    ($message:expr) => {
        e_yellow!("warning: ");
        eprintln!($message);
    };
}

fn main() -> Result<()> {
    let args = args::Args::parse();
    if args.file.is_none() && args.ucode.is_none() {
        warning!("No execution targets specified. Exiting.");
        return Ok(());
    }

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
