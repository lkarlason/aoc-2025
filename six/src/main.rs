use std::path::Path;

use clap::Parser;
use utils::Args;

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

fn get_op_and_init(s: &str) -> Option<(Operation, u64)> {
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

fn part_one(lines: &[String]) -> u64 {
    let rows = lines.len();
    let ops: Vec<(Operation, u64)> = utils::split_whitespace(&lines[rows - 1])
        .iter()
        .map(|s| get_op_and_init(s))
        .flatten()
        .collect();

    for line in lines[0..rows - 1].iter() {
        let nums = utils::get_uints(line);
        if nums.len() != ops.len() {
            return 0;
        }

        for (idx, n) in nums.iter().enumerate() {
            let (op, prev) = &mut ops[idx];
            *prev = op.execute(n, *prev);
    }
    0
}

fn main() {
    let args = Args::parse();
    let puzzle_input =
        utils::read_file(Path::new(&args.puzzle_input)).expect("Failed to read input");
    let lines = utils::get_lines(&puzzle_input);

    let part_one_result = part_one(&lines);

    println!("Part One: {part_one_result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let puzzle_input = r#"123 328  51 64 
         45 64  387 23 
          6 98  215 314
        *   +   *   +  "#;
        let lines = utils::get_lines(&puzzle_input);
        let result = part_one(&lines);

        assert_eq!(4277556, result);
    }
}
