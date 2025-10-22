use crate::etc::grid::TAXICAB_DIRECTIONS;
use crate::{Grid, Point, Solution, SolutionPair};

#[derive(Copy, PartialEq, Clone)]
enum Cell {
    Wall,
    // track with distance from start
    Track(Option<u64>),
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Wall => f.write_str("#####"),
            Cell::Track(None) => f.write_str("....."),
            Cell::Track(Some(dist)) => f.write_fmt(format_args!(" {dist:03} ")),
        }
    }
}

type Map = Grid<Cell>;

fn prepare(input: &str) -> (Map, Point) {
    let grid = Grid::new(input);
    let start = grid
        .position(|&c| c == 'S')
        .expect("missing start position");
    let map = grid.new_from(|&c| match c {
        '#' => Cell::Wall,
        '.' | 'S' | 'E' => Cell::Track(None),
        _ => unreachable!("wrong cell type"),
    });
    (map, start)
}

/// Compute track distance from start, update the track distances accordingly.
fn compute_distances(map: &mut Map, start: Point) {
    let mut at = Some(start);
    let mut dist = 0;
    while let Some(pos) = at {
        map.update(&pos, Cell::Track(Some(dist)));
        dist += 1;
        at = TAXICAB_DIRECTIONS
            .iter()
            .map(|dir| pos + *dir)
            .find(|neigh| matches!(map.get(neigh), Some(Cell::Track(None))));
    }
}

/// Compute the list of how much each distinct cheat saves.
///
/// For each position on the track, evaluate the possible cheats in the four directions.
fn compute_cheats(map: &Map, save_min: u64, save_max: u64) -> Vec<u64> {
    let mut cheats: Vec<u64> = Default::default();
    map.for_each_with_position(|pos, cell| {
        if let Cell::Track(Some(dist)) = cell {
            for dir in [
                Point::NORTH * 2,
                Point::EAST * 2,
                Point::SOUTH * 2,
                Point::WEST * 2,
            ] {
                let at = pos + dir;
                if let Some(Cell::Track(Some(other_dist))) = map.get(&at) {
                    if *other_dist > *dist + 2 {
                        let saves = other_dist - dist - 2;
                        if saves >= save_min && saves <= save_max {
                            cheats.push(saves);
                        }
                    }
                }
            }
        }
    });
    cheats
}

fn solve_part1(input: &str, save_min: u64, save_max: u64) -> u64 {
    let (mut map, start) = prepare(input);
    compute_distances(&mut map, start);
    let cheats = compute_cheats(&map, save_min, save_max);
    cheats.len().try_into().unwrap()
}

/// Compute the set of track points reachable from `start`.
/// Return the mapping from reachable track points to the distance from track start.
fn bfs_wall(
    map: &Map,
    pos: &Point,
    pos_dist: u64,
    max_len: u64,
) -> std::collections::HashMap<
    Point,
    (
        /* distance from start on track */ u64,
        /* distance using cheat */ u64,
    ),
> {
    let mut reachable_tracks: std::collections::HashMap<Point, (u64, u64)> = Default::default();
    let mut distances: std::collections::HashMap<Point, u64> = Default::default();
    let mut worklist: Vec<Point> = Default::default();

    distances.insert(*pos, pos_dist);
    worklist.push(*pos);

    let mut cheat_dist = pos_dist;
    while !worklist.is_empty() && cheat_dist < (pos_dist + max_len) {
        cheat_dist += 1;
        let mut next_worklist: Vec<Point> = Default::default();
        while let Some(pos) = worklist.pop() {
            for dir in TAXICAB_DIRECTIONS {
                let neigh = pos + dir;
                let neigh_cell = map.get(&neigh);
                match neigh_cell {
                    Some(Cell::Wall) => {
                        if !distances.contains_key(&neigh) {
                            distances.insert(neigh, cheat_dist);
                            next_worklist.push(neigh);
                        }
                    }
                    Some(Cell::Track(Some(neigh_dist))) => {
                        if !distances.contains_key(&neigh) {
                            distances.insert(neigh, cheat_dist);
                            next_worklist.push(neigh);
                        }
                        reachable_tracks
                            .entry(neigh)
                            .or_insert((*neigh_dist, cheat_dist));
                    }
                    _ => (),
                }
            }
        }
        std::mem::swap(&mut worklist, &mut next_worklist);
    }
    reachable_tracks
}

/// Compute the list of how much each distinct cheat saves.
/// Cheats can be up to `max_len` long.
fn compute_cheats_upto(map: &Map, save_min: u64, save_max: u64, max_len: u64) -> Vec<u64> {
    let mut track: Vec<(Point, u64)> = Default::default();
    map.for_each_with_position(|pos, cell| {
        if let Cell::Track(Some(dist)) = cell {
            track.push((pos, *dist));
        }
    });

    let mut cheats: Vec<u64> = Default::default();
    for &(pos, dist) in &track {
        for (_other, (other_dist, cheat_dist)) in bfs_wall(map, &pos, dist, max_len) {
            if other_dist > dist {
                let saves = other_dist - cheat_dist;
                if saves >= save_min && saves <= save_max {
                    cheats.push(saves);
                }
            }
        }
    }

    cheats
}

fn solve_part2(input: &str, save_min: u64, save_max: u64, max_len: u64) -> u64 {
    let (mut map, start) = prepare(input);
    compute_distances(&mut map, start);
    let cheats = compute_cheats_upto(&map, save_min, save_max, max_len);
    cheats.len().try_into().unwrap()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input, 100, u64::max_value());
    let sol2 = solve_part2(&input, 100, u64::max_value(), 20);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT, 64, 64), 1);
        assert_eq!(solve_part1(EXAMPLE_INPUT, 40, 40), 1);
        assert_eq!(solve_part1(EXAMPLE_INPUT, 38, 38), 1);
        assert_eq!(solve_part1(EXAMPLE_INPUT, 36, 36), 1);
        assert_eq!(solve_part1(EXAMPLE_INPUT, 20, 20), 1);
        assert_eq!(solve_part1(EXAMPLE_INPUT, 12, 12), 3);
        assert_eq!(solve_part1(EXAMPLE_INPUT, 10, 10), 2);
        assert_eq!(solve_part1(EXAMPLE_INPUT, 8, 8), 4);
        assert_eq!(solve_part1(EXAMPLE_INPUT, 6, 6), 2);
        assert_eq!(solve_part1(EXAMPLE_INPUT, 4, 4), 14);
        assert_eq!(solve_part1(EXAMPLE_INPUT, 2, 2), 14);
    }

    #[test]
    fn example_part2() {
        // from part 1
        assert_eq!(solve_part2(EXAMPLE_INPUT, 64, 64, 2), 1);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 40, 40, 2), 1);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 38, 38, 2), 1);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 36, 36, 2), 1);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 20, 20, 2), 1);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 12, 12, 2), 3);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 10, 10, 2), 2);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 8, 8, 2), 4);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 6, 6, 2), 2);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 4, 4, 2), 14);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 2, 2, 2), 14);

        // with cheats up to 20 ps
        assert_eq!(solve_part2(EXAMPLE_INPUT, 76, 76, 20), 3);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 74, 74, 20), 4);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 70, 70, 20), 12);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 72, 72, 20), 22);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 50, 50, 20), 32);
    }

}
