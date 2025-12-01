use std::fs;
use std::io;
use std::path::Path;

pub fn read_file(path: &Path) -> io::Result<String> {
    Ok(fs::read_to_string(path)?)
}

pub fn get_lines(s: &str) -> Vec<String> {
    s.lines().map(|raw| raw.trim().to_string()).collect()
}
