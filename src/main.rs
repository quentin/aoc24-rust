mod days;
mod etc;

use etc::solution::Solution;
use days::{day01, day02};
use std::env;

pub type SolutionPair = (Solution, Solution);

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
        let input = std::fs::read_to_string(format!("./input/day{:0>2}.txt", day)).unwrap();
        let (p1, p2) = match day {
            1 => day01::solve(input),
            2 => day02::solve(input),
            _ => unimplemented!(),
        };

        println!("\n=== Day {:02} ===", day);
        println!("   Part 1: {}", p1);
        println!("   Part 2: {}", p2);
    }
}
