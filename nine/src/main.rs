use std::{path::Path, u64};

use clap::Parser;
use utils::Args;

fn get_tuple(s: &str) -> Option<(usize, usize)> {
    s.split_once(",").and_then(|(ls, rs)| {
        let left = ls.parse().ok();
        let right = rs.parse().ok();

        left.zip(right)
    })
}

fn is_contained(points: &[(usize, usize)], point: (usize, usize)) -> bool {
    let n = points.len();

    // Check edge
    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];

        if y1 == y2 && point.1 == y1 {
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            if point.0 >= min_x && point.0 <= max_x {
                return true;
            }
        }

        if x1 == x2 && point.0 == x1 {
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);
            if point.1 >= min_y && point.1 <= max_y {
                return true;
            }
        }
    }

    let mut crossings = 0;
    // Check inside
    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];

        if x1 != x2 {
            continue;
        }
        let x = x1;

        let y_min = y1.min(y2);
        let y_max = y1.max(y2);

        if point.1 > y_min && point.1 <= y_max && point.0 < x {
            crossings += 1;
        }
    }

    crossings % 2 == 1
}

fn rect_cuts_boundary(
    points: &[(usize, usize)],
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
) -> bool {
    let n = points.len();

    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];

        if x1 == x2 {
            let x = x1;
            let (ylow, yhigh) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

            if x > xmin && x < xmax {
                if ylow < ymin && yhigh > ymin {
                    return true;
                }
                if ylow < ymax && yhigh > ymax {
                    return true;
                }
            }
        } else {
            let y = y1;
            let (xlow, xhigh) = if x1 < x2 { (x1, x2) } else { (x2, x1) };

            if y > ymin && y < ymax {
                if xlow < xmin && xhigh > xmin {
                    return true;
                }
                if xlow < xmax && xhigh > xmax {
                    return true;
                }
            }
        }
    }

    false
}

fn rectangle_contained(points: &[(usize, usize)], p1: (usize, usize), p2: (usize, usize)) -> bool {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let xmin = x1.min(x2);
    let xmax = x1.max(x2);
    let ymin = y1.min(y2);
    let ymax = y1.max(y2);

    let corners = [(xmin, ymin), (xmin, ymax), (xmax, ymin), (xmax, ymax)];

    if !corners.iter().all(|p| is_contained(&points, *p)) {
        return false;
    };

    if rect_cuts_boundary(points, xmin, xmax, ymin, ymax) {
        return false;
    }

    true
}

fn part_two(points: &[(usize, usize)]) -> u64 {
    let mut max_area = 0;
    let n = points.len();
    for i in 0..n {
        for j in i + 1..n {
            let (xi, yi) = points[i];
            let (xj, yj) = points[j % n];

            let dx = ((xi as i64) - (xj as i64)).abs() as usize + 1;
            let dy = ((yi as i64) - (yj as i64)).abs() as usize + 1;
            let area = dx * dy;

            if rectangle_contained(&points, (xi, yi), (xj, yj)) {
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    max_area as u64
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
