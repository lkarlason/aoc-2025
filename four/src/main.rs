use std::path::Path;

use clap::Parser;
use utils::{Args, Grid};

fn get_removable(grid: &Grid<u8>) -> Vec<(usize, usize)> {
    let mut indices = vec![];

    let rows = grid.rows();
    let cols = grid.cols();
    for i in 0..rows {
        for j in 0..cols {
            let elem = grid.get(i, j).unwrap_or(0) as u64;
            if elem == 0 {
                continue;
            }

            // No need to check corners
            if (i == 0 && j == 0)
                || (i == rows - 1 && j == 0)
                || (i == 0 && j == cols - 1)
                || (i == rows - 1 && j == cols - 1)
            {
                indices.push((i, j));
                continue;
            }

            let grid_range = if j == 0 {
                grid.get_grid_range((i - 1, i + 1), (j, j + 1))
            } else if j == cols - 1 {
                grid.get_grid_range((i - 1, i + 1), (j - 1, j))
            } else if i == 0 {
                grid.get_grid_range((i, i + 1), (j - 1, j + 1))
            } else if i == rows - 1 {
                grid.get_grid_range((i - 1, i), (j - 1, j + 1))
            } else {
                grid.get_grid_range((i - 1, i + 1), (j - 1, j + 1))
            };

            let adjacent: u8 = grid_range.iter().sum();

            if adjacent - 1 < 4 {
                indices.push((i, j));
            }
        }
    }

    indices
}

fn part_two(grid: &mut Grid<u8>) -> u64 {
    let mut sum = 0;
    loop {
      let removable = get_removable(grid);
      if removable.len() == 0 {
        break;
      }
      sum += removable.len() as u64;
      for (i, j) in removable {
        grid.set(i, j, 0);
      }
    }
    sum
}

fn part_one(grid: &Grid<u8>) -> u64 {
    get_removable(grid).len() as u64
}

fn main() {
    let args = Args::parse();
    let puzzle_input =
        utils::read_file(Path::new(&args.puzzle_input)).expect("Failed to read puzzle input");
    let mut grid = utils::get_binary_grid(&puzzle_input, '@');

    let part_one_result = part_one(&grid);
    let part_two_result = part_two(&mut grid);
    println!("Part One: {part_one_result}");
    println!("Part Two: {part_two_result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let puzzle_input = r#"..@@.@@@@.
          @@@.@.@.@@
          @@@@@.@.@@
          @.@@@@..@.
          @@.@@@@.@@
          .@@@@@@@.@
          .@.@.@.@@@
          @.@@@.@@@@
          .@@@@@@@@.
          @.@.@@@.@."#;
        let grid = utils::get_binary_grid(puzzle_input, '@');
        let result = part_one(&grid);

        assert_eq!(13, result);
    }

    #[test]
    fn test_part_two() {
        let puzzle_input = r#"..@@.@@@@.
          @@@.@.@.@@
          @@@@@.@.@@
          @.@@@@..@.
          @@.@@@@.@@
          .@@@@@@@.@
          .@.@.@.@@@
          @.@@@.@@@@
          .@@@@@@@@.
          @.@.@@@.@."#;
        let mut grid = utils::get_binary_grid(puzzle_input, '@');
        let result = part_two(&mut grid);

        assert_eq!(43, result);
    }
}
