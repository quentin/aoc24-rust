use crate::{Grid, Point, Solution, SolutionPair};

fn prepare(input: &str) -> Vec<Point> {
    let re = regex::Regex::new(r"([0-9]+),([0-9]+)").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            Point(
                caps.get(1).unwrap().as_str().parse().unwrap(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect()
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
enum Cell {
    #[default]
    Free,
    Corrupted,
    Reached(u64),
}

fn bfs(map: &mut Grid<Cell>, start: Point) {
    let mut worklist: std::collections::VecDeque<Point> = Default::default();
    map.update(&start, Cell::Reached(0));
    worklist.push_back(start);
    while let Some(pos) = worklist.pop_front() {
        if let &Cell::Reached(dist) = map.unchecked_get(&pos) {
            for dir in [Point::NORTH, Point::EAST, Point::SOUTH, Point::WEST] {
                let at = pos + dir;
                match map.get_mut(&at) {
                    Some(c @ Cell::Free) => {
                        *c = Cell::Reached(dist + 1);
                        worklist.push_back(at);
                    }
                    _ => (),
                }
            }
        }
    }
}

fn solve_part1(input: &str, columns: usize, lines: usize, steps: u64) -> u64 {
    let corruptions = prepare(input);
    let mut map = Grid::<Cell>::default(lines, columns);
    for i in 0..steps {
        map.update(&corruptions[i as usize], Cell::Corrupted);
    }

    bfs(&mut map, Point(0, 0));

    match map.unchecked_get(&Point(
        (columns - 1) as i64,
        (lines - 1).try_into().unwrap(),
    )) {
        Cell::Reached(dist) => *dist,
        _ => unreachable!("no path found"),
    }
}

fn solve_part2(input: &str, lines: usize, columns: usize) -> String {
    let corruptions = prepare(input);
    let mut map = Grid::<Cell>::default(lines, columns);
    for i in 0..corruptions.len() {
        let corrupt = &corruptions[i as usize];
        map.update(corrupt, Cell::Corrupted);
        map.update_each(|cell| {
            if matches!(cell, Cell::Reached(_)) {
                *cell = Cell::Free
            }
        });
        bfs(&mut map, Point(0, 0));
        if *map.unchecked_get(&Point(
            (columns - 1).try_into().unwrap(),
            (lines - 1).try_into().unwrap(),
        )) == Cell::Free
        {
            return format!("{},{}", corrupt.0, corrupt.1).to_string()
        }
    }

    unreachable!("did not find the point")
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input, 71, 71, 1024);
    let sol2 = solve_part2(&input, 71, 71);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "5,4
  4,2
  4,5
  3,0
  2,1
  6,3
  2,4
  1,5
  0,6
  3,3
  2,6
  5,1
  1,2
  5,5
  2,5
  6,5
  1,4
  0,4
  6,4
  1,1
  6,1
  1,0
  0,5
  1,6
  2,0";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT, 7, 7, 12), 22);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT, 7, 7), "6,1");
    }
}
