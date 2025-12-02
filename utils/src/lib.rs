use clap::Parser;

pub mod input_helpers;
pub use input_helpers::{read_file, get_lines, split};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long = "puzzle_input")]
    pub puzzle_input: String,
}
