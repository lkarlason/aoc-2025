use core::num;
use std::{
    collections::{HashMap, VecDeque}, f32::consts::E, path::Path
};

use clap::Parser;
use regex::Regex;
use utils::{Args, Matrix};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct State([usize; 16]);

fn get_target_state(s: &str) -> (u16, usize) {
    let re_bracket = Regex::new(r"\[(?P<brackets>[^\]]+)\]").unwrap();

    let caps = re_bracket.captures(s).unwrap();

    let target_bits: Vec<u8> = caps
        .name("brackets")
        .unwrap()
        .as_str()
        .chars()
        .map(|c| match c {
            '#' => 1,
            _ => 0,
        })
        .collect();

    let mut target = 0;
    for (i, &bit) in target_bits.iter().enumerate() {
        if bit == 1 {
            target |= 1 << i;
        }
    }
    (target, target_bits.len())
}

fn get_buttons(s: &str) -> Vec<u16> {
    let re_parenthesis = Regex::new(r"\(([^)]+)\)").unwrap();

    re_parenthesis
        .captures_iter(s)
        .map(|cap| cap.get(1).unwrap().as_str())
        .map(|s| {
            s.split(',')
                .map(|el| el.parse().ok())
                .flatten()
                .collect::<Vec<usize>>()
        })
        .map(|bits| {
            let mut mask = 0;
            for i in bits {
                mask |= 1 << i;
            }
            mask
        })
        .collect()
}

fn get_buttons_matrix(s: &str, n: usize) -> Matrix {
    let re_parenthesis = Regex::new(r"\(([^)]+)\)").unwrap();

    let btns: Vec<Vec<usize>> = re_parenthesis
        .captures_iter(s)
        .map(|cap| cap.get(1).unwrap().as_str())
        .map(|s| {
            s.split(',')
                .map(|el| el.parse().ok())
                .flatten()
                .collect::<Vec<usize>>()
        })
        .collect();

    let m = btns.len();
    let mut bin_btns = vec![vec![0f64; m]; n];
    for (btn_idx, btn) in btns.iter().enumerate() {
        for &idx in btn {
            bin_btns[idx][btn_idx] = 1.0;
        }
    }

    println!("{bin_btns:?}");
    Matrix::new(bin_btns)
}

fn get_target_joltage(s: &str) -> Vec<usize> {
    let re_braces = Regex::new(r"\{(?P<braces>[^\}]+)\}").unwrap();

    let caps = re_braces.captures(s).unwrap();

    caps.name("braces")
        .unwrap()
        .as_str()
        .split(',')
        .map(|part| part.parse())
        .flatten()
        .collect()
}

fn get_min_buttons(buttons: &[u16], target: u16, num_bits: u8) -> Option<Vec<u16>> {
    if num_bits > 16 {
        panic!("Maximum number of indicators is 16");
    }

    let mut visited = vec![false; 1 << num_bits];
    let mut parent = vec![None::<(u16, u16)>; 1 << num_bits];

    let mut queue = VecDeque::new();
    queue.push_back(0u16);
    visited[0] = true;

    while let Some(state) = queue.pop_front() {
        if state == target {
            let mut path = vec![];
            let mut current = state;
            while let Some((prev, button)) = parent[current as usize] {
                path.push(button);
                current = prev;
            }

            return Some(path);
        }

        for &button in buttons {
            let next = state ^ button;
            let idx = next as usize;
            if !visited[idx] {
                visited[idx] = true;
                parent[idx] = Some((state, button));
                queue.push_back(next);
            }
        }
    }

    None
}

fn part_two(lines: &[String]) -> usize {
    let mut res = 0;
    for line in lines {
        let target: Vec<f64> = get_target_joltage(line)
            .iter()
            .map(|&el| el as f64)
            .collect();
        let buttons = get_buttons_matrix(line, target.len());

        let echelon = buttons.reduced_echelon();
        let solution = buttons.gauss_elim(&target)
            .unwrap();
        let answer = buttons.vec_mult(&solution);

        println!("Solution: {solution:?}");
        println!("Result: {answer:?}");

        let n = buttons.rows();
        let m = buttons.cols();

        for i in 0..n {
            for j in 0..m {
                print!(" {}", echelon[i][j]);
            }
            println!();
        }

        break;
    }

    res
}

fn part_one(lines: &[String]) -> usize {
    let mut res = 0;
    for line in lines {
        let (target, num_bits) = get_target_state(line);
        let buttons = get_buttons(line);

        let min_buttons = get_min_buttons(&buttons, target, num_bits as u8);
        res += min_buttons.map(|btns| btns.len()).unwrap_or(0);
    }

    res
}

fn main() {
    let args = Args::parse();
    let puzzle_input =
        utils::read_file(Path::new(&args.puzzle_input)).expect("Failed to read input");
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
        let puzzle_input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
        let lines = utils::get_lines(&puzzle_input);

        let result = part_one(&lines);

        assert_eq!(7, result);
    }

    #[test]
    fn test_part_two() {
        let puzzle_input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
        let lines = utils::get_lines(&puzzle_input);

        let result = part_two(&lines);

        assert_eq!(33, result);
    }
}
