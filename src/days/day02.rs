use crate::{Solution, SolutionPair};
use std::iter::Iterator;

fn line(input: &str) -> Vec<u8> {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn prepare(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(line).collect()
}

// check increasing or decreasing property between two successive values.
fn check_xcreasing(increasing: bool, a: u8, b: u8) -> bool {
    ((increasing && a < b) || (!increasing && a > b)) && (1..=3).contains(&a.abs_diff(b))
}

fn check_xcreasing_from(increasing: bool, pos: usize, report: &Vec<u8>) -> bool {
    assert!(pos > 0);
    if pos >= report.len() {
        return true;
    }
    check_xcreasing(increasing, report[pos - 1], report[pos])
        && check_xcreasing_from(increasing, pos + 1, report)
}

fn solve_part1(input: &str) -> usize {
    let reports = prepare(input);
    reports
        .iter()
        .filter(|&report| {
            check_xcreasing_from(true, 1, report) || check_xcreasing_from(false, 1, report)
        })
        .count()
}

fn check_xcreasing_with_dampener(increasing: bool, pos: usize, report: &Vec<u8>) -> bool {
    if pos >= report.len() {
        return true;
    }
    if check_xcreasing(increasing, report[pos - 1], report[pos])
        && check_xcreasing_with_dampener(increasing, pos + 1, report)
    {
        return true;
    } else {
        if pos == 1 {
            // try skip first element of the report
            if check_xcreasing_from(increasing, 2, report) {
                return true;
            }
        }
        // drop current element from the report
        let mut report = report.clone();
        report.remove(pos);
        check_xcreasing_from(increasing, pos, &report)
    }
}

fn solve_part2(input: &str) -> usize {
    let reports = prepare(input);
    reports
        .iter()
        .filter(|&report| {
            check_xcreasing_with_dampener(true, 1, report)
                || check_xcreasing_with_dampener(false, 1, report)
        })
        .count()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1: usize = solve_part1(&input);
    let sol2: usize = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
      1 2 7 8 9
      9 7 6 2 1
      1 3 2 4 5
      8 6 4 4 1
      1 3 6 7 9";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 4);
    }

    fn check_increasing_with_dampener(pos: usize, report: &Vec<u8>) -> bool {
        check_xcreasing_with_dampener(true, pos, report)
    }

    fn check_decreasing_with_dampener(pos: usize, report: &Vec<u8>) -> bool {
        check_xcreasing_with_dampener(false, pos, report)
    }

    #[test]
    fn increasing() {
        assert!(check_increasing_with_dampener(1, &vec![50, 48, 50]));
        assert!(!check_increasing_with_dampener(1, &vec![50, 48, 48, 50]));
    }

    #[test]
    fn decreasing() {
        assert!(check_decreasing_with_dampener(1, &vec![50, 48, 50]));
        assert!(!check_decreasing_with_dampener(1, &vec![50, 48, 48, 50]));
        assert!(check_decreasing_with_dampener(1, &vec![50, 48, 50]));
    }

    #[test]
    fn bugs() {
        assert!(check_decreasing_with_dampener(1, &vec![26, 25, 22, 24, 23]));
        assert!(check_increasing_with_dampener(1, &vec![66, 68, 67, 68, 70]));
        assert!(check_increasing_with_dampener(
            1,
            &vec![53, 50, 54, 56, 59, 60, 62]
        ));
    }
}
