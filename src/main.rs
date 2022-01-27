use anyhow::Result;
use clap::Parser;

use runner::Runner;

mod args;
mod program;
mod repl;
mod runner;

fn main() -> Result<()> {
    let args = args::Args::parse();
    if args.file.is_none() && args.ucode.is_none() {
        repl::start(args.size)
    } else {
        match args.size {
            1 => {
                let mut runner = Runner::<i8>::try_from((args.file, args.ucode))?;
                runner.run()
            }
            2 => {
                let mut runner = Runner::<i16>::try_from((args.file, args.ucode))?;
                runner.run()
            }
            8 => {
                let mut runner = Runner::<i64>::try_from((args.file, args.ucode))?;
                runner.run()
            }
            16 => {
                let mut runner = Runner::<i128>::try_from((args.file, args.ucode))?;
                runner.run()
            }
            _ => {
                // default is i32
                let mut runner = Runner::<i32>::try_from((args.file, args.ucode))?;
                runner.run()
            }
        }
    }
}
