use crate::{Solution, SolutionPair};
use regex::Regex;

fn solve_part1(input: &str) -> u64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            caps.get(1).unwrap().as_str().parse::<u64>().unwrap()
                * caps.get(2).unwrap().as_str().parse::<u64>().unwrap()
        })
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();
    let mut factor = 1;
    re.captures_iter(input)
        .map(|caps| {
            let all = caps.get(0).unwrap().as_str();
            if all.starts_with("mul") {
                factor
                    * caps.get(1).unwrap().as_str().parse::<u64>().unwrap()
                    * caps.get(2).unwrap().as_str().parse::<u64>().unwrap()
            } else if all.starts_with("don") {
                factor = 0;
                0
            } else {
                assert!(all.starts_with("do"));
                factor = 1;
                0
            }
        })
        .sum()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1: u64 = solve_part1(&input);
    let sol2: u64 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const EXAMPLE_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT1), 161);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT2), 48);
    }
}
