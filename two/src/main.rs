use std::io;
use std::path::Path;

use clap::Parser;

fn get_range(s: &str) -> Option<(u64, u64)> {
    let (low, high) = s.split_once('-')?;

    low.parse().ok().zip(high.parse().ok())
}

fn get_squares(low: u64, high: u64) -> Vec<u64> {
    let mut result = vec![];
    for d in low..high + 1 {
        let d_str = d.to_string();
        if d_str.len() % 2 != 0 {
            continue;
        }
        let mid = d_str.len() / 2;
        if d_str[0..mid] == d_str[mid..] {
            result.push(d);
        }
    }
    result
}

fn get_divisors(n: usize) -> Vec<usize> {
    println!("Calculating divisors for {n}");
    let mut divisors = vec![];
    for i in 1..=n / 2 {
        if n % i == 0 {
            divisors.push(i);
        }
    }

    divisors
}

fn get_repeated_string(s: &str, k: usize) -> String {
    let mut result = String::with_capacity(s.len() * k);
    for _ in 0..k {
        result.push_str(s);
    }

    result
}

fn get_periodic(low: u64, high: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut divisor_cache = std::collections::HashMap::new();
    for n in low..=high {
        let s = n.to_string();
        let s_len = s.len();
        let divisors = divisor_cache
            .entry(s_len)
            .or_insert_with(|| get_divisors(s_len));

        'check_periodic: for &mut div in divisors {
            let block = &s[..div];
            let factor = s_len / div;

            if get_repeated_string(block, factor) == s {
              result.push(n);
              break 'check_periodic;
            }
        }
    }
    result
}

fn part_one(lines: &[String]) -> Option<u64> {
    let mut count = 0;
    for line in lines {
        if let Some((low, high)) = get_range(line) {
            let squares = get_squares(low, high);
            count += squares.iter().sum::<u64>();
        }
    }
    Some(count)
}

fn part_two(lines: &[String]) -> Option<u64> {
    let mut count = 0;
    for line in lines {
        if let Some((low, high)) = get_range(line) {
            let squares = get_periodic(low, high);
            count += squares.iter().sum::<u64>();
        }
    }
    Some(count)
}

fn main() -> io::Result<()> {
    let args = utils::Args::parse();
    let puzzle_input = utils::read_file(Path::new(&args.puzzle_input))?;
    let lines = utils::split(&puzzle_input, ",");

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
        let puzzle_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let lines = utils::split(puzzle_input, ",");

        let result = part_one(&lines).unwrap();
        assert_eq!(1227775554, result);
    }

    #[test]
    fn test_part_two() {
        let puzzle_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let lines = utils::split(puzzle_input, ",");

        let result = part_two(&lines).unwrap();
        assert_eq!(4174379265, result);
    }
}
