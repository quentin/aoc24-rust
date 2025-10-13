use crate::etc::grid::Point;
use crate::{Grid, Solution, SolutionPair};
use itertools::Itertools;
use std::ops::Sub;

type Antennas = std::collections::HashMap<char, std::collections::HashSet<Point>>;

fn prepare(input: &str) -> (Grid<char>, Antennas) {
    let grid = Grid::new(input);
    let mut antennas: Antennas = Default::default();
    grid.for_each_with_position(|pos, &cell| {
        if cell != '.' {
            antennas.entry(cell).or_default().insert(pos);
        }
    });
    (grid, antennas)
}

fn solve_part1(input: &str) -> usize {
    let (grid, antennas) = prepare(input);
    let mut antinodes: std::collections::HashSet<Point> = Default::default();
    for (_, positions) in &antennas {
        for [a1, a2] in positions.iter().array_combinations() {
            if let Some(h1) = grid.step(a1, &(a1.sub(*a2))) {
                antinodes.insert(h1);
            }
            if let Some(h2) = grid.step(a2, &(a2.sub(*a1))) {
                antinodes.insert(h2);
            }
        }
    }
    antinodes.len()
}

fn solve_part2(input: &str) -> usize {
    let (grid, antennas) = prepare(input);
    let mut antinodes: std::collections::HashSet<Point> = Default::default();
    for (_, positions) in &antennas {
        for [a1, a2] in positions.iter().array_combinations() {
            let d = a1.sub(*a2);
            let gcd = num::integer::gcd(d.0, d.1);
            let d = Point(d.0 / gcd, d.1 / gcd);
            let mut h1 = Some(*a1);
            while let Some(h) = h1 {
                antinodes.insert(h.clone());
                h1 = grid.step(&h, &d);
            }
            let d = d.rotate_180();
            let mut h2 = Some(*a2);
            while let Some(h) = h2 {
                antinodes.insert(h.clone());
                h2 = grid.step(&h, &d);
            }
        }
    }
    antinodes.len()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 14);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 34);
    }
}
