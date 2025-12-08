use std::path::Path;
use std::cmp;

use clap::Parser;
use utils::{Args, UnionFind};

#[derive(Debug)]
struct Edge {
    weight: u64,
    idx_a: usize,
    idx_b: usize,
}

fn get_points(s: &str) -> (usize, usize, usize) {
    let points: Vec<usize> = s
        .splitn(3, ",")
        .map(|part| part.parse::<usize>().ok())
        .flatten()
        .collect();

    (points[0], points[1], points[2])
}

fn part_two(lines: &[String]) -> usize {
    let points: Vec<(usize, usize, usize)> = lines.iter().map(|line| get_points(&line)).collect();
    let n = points.len();
    let mut edges = vec![];
    let mut uf = UnionFind::new(n);

    for i in 0..n {
        for j in i + 1..n {
            let (xi, yi, zi) = points[i];
            let (xj, yj, zj) = points[j];

            let dx = (xi as i64) - (xj as i64);
            let dy = (yi as i64) - (yj as i64);
            let dz = (zi as i64) - (zj as i64);

            let dist_sq = (dx * dx + dy * dy + dz * dz) as u64;

            edges.push(Edge {
                weight: dist_sq,
                idx_a: i,
                idx_b: j,
            });
        }
    }

    edges.sort_unstable_by_key(|e| e.weight);

    for edge in &edges {
        uf.union(edge.idx_a, edge.idx_b);

        if uf.num_parts() == 1 {
            let xa = points[edge.idx_a].0;
            let xb = points[edge.idx_b].0;
            return xa * xb;
        }
    }
    0
}

fn part_one(lines: &[String], max_conns: usize) -> usize {
    let points: Vec<(usize, usize, usize)> = lines.iter().map(|line| get_points(&line)).collect();
    let n = points.len();
    let mut edges = vec![];
    let mut uf = UnionFind::new(n);

    for i in 0..n {
        for j in i + 1..n {
            let (xi, yi, zi) = points[i];
            let (xj, yj, zj) = points[j];

            let dx = (xi as i64) - (xj as i64);
            let dy = (yi as i64) - (yj as i64);
            let dz = (zi as i64) - (zj as i64);

            let dist_sq = (dx * dx + dy * dy + dz * dz) as u64;

            edges.push(Edge {
                weight: dist_sq,
                idx_a: i,
                idx_b: j,
            });
        }
    }

    edges.sort_unstable_by_key(|e| e.weight);

    let mut connections = 0;
    for edge in &edges {
        uf.union(edge.idx_a, edge.idx_b);
        connections += 1;

        if connections == max_conns {
            break;
        }
    }

    let mut sizes = uf.all_sizes();
    let size_len = sizes.len();
    sizes.sort_unstable();

    sizes[size_len - 1] * sizes[size_len - 2] * sizes[size_len - 3]
}

fn main() {
    let args = Args::parse();
    let puzzle_input =
        utils::read_file(Path::new(&args.puzzle_input)).expect("Failed to parse puzzle input");
    let lines = utils::get_lines(&puzzle_input);

    let part_one_result = part_one(&lines, 1000);
    let part_two_result = part_two(&lines);

    println!("Part One: {part_one_result}");
    println!("Part Two: {part_two_result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let puzzle_input = r#"162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689"#;

        let lines = utils::get_lines(&puzzle_input);
        let result = part_one(&lines, 10);

        assert_eq!(40, result);
    }

    #[test]
    fn test_part_two() {
        let puzzle_input = r#"162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689"#;

        let lines = utils::get_lines(&puzzle_input);
        let result = part_two(&lines);

        assert_eq!(25272, result);
    }
}
