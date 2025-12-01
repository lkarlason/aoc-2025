use clap::Parser;
use std::io;
use std::path::Path;
use utils;

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::LEFT),
            'R' => Ok(Direction::RIGHT),
            other => Err(format!("Invalid direction char: {}", other)),
        }
    }
}

fn parse_rotation(rotation: &str) -> Option<(Direction, i64)> {
    rotation.split_at_checked(1).and_then(|(d, n)| {
        let direction: Option<Direction> = d.chars().next().and_then(|c| c.try_into().ok());
        let clicks: Option<i64> = n.parse().ok();

        direction.zip(clicks)
    })
}

fn part_one(lines: &Vec<String>) -> Option<u64> {
    let mut dial: i64 = 50;
    let mut count = 0;
    for line in lines {
        dial = if let Some((direction, clicks)) = parse_rotation(line) {
            match direction {
                Direction::LEFT => (dial - clicks).rem_euclid(100),
                Direction::RIGHT => (dial + clicks).rem_euclid(100),
            }
        } else {
            return None;
        };

        if dial == 0 {
            count += 1;
        }
    }
    Some(count)
}

fn part_two(lines: &Vec<String>) -> Option<u64> {
    let mut dial = 50;
    let mut count = 0;

    for line in lines {
        let new_dial = if let Some((direction, clicks)) = parse_rotation(line) {
            match direction {
                Direction::LEFT => dial - clicks,
                Direction::RIGHT => dial + clicks,
            }
        } else {
            return None;
        };

        if new_dial <= 0 && dial != 0 {
            count += (-(new_dial / 100) + 1) as u64;
        } else if new_dial <= 0 {
            count += -(new_dial / 100) as u64;
        } else if new_dial >= 100 {
            count += (new_dial / 100) as u64;
        }

        dial = new_dial.rem_euclid(100);
    }

    Some(count)
}

fn main() -> io::Result<()> {
    let args = utils::Args::parse();

    let file_content = utils::read_file(Path::new(&args.puzzle_input))?;
    let lines = utils::get_lines(&file_content);

    let part_one_result = part_one(&lines).expect("Could not calculate answer for part one");
    let part_two_result = part_two(&lines).expect("Could not calculate answer for part two");

    println!("Part one: {part_one_result}");
    println!("Part two: {part_two_result}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let puzzle_input = r#"L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82"#;
        let lines = utils::get_lines(puzzle_input);

        let result = part_one(&lines).unwrap();

        assert_eq!(3, result);
    }

    #[test]
    fn test_part_two() {
        let puzzle_input = r#"L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82"#;
        let lines = utils::get_lines(puzzle_input);
        let result = part_two(&lines).unwrap();

        assert_eq!(6, result);
    }
}
