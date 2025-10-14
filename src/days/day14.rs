use crate::{Point, Solution, SolutionPair};
use regex::Regex;

struct Robot {
    position: Point,
    velocity: Point,
}

type Robots = Vec<Robot>;

fn prepare(input: &str) -> Robots {
    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    re.captures_iter(input)
        .map(|caps| Robot {
            position: Point(
                caps.get(1).unwrap().as_str().parse().unwrap(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
            ),
            velocity: Point(
                caps.get(3).unwrap().as_str().parse().unwrap(),
                caps.get(4).unwrap().as_str().parse().unwrap(),
            ),
        })
        .collect()
}

/// Update robots position as a vector transposition.
fn transpose_robots(robots: &mut Robots, columns: u64, lines: u64, steps: u64) {
    let limit = Point(columns as i64, lines as i64);
    robots.iter_mut().for_each(|robot| {
        robot.position =
            (((robot.position + robot.velocity * (steps as i64)) % limit) + limit) % limit
    })
}

/// Compute the safety factor for the given robot positions.
fn safety_factor(robots: &Robots, columns: u64, lines: u64) -> u64 {
    let mid_column = (columns / 2) as i64;
    let mid_line = (lines / 2) as i64;
    robots
        .iter()
        .fold(
            [0u64, 0u64, 0u64, 0u64],
            |[mut q1, mut q2, mut q3, mut q4],
             Robot {
                 position,
                 velocity: _,
             }| {
                if position.0 < mid_column {
                    if position.1 < mid_line {
                        q1 += 1;
                    } else if position.1 > mid_line {
                        q2 += 1;
                    }
                } else if position.0 > mid_column {
                    if position.1 < mid_line {
                        q3 += 1;
                    } else if position.1 > mid_line {
                        q4 += 1;
                    }
                }
                [q1, q2, q3, q4]
            },
        )
        .iter()
        .product()
}

fn solve_part1(input: &str, columns: u64, lines: u64) -> u64 {
    let mut robots = prepare(input);
    transpose_robots(&mut robots, columns, lines, 100);
    safety_factor(&robots, columns, lines)
}

fn has_overlap(robots: &Robots) -> bool {
    let mut positions = std::collections::BTreeSet::<Point>::new();
    for &Robot {
        position,
        velocity: _,
    } in robots
    {
        if !positions.insert(position) {
            return true;
        }
    }
    return false;
}

/// Find the number of steps required to have no robots overlapping.
fn solve_part2(input: &str) -> u64 {
    let mut robots = prepare(input);
    for steps in 1..=103 * 101 {
        transpose_robots(&mut robots, 101, 103, 1);
        if !has_overlap(&robots) {
            return steps;
        }
    }
    unreachable!("did not find a configuration without overlap")
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input, 101, 103);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "p=0,4 v=3,-3
      p=6,3 v=-1,-3
      p=10,3 v=-1,2
      p=2,0 v=2,-1
      p=0,0 v=1,3
      p=3,0 v=-2,-2
      p=7,6 v=-1,-3
      p=3,0 v=-1,-2
      p=9,3 v=2,3
      p=7,3 v=-1,2
      p=2,4 v=2,-3
      p=9,5 v=-3,-3";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT, 7, 11), 12);
    }
}
