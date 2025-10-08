use crate::{Solution, SolutionPair};
use std::collections::HashMap;

fn line(input: &str) -> (u64, u64) {
    let mut it = input.split_ascii_whitespace();
    let a = it.next().unwrap().parse().unwrap();
    let b = it.next().unwrap().parse().unwrap();
    (a,b)
}

fn prepare(input: &str) -> (Vec<u64>, Vec<u64>) {
    input.lines().map(line).unzip()
}

fn solve_part1(input: &str) -> u64 {
    let (mut a, mut b) = prepare(input);
    a.sort();
    b.sort();
    a.into_iter().zip(b).map(|(a,b)| a.abs_diff(b)).sum()
}

fn solve_part2(input: &str) -> u64 {
    let (a, b) = prepare(input);
    let mut counts = HashMap::new();
    for num in b {
        *counts.entry(num).or_default() += 1;
    }
    a.iter().map(|x| x * counts.get(x).unwrap_or(&0)).sum()
}

pub fn solve(input: String) -> SolutionPair {
    let p1: u64 = solve_part1(&input);
    let p2: u64 = solve_part2(&input);

    (Solution::from(p1), Solution::from(p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 11);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 31);
    }
}
