use crate::etc::grid::{Displacement, Position};
use crate::{Grid, Solution, SolutionPair};

#[derive(Debug, Clone, PartialEq, Default)]
enum Cell {
    #[default]
    Empty,
    Obstruction,
}

type Map = Grid<Cell>;

fn prepare(input: &str) -> (Map, Position) {
    let grid = Grid::new(input);
    (
        grid.new_from(|x| match x {
            '.' | '^' => Cell::Empty,
            '#' => Cell::Obstruction,
            _ => unreachable!(),
        }),
        grid.position(|&x| x == '^').unwrap(),
    )
}

/// Execute the guard's patrol, return the set of positions visited by the guard
/// and whether the patrol is a loop.
fn patrol(map: &Map, mut guard: Position) -> (std::collections::BTreeSet<Position>, bool) {
    let mut direction = Displacement::NORTH;
    let mut patrolled = std::collections::BTreeSet::new();
    let mut locations = std::collections::BTreeSet::new();
    loop {
        if !patrolled.insert((guard, direction)) {
            return (locations, true);
        }

        locations.insert(guard);

        if let Some(ahead) = map.step(&guard, &direction) {
            guard = match map.get(&ahead).unwrap() {
                Cell::Empty => ahead,
                Cell::Obstruction => {
                    direction = direction.turn_right();
                    guard
                }
            };
        } else {
            return (locations, false);
        }
    }
}

fn solve_part1(input: &str) -> usize {
    let (map, guard) = prepare(input);
    patrol(&map, guard).0.len()
}

fn solve_part2(input: &str) -> usize {
    let (mut map, guard) = prepare(input);
    let mut positions = patrol(&map, guard).0;
    positions.remove(&guard);
    positions.iter().filter(|obstruction| {
        *map.get_mut(&obstruction).unwrap() = Cell::Obstruction;
        let is_loop = patrol(&map, guard).1;
        *map.get_mut(&obstruction).unwrap() = Cell::Empty;
        is_loop
    }).count()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 41);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn preparation() {
        let (map, guard) = prepare(EXAMPLE_INPUT);
        assert_eq!(guard, Position(6, 4));
        assert_eq!(map.at(0, 0), Some(&Cell::Empty));
        assert_eq!(map.at(3, 2), Some(&Cell::Obstruction));
        assert_eq!(map.at(6, 4), Some(&Cell::Empty));
    }
}
