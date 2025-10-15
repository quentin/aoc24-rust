use crate::{Grid, Point, Solution, SolutionPair};

#[derive(Copy, Clone)]
enum Cell {
    /// A wall
    Wall,
    /// Not reached yet
    Unreached,
    /// Reached with minimum cost
    Reached(u64),
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => f.write_str("#######"),
            Self::Unreached => f.write_str("       "),
            Self::Reached(points) => f.write_fmt(format_args!("{points:6} ")),
        }
    }
}

type Map = Grid<Cell>;

fn prepare(input: &str) -> (Map, Point, Point) {
    let grid = Grid::new(input);
    let start = grid.position(|&c| c == 'S').expect("missing start cell");
    let end = grid.position(|&c| c == 'E').expect("missing end cell");
    let map = grid.new_from(|c| match c {
        '#' => Cell::Wall,
        '.' | 'E' | 'S' => Cell::Unreached,
        _ => unreachable!("wrong char"),
    });
    (map, start, end)
}

/// Compute least distance from start point.
fn dfs(map: &mut Map, end: &Point, accumulated_points: u64, at: Point, direction: Point) {
    let cell = map.get_mut(&at);
    match cell {
        None | Some(Cell::Wall) => return,
        Some(Cell::Reached(points)) if *points <= accumulated_points => return,
        Some(c @ Cell::Unreached) | Some(c @ Cell::Reached(_)) => {
            *c = Cell::Reached(accumulated_points)
        }
    }

    if at == *end {
        return;
    }

    for (next_direction, cost) in [
        // same direction
        (direction, 1),
        // turn right
        (direction.rotate_90_clockwise(), 1001),
        // turn left
        (direction.rotate_90_counterclockwise(), 1001),
    ] {
        dfs(
            map,
            end,
            accumulated_points + cost,
            at + next_direction,
            next_direction,
        );
    }
}

/// Compute least distance from start to each cell and return the least distance to end point.
fn compute_least_distances(map: &mut Map, start: Point, end: Point) -> u64 {
    dfs(map, &end, 0, start, Point::EAST);
    let best = if let Some(Cell::Reached(points)) = map.get(&end) {
        *points
    } else {
        unreachable!("no path found")
    };
    best
}

fn solve_part1(input: &str) -> u64 {
    let (mut map, start, end) = prepare(input);
    compute_least_distances(&mut map, start, end)
}

/// Mark every cell on a best path using a least distance map.
fn backward_dfs(
    least_distance_map: &Map,
    start: &Point,
    on_a_best_path: &mut std::collections::HashSet<Point>,
    remaining_points: u64,
    at: Point,
    incoming_direction: Point,
) {
    on_a_best_path.insert(at);

    if at == *start {
        return;
    }

    for (turn_direction, cost) in [
        // same direction
        (incoming_direction, 1),
        // turn right
        (incoming_direction.rotate_90_clockwise(), 1001),
        // turn left
        (incoming_direction.rotate_90_counterclockwise(), 1001),
    ] {
        let at_turn = at + turn_direction;
        if let Some(Cell::Reached(forward_points)) = least_distance_map.get(&at_turn) {
            if *forward_points <= remaining_points - cost {
                backward_dfs(
                    least_distance_map,
                    start,
                    on_a_best_path,
                    remaining_points - cost,
                    at_turn,
                    turn_direction,
                );
            }
        }
    }
}

fn solve_part2(input: &str) -> u64 {
    let (mut least_distance_map, start, end) = prepare(input);
    let mut on_a_best_path = std::collections::HashSet::<Point>::new();

    let best = compute_least_distances(&mut least_distance_map, start, end);

    // run backward dfs using least distance map computed in previous stage
    for direction in [Point::NORTH, Point::EAST, Point::SOUTH, Point::WEST] {
        backward_dfs(
            &least_distance_map,
            &start,
            &mut on_a_best_path,
            best,
            end,
            direction,
        );
    }

    on_a_best_path.len().try_into().unwrap()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############";

    const EXAMPLE_INPUT_2: &str = "#################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 7036);
        assert_eq!(solve_part1(EXAMPLE_INPUT_2), 11048);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 45);
        assert_eq!(solve_part2(EXAMPLE_INPUT_2), 64);
    }
}
