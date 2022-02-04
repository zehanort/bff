use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(
        name = "file",
        conflicts_with = "Unefunge program",
        help = "The path to the Unefunge/Befunge 98 source file"
    )]
    pub file: Option<PathBuf>,
    #[clap(
        name = "Unefunge program",
        short = 'u',
        conflicts_with = "file",
        help = "Run the Unefunge 98 program provided in the command line"
    )]
    pub ucode: Option<String>,
}
