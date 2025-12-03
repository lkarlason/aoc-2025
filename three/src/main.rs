use std::path::Path;

use clap::Parser;
use utils::Args;

fn find_max_tuple(numbers: &[u64]) -> u64 {
    let mut first = 0;
    let mut first_idx = 0;
    let mut second = 0;
    let size = numbers.len();

    for (idx, &n) in numbers[..size - 1].iter().enumerate() {
        if n > first {
            first = n;
            first_idx = idx;
        }
    }

    for &n in &numbers[first_idx + 1..] {
        if n > second {
            second = n;
        }
    }

    first * 10 + second
}

fn find_max_combination<const T: usize>(numbers: &[u64]) -> [u64; T] {
    let mut combination: [u64; T] = [0; T];
    let mut first_index = 0;

    for i in (0..T).rev() {
        let last_index = numbers.len() - (i * 1);
        let mut max = 0;
        let mut max_idx = first_index;
        for (idx, &n) in numbers[max_idx..last_index].iter().enumerate() {
            if n > max {
                max = n;
                max_idx = idx;
            }
        }
        first_index += max_idx + 1;
        combination[T - (i + 1)] = max;
    }

    combination
}

fn get_numbers(line: &String) -> Vec<u64> {
    line.chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).map(|d| d as u64))
        .flatten()
        .collect()
}

fn get_decimal_repr<const T: usize>(digits: &[u64; T]) -> u64 {
    let mut decimal = 0;
    for (idx, d) in digits.iter().enumerate() {
      let exp = (T - (idx + 1)) as u32;
      decimal += d * (10 as u64).pow(exp);
    }

    decimal
}

fn part_one(lines: &[String]) -> u64 {
    let mut sum = 0;
    for line in lines {
        let numbers = get_numbers(line);
        sum += find_max_tuple(&numbers);
    }

    sum
}

fn part_two(lines: &[String]) -> u64 {
    let mut sum = 0;
    for line in lines {
        let numbers = get_numbers(line);
        let combination = find_max_combination::<12>(&numbers);
        sum += get_decimal_repr(&combination);
    }

    sum
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
        let puzzle_input = r#"987654321111111
          811111111111119
          234234234234278
          818181911112111"#;

        let lines = utils::get_lines(&puzzle_input);
        let result = part_one(&lines);

        assert_eq!(357, result);
    }

    #[test]
    fn test_part_two() {
        let puzzle_input = r#"987654321111111
          811111111111119
          234234234234278
          818181911112111"#;

        let lines = utils::get_lines(&puzzle_input);
        let result = part_two(&lines);

        assert_eq!(3121910778619, result);
    }
}
