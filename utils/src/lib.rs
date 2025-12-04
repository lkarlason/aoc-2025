use clap::Parser;

pub mod grid;
pub mod input_helpers;
pub use grid::Grid;
pub use input_helpers::{get_binary_grid, get_lines, read_file, split};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long = "puzzle_input")]
    pub puzzle_input: String,
}
