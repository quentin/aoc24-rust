use crate::{Grid, Point, Solution, SolutionPair};

#[derive(Debug, Clone, PartialEq, Default)]
enum Cell {
    #[default]
    Empty,
    Obstruction,
}

type Map = Grid<Cell>;

fn prepare(input: &str) -> (Map, Point) {
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

mod slow {
    //! Simple but slow implementation
    #![allow(dead_code)]
    use super::*;

    /// Execute the guard's patrol, return the set of positions visited by the guard
    /// and whether the patrol is a loop.
    fn patrol(map: &Map, mut guard: Point) -> (std::collections::BTreeSet<Point>, bool) {
        let mut direction = Point::NORTH;
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
                        direction = direction.rotate_90_clockwise();
                        guard
                    }
                };
            } else {
                return (locations, false);
            }
        }
    }

    pub fn solve_part1(input: &str) -> usize {
        let (map, guard) = prepare(input);
        patrol(&map, guard).0.len()
    }

    pub fn solve_part2(input: &str) -> usize {
        let (mut map, guard) = prepare(input);
        let mut positions = patrol(&map, guard).0;
        positions.remove(&guard);
        positions
            .iter()
            .filter(|obstruction| {
                *map.get_mut(&obstruction).unwrap() = Cell::Obstruction;
                let is_loop = patrol(&map, guard).1;
                *map.get_mut(&obstruction).unwrap() = Cell::Empty;
                is_loop
            })
            .count()
    }
}

mod fast {
    //! Fast implementation
    use super::*;

    /// Execute the guard's patrol, return the set of positions visited by the guard
    /// and whether the patrol is a loop.
    ///
    ///
    fn patrol(map: &Map, mut guard: Point) -> (Vec<[bool; 4]>, bool) {
        // current guard partrolling direction
        let mut direction = Point::NORTH;

        // provide a numerical identifier for each of the four cardinal directions
        let direction_id = |direction: Point| match direction {
            Point::NORTH => 0,
            Point::EAST => 1,
            Point::SOUTH => 2,
            Point::WEST => 3,
            _ => unreachable!(),
        };

        // a boolean vector representing the `Set<(position, direction)>` of patrolled locations.
        let mut patrolled = vec![[false, false, false, false]; map.size()];
        let mut is_loop = false;
        loop {
            // if we already patrolled this location with current direction, the patrol is a loop
            let loc = patrolled.get_mut(map.unchecked_index(&guard)).unwrap();
            let did = direction_id(direction);
            if loc[did] {
                is_loop = true;
                break;
            }
            loc[did] = true;

            if let Some(ahead) = map.step(&guard, &direction) {
                guard = match map.unchecked_get(&ahead) {
                    Cell::Empty => ahead,
                    Cell::Obstruction => {
                        direction = direction.rotate_90_clockwise();
                        guard
                    }
                };
            } else {
                // left the area
                break;
            }
        }

        return (patrolled, is_loop);
    }

    pub fn solve_part1(input: &str) -> usize {
        let (map, guard) = prepare(input);
        patrol(&map, guard)
            .0
            .iter()
            .filter(|loc| loc.iter().any(|b| *b))
            .count()
    }

    pub fn solve_part2(input: &str) -> usize {
        let (mut map, guard) = prepare(input);
        let guard_index = map.strict_index(&guard);
        let positions = patrol(&map, guard)
            .0
            .iter()
            .enumerate()
            .filter_map(|(index, loc)| {
                if index != guard_index && loc.iter().any(|b| *b) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        positions
            .iter()
            .filter(|&&obstruction| {
                map.set_at(obstruction, Cell::Obstruction);
                let is_loop = patrol(&map, guard).1;
                map.set_at(obstruction, Cell::Empty);
                is_loop
            })
            .count()
    }
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = fast::solve_part1(&input);
    let sol2 = fast::solve_part2(&input);
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
        assert_eq!(slow::solve_part1(EXAMPLE_INPUT), 41);
        assert_eq!(fast::solve_part1(EXAMPLE_INPUT), 41);
    }

    #[test]
    fn example_part2() {
        assert_eq!(slow::solve_part2(EXAMPLE_INPUT), 6);
        assert_eq!(fast::solve_part2(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn preparation() {
        let (map, guard) = prepare(EXAMPLE_INPUT);
        assert_eq!(guard, Point(6, 4));
        assert_eq!(map.at(0, 0), Some(&Cell::Empty));
        assert_eq!(map.at(3, 2), Some(&Cell::Obstruction));
        assert_eq!(map.at(6, 4), Some(&Cell::Empty));
    }
}
