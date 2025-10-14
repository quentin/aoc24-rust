use crate::etc::grid::{ALL_DIRECTIONS, Point};
use crate::{Grid, Solution, SolutionPair};

fn prepare(input: &str) -> Grid {
    Grid::new(input)
}

fn solve_part1(input: &str) -> usize {
    let grid = prepare(input);
    let mut count = 0;
    for l in 0..(grid.lines as i64){
        for c in 0..(grid.columns as i64) {
            for step in &ALL_DIRECTIONS {
                if let Some(['X', 'M', 'A', 'S']) = grid.step_extract(&Point(l, c), step) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn solve_part2(input: &str) -> usize {
    let grid = prepare(input);
    let mut count = 0;
    for l in 0..(grid.lines as i64) {
        for c in 0..(grid.columns as i64) {
            // check the first diagonal in both directions
            let center = Point(l, c);
            let diag1 = (Some(['M', 'A', 'S'])
                == grid.deltas_extract(&center, [(-1, -1), (0, 0), (1, 1)].map(Point::from)))
                || (Some(['M', 'A', 'S'])
                    == grid.deltas_extract(&center, [(1, 1), (0, 0), (-1, -1)].map(Point::from)));
            // check the second diagonal in both directions
            let diag2 = (Some(['M', 'A', 'S'])
                == grid.deltas_extract(&center, [(1, -1), (0, 0),(-1, 1)].map(Point::from)))
                || (Some(['M', 'A', 'S'])
                    == grid
                        .deltas_extract(&center, [(-1, 1), (0, 0), (1, -1)].map(Point::from)));
            if diag1 && diag2 {
                count += 1;
            }
        }
    }
    count
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "MMMSXXMASM
  MSAMXMSMSA
  AMXSXMAAMM
  MSAMASMSMX
  XMASAMXAMM
  XXAMMXXAMA
  SMSMSASXSS
  SAXAMASAAA
  MAMMMXMMMM
  MXMXAXMASX";

    #[test]
    fn test_prepare() {
        let grid = prepare(EXAMPLE_INPUT);
        assert_eq!(grid.lines, 10);
        assert_eq!(grid.columns, 10);
        assert_eq!(grid.items[0], 'M');
        assert_eq!(grid.items[10], 'M');
        assert_eq!(grid.items[11], 'S');
        assert_eq!(grid.items[20], 'A')
    }

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 18);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 9);
    }
}
