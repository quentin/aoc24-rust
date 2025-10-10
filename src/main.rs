mod days;
mod etc;

use days::*;
use etc::grid::Grid;
use etc::solution::Solution;
use std::env;

pub type SolutionPair = (Solution, Solution);

fn solve_day(day: u8) -> SolutionPair {
    let input = std::fs::read_to_string(format!("./input/day{:0>2}.txt", day)).unwrap();
    match day {
        1 => day01::solve(input),
        2 => day02::solve(input),
        3 => day03::solve(input),
        4 => day04::solve(input),
        5 => day05::solve(input),
        6 => day06::solve(input),
        7 => day07::solve(input),
        8 => day08::solve(input),
        _ => unimplemented!(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day(s)");
    }

    let days: Vec<u8> = args[1..]
        .iter()
        .map(|x| {
            x.parse()
                .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
        })
        .collect();

    for day in days {
        let (p1, p2) = solve_day(day);
        println!("\n=== Day {:02} ===", day);
        println!("   Part 1: {}", p1);
        println!("   Part 2: {}", p2);
    }
}

#[cfg(test)]
mod tests {
    use crate::solve_day;
    use crate::Solution;

    #[test]
    fn my_puzzles() {
        assert_eq!(solve_day(1), (Solution::from(765748u64), Solution::from(27732508u64)));
        assert_eq!(solve_day(2), (Solution::from(479usize), Solution::from(531usize)));
        assert_eq!(solve_day(3), (Solution::from(170807108u64), Solution::from(74838033u64)));
        assert_eq!(solve_day(4), (Solution::from(2397usize), Solution::from(1824usize)));
        assert_eq!(solve_day(5), (Solution::from(7024usize), Solution::from(4151usize)));
        assert_eq!(solve_day(6), (Solution::from(4939usize), Solution::from(1434usize)));
        assert_eq!(solve_day(7), (Solution::from(4555081946288u64), Solution::from(227921760109726u64)));
        assert_eq!(solve_day(8), (Solution::from(269usize), Solution::from(949usize)));
    }
}
