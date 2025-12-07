use std::path::Path;

use clap::Parser;
use utils::{Args, Grid};

#[derive(Debug)]
enum Operation {
    ADD,
    MULT,
}

impl TryFrom<char> for Operation {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Operation::ADD),
            '*' => Ok(Operation::MULT),
            other => Err(format!("Invalid operation char: {}", other)),
        }
    }
}

impl Operation {
    fn execute(&self, a: u64, b: u64) -> u64 {
        match &self {
            Operation::ADD => a + b,
            Operation::MULT => a * b,
        }
    }
}

fn str_to_op(s: &str) -> Option<(Operation, u64)> {
    if s.len() != 1 {
        return None;
    }

    s.chars()
        .next()
        .and_then(|c| c.try_into().ok())
        .and_then(|op| match op {
            Operation::ADD => Some((op, 0)),
            Operation::MULT => Some((op, 1)),
        })
}

fn char_to_op(c: char) -> Option<(Operation, u64)> {
    c.try_into().ok().and_then(|op| match op {
        Operation::ADD => Some((op, 0)),
        Operation::MULT => Some((op, 1)),
    })
}

fn part_two(grid: &Grid<char>) -> u64 {
    let rows = grid.rows();
    let cols = grid.cols();
    let mut ops: Vec<(Operation, u64)> = grid
        .get_row(rows - 1)
        .iter()
        .filter(|c| !c.is_whitespace())
        .map(|&c| char_to_op(c))
        .flatten()
        .collect();

    let mut current_idx = 0;
    let mut sum = 0;

    for j in 0..cols {
        if current_idx > ops.len() - 1 {
            return 0;
        }

        let mut n = 0;
        for i in 0..rows - 1 {
            grid.get(i, j)
                .and_then(|c| c.to_digit(10))
                .map(|d| n = n * 10 + (d as u64));
        }

        let (op, prev) = &mut ops[current_idx];

        if n == 0 {
            sum += *prev;
            current_idx += 1;
        } else {
            *prev = op.execute(n, *prev);
            if j == cols - 1 {
                sum += *prev;
            }
        }
    }

    sum
}

fn part_one(lines: &[String]) -> u64 {
    let rows = lines.len();
    let mut ops: Vec<(Operation, u64)> = utils::split_whitespace(&lines[rows - 1])
        .iter()
        .map(|s| str_to_op(s))
        .flatten()
        .collect();

    for line in lines[0..rows - 1].iter() {
        let nums = utils::get_uints(line);
        if nums.len() != ops.len() {
            return 0;
        }

        for (idx, n) in nums.iter().enumerate() {
            let (op, prev) = &mut ops[idx];
            *prev = op.execute(*n, *prev);
        }
    }

    ops.iter().map(|(_, res)| *res).sum()
}

fn main() {
    let args = Args::parse();
    let puzzle_input =
        utils::read_file(Path::new(&args.puzzle_input)).expect("Failed to read input");
    let lines = utils::get_lines(&puzzle_input);
    let grid = utils::get_raw_grid(&puzzle_input);

    let part_one_result = part_one(&lines);
    let part_two_result = part_two(&grid);

    println!("Part One: {part_one_result}");
    println!("Part Two: {part_two_result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let puzzle_input = r#"123 328  51 64
        45 64  387 23
        6 98  215 314
        *   +   *   +"#;
        let lines = utils::get_lines(&puzzle_input);
        let result = part_one(&lines);

        assert_eq!(4277556, result);
    }

    #[test]
    fn test_part_two() {
        let puzzle_input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;
        let grid = utils::get_raw_grid(&puzzle_input);
        let result = part_two(&grid);

        assert_eq!(3263827, result);
    }
}
