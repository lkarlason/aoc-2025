use std::{collections::HashSet, path::Path};

use clap::Parser;
use utils::{Args, Grid};

fn part_two(grid: &Grid<char>) -> u64 {
    let rows = grid.rows();
    let cols = grid.cols();

    let mut timelines = vec![vec![0u64; cols]; rows];

    for col in 0..cols {
        timelines[rows - 1][col] = 1;
    }

    for row in (0..rows - 1).rev() {
        for col in 0..cols {
            let down = (row + 1, col);
            match grid.get(down.0, down.1) {
                Some('^') => {
                    let left = if col > 0 {
                        timelines[row + 1][col - 1]
                    } else {
                        0
                    };

                    let right = if col + 1 < cols {
                        timelines[row + 1][col + 1]
                    } else {
                        0
                    };

                    timelines[row][col] = left + right;
                }

                Some('.') => {
                    timelines[row][col] = timelines[row + 1][col];
                }

                Some(_) | None => {}
            }
        }
    }

    let (start_row, start_col) = grid.first_pos_of('S').unwrap();

    timelines[start_row][start_col]
}

fn part_one(grid: &Grid<char>) -> u64 {
    let start_pos = match grid.first_pos_of('S') {
        Some(pos) => pos,
        None => return 0,
    };

    let mut beams = vec![start_pos];
    let mut seen = HashSet::new();
    seen.insert(start_pos);
    let mut res = 0;

    while let Some((row, col)) = beams.pop() {
        if row + 1 >= grid.rows() {
            continue;
        }
        let down = (row + 1, col);

        match grid.get(down.0, down.1) {
            Some('^') => {
                let left = (row + 1, col.wrapping_sub(1));
                let right = (row + 1, col + 1);
                let mut splitted = false;

                if col > 0 && seen.insert(left) {
                    beams.push(left);
                    splitted = true;
                }

                if col + 1 < grid.cols() && seen.insert(right) {
                    beams.push(right);
                    splitted = true;
                }

                if splitted {
                    res += 1;
                }
            }

            Some('.') => {
                if seen.insert(down) {
                    beams.push(down);
                }
            }

            Some(_) | None => {}
        }
    }
    res
}

fn main() {
    let args = Args::parse();
    let puzzle_input =
        utils::read_file(Path::new(&args.puzzle_input)).expect("Failed to read puzzle input");
    let grid = utils::get_raw_grid(&puzzle_input);

    let part_one_result = part_one(&grid);
    let part_two_result = part_two(&grid);

    println!("Part One: {part_one_result}");
    println!("Part Two: {part_two_result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let puzzle_input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

        let grid = utils::get_raw_grid(&puzzle_input);
        let result = part_one(&grid);

        assert_eq!(21, result);
    }
    #[test]
    fn test_part_two() {
        let puzzle_input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

        let grid = utils::get_raw_grid(&puzzle_input);
        let result = part_two(&grid);

        assert_eq!(40, result);
    }
}
