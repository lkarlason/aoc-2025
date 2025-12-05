use std::{path::Path, vec};

use clap::Parser;
use utils::Args;

fn get_tuple(s: &str) -> Option<(usize, usize)> {
    s.split_once('-').and_then(|(low, high)| {
        let low = low.parse().ok();
        let high = high.parse().ok();

        low.zip(high)
    })
}

fn merge_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if ranges.is_empty() {
        return vec![];
    }

    ranges.sort_by_key(|range| range.0);
    let mut merged = vec![];
    let mut current = ranges[0];

    for range in ranges.into_iter().skip(1) {
        if range.0 <= current.1 {
            current.1 = current.1.max(range.1);
        } else {
            merged.push(current);
            current = range;
        }
    }

    merged.push(current);

    merged
}

fn part_two(lines: &[String]) -> u64 {
    let mut ranges = vec![];
    for line in lines {
        if line.is_empty() {
            break;
        }

        match get_tuple(&line) {
            Some(r) => ranges.push(r),
            None => continue,
        };
    }

    let ranges = merge_ranges(ranges);

    let mut valid = 0;
    for (low, high) in ranges {
        valid += (high - low + 1) as u64;
    }

    valid
}

fn part_one(lines: &[String]) -> u64 {
    let mut valid = 0;
    let mut ranges = vec![];
    let mut id_start_idx = 0;

    for (idx, line) in lines.iter().enumerate() {
        if line.is_empty() {
            id_start_idx = idx + 1;
            break;
        }

        if let Some(range) = get_tuple(&line) {
            ranges.push(range);
        }
    }

    for line in &lines[id_start_idx..] {
        let id: usize = match line.parse().ok() {
            Some(n) => n,
            _ => continue,
        };

        'check: for &(low, high) in &ranges {
            if id >= low && id <= high {
                valid += 1;
                break 'check;
            }
        }
    }

    valid
}

fn main() {
    let args = Args::parse();
    let puzzle_input =
        utils::read_file(Path::new(&args.puzzle_input)).expect("Failed to read puzzle input");
    let lines = utils::get_lines(&puzzle_input);

    let part_one_result = part_one(&lines);
    let part_two_result = part_two(&lines);

    println!("Part One: {part_one_result}");
    println!("Part Two: {part_two_result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let puzzle_input = r#"3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32"#;
        let lines = utils::get_lines(&puzzle_input);
        println!("{lines:?}");

        let result = part_one(&lines);

        assert_eq!(3, result);
    }

    #[test]
    fn test_part_two() {
        let puzzle_input = r#"3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32"#;

        let lines = utils::get_lines(&puzzle_input);

        let result = part_two(&lines);

        assert_eq!(14, result);
    }
}
