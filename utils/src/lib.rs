use clap::Parser;

pub mod grid;
pub mod input_helpers;
pub use grid::Grid;
pub use input_helpers::{
    get_binary_grid, get_lines, get_raw_grid, get_uints, read_file, split, split_whitespace,
};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long = "puzzle_input")]
    pub puzzle_input: String,
}
