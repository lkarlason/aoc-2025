use std::fs;
use std::io;
use std::path::Path;

use crate::grid::Grid;

pub fn read_file(path: &Path) -> io::Result<String> {
    Ok(fs::read_to_string(path)?)
}

pub fn split_whitespace(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(String::from)
        .collect()
}

pub fn get_uints(s: &str) -> Vec<u64> {
    s.split_whitespace()
        .map(|part| part.parse().ok())
        .flatten()
        .collect()
}

pub fn get_lines(s: &str) -> Vec<String> {
    s.lines().map(|raw| raw.trim().to_string()).collect()
}

pub fn split(s: &str, separator: &str) -> Vec<String> {
    s.split(separator).map(String::from).collect()
}

pub fn get_binary_grid(s: &str, one: char) -> Grid<u8> {
    let lines = get_lines(s);
    let rows = lines.len();
    let raw_grid: Vec<u8> = lines.iter()
        .map(|line| line.chars())
        .flatten()
        .map(|c| {
            if c == one {
                1
            } else {
                0
            }
        })
        .collect();

    Grid::new(raw_grid, rows)
}

pub fn get_raw_grid(s: &str) -> Grid<char> {
    let lines: Vec<String> = s.lines()
        .map(String::from)
        .collect();

     let chars = lines
        .iter()
        .map(|line| line.chars())
        .flatten()
        .collect();

    Grid::new(chars, lines.len())
}
