use crate::{Solution, SolutionPair};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Pattern = Vec<char>;
type Design = Vec<char>;

type Patterns = Vec<Pattern>;
type Designs = Vec<Design>;

fn prepare(input: &str) -> (Patterns, Designs) {
    // patterns
    let mut lines = input.lines();
    let line = lines.next().unwrap();
    let patterns = line.split(", ").map(|s| s.chars().collect()).collect();
    lines.next();
    let designs = lines.map(|s| s.trim().chars().collect()).collect();

    (patterns, designs)
}

/// Some sort of 1D DFS where we apply patterns from left to right on each design.
fn solve_part1(input: &str) -> usize {
    let (patterns, designs) = prepare(input);
    designs
        .iter()
        .filter(|&design| {
            // how far we covered the design from left to right
            let mut upto: std::collections::HashSet<usize> = Default::default();
            // where to continue covering the design with a pattern
            let mut worklist: BinaryHeap<usize> = Default::default();
            worklist.push(0);
            // peek the longest covered length in priority
            while let Some(len) = worklist.pop() {
                if len == design.len() {
                    return true;
                }
                for pattern in &patterns {
                    let newlen = len + pattern.len();
                    if upto.contains(&newlen) {
                        // already covered the design up to that point
                        continue;
                    }
                    if design[len..].starts_with(&pattern) {
                        assert!(upto.insert(newlen));
                        // covered the design up to that point
                        worklist.push(newlen);
                    }
                }
            }
            false
        })
        .count()
}

/// Some sort of 1D BFS where we apply patterns from left to right and count how
/// many combinations of patterns cover the design.
fn solve_part2(input: &str) -> u64 {
    let (patterns, designs) = prepare(input);
    designs
        .iter()
        .filter_map(|design| {
            // how far and how many times we covered the design from left to right
            let mut upto: std::collections::HashMap<usize, u64> = Default::default();
            upto.insert(0, 1);
            // where to start covering the design with a pattern
            let mut worklist = BinaryHeap::new();
            worklist.push(Reverse(0usize));
            // peek the shortest covered length in priority
            while let Some(Reverse(len)) = worklist.pop() {
                if len == design.len() {
                    continue;
                }
                let factor = *upto.get(&len).unwrap();
                for pattern in &patterns {
                    let newlen = len + pattern.len();
                    if design[len..].starts_with(&pattern) {
                        match upto.entry(newlen) {
                            std::collections::hash_map::Entry::Vacant(v) => {
                                v.insert(factor);
                                // first time we covered the design up to that point
                                worklist.push(Reverse(newlen));
                            }
                            std::collections::hash_map::Entry::Occupied(mut o) => {
                                *o.get_mut() += factor;
                            }
                        }
                    }
                }
            }
            upto.get(&design.len()).copied()
        })
        .sum()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

      brwrr
      bggr
      gbbr
      rrbgbr
      ubwu
      bwurrg
      brgr
      bbrgwb";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 16);
    }
}
