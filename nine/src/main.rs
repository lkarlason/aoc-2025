use std::{path::Path, u64};

use clap::Parser;
use utils::{Args, Grid};

fn get_tuple(s: &str) -> Option<(usize, usize)> {
    s.split_once(",").and_then(|(ls, rs)| {
        let left = ls.parse().ok();
        let right = rs.parse().ok();

        left.zip(right)
    })
}

fn part_two(points: &[(usize, usize)]) -> u64 {
    let mut min_x = usize::MAX;
    let mut max_x: usize = 0;
    let mut min_y = usize::MAX;
    let mut max_y: usize = 0;

    for &(x, y) in points {
        if x > max_x {
            max_x = x;
        }
        if x < min_x {
            min_x = x;
        }

        if y > max_y {
            max_y = y;
        }
        if y < min_y {
            min_y = y;
        }
    }

    let mut grid = Grid::fill((max_y - min_y) + 1, (max_x - min_x) + 1, '.');
    for i in 0..points.len() {
        let next = if i == points.len() - 1 { 0 } else { i + 1 };

        let (x, y) = points[i];
        let (x_next, y_next) = points[next];

        let row_min = y.min(y_next);
        let row_max = y.max(y_next);
        let col_min = x.min(x_next);
        let col_max = x.max(x_next);

        for row in row_min..=row_max {
            for col in col_min..=col_max {
                grid.set(row - min_y, col - min_x, 'X');
            }
        }

        grid.set(y - min_y, x - min_x, '#');
        grid.set(y_next - min_y, x_next - min_x, '#');
    }

    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            print!("{}", grid.get(i, j).unwrap());
        }
        println!();
    }

    0
}

fn part_one(points: &[(usize, usize)]) -> u64 {
    let mut max_area = 0;
    for i in 0..points.len() {
        let (xi, yi) = points[i];
        for j in i + 1..points.len() {
            let (xj, yj) = points[j];

            let dx = ((xi as i64) - (xj as i64)).abs() + 1;
            let dy = (yi as i64) - (yj as i64).abs() + 1;
            let area = dx * dy;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area as u64
}

fn main() {
    let args = Args::parse();
    let puzzle_input =
        utils::read_file(Path::new(&args.puzzle_input)).expect("Failed to read puzzle input");
    let lines: Vec<(usize, usize)> = utils::get_lines(&puzzle_input)
        .iter()
        .map(|line| get_tuple(line))
        .flatten()
        .collect();

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
        let puzzle_input = r#"7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3"#;
        let lines: Vec<(usize, usize)> = utils::get_lines(&puzzle_input)
            .iter()
            .map(|line| get_tuple(line))
            .flatten()
            .collect();

        let result = part_one(&lines);

        assert_eq!(50, result);
    }

    #[test]
    fn test_part_two() {
        let puzzle_input = r#"7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3"#;
        let lines: Vec<(usize, usize)> = utils::get_lines(&puzzle_input)
            .iter()
            .map(|line| get_tuple(line))
            .flatten()
            .collect();

        let result = part_two(&lines);

        assert_eq!(24, result);
    }
}
