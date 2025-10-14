use crate::{Solution, SolutionPair};
use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

fn prepare(input: &str) -> Vec<Machine> {
    let re = Regex::new(
        r"Button A: X\+([0-9]+), Y\+([0-9]+)\nButton B: X\+([0-9]+), Y\+([0-9]+)\nPrize: X=([0-9]+), Y=([0-9]+)",
    )
    .unwrap();
    re.captures_iter(input)
        .map(|caps| {
            let a_x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let a_y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let b_x = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let b_y = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
            let prize_x = caps.get(5).unwrap().as_str().parse::<i64>().unwrap();
            let prize_y = caps.get(6).unwrap().as_str().parse::<i64>().unwrap();
            Machine {
                a_x,
                a_y,
                b_x,
                b_y,
                prize_x,
                prize_y,
            }
        })
        .collect()
}

/// Solve each machine with a brute-force test.
/// From the puzzle, each button is pressed at most 100 times.
fn solve_part1(input: &str) -> u64 {
    let machines = prepare(input);
    let mut fewest_tokens = 0i64;
    for Machine {
        a_x,
        a_y,
        b_x,
        b_y,
        prize_x,
        prize_y,
    } in machines
    {
        let mut best_tokens = None;
        for a in 0..=100 {
            // try to skip as early as possible
            let a_a_x = a * a_x;
            let a_a_y = a * a_y;
            if a_a_x > prize_x || a_a_y > prize_y {
                continue;
            }

            for b in 0..=100 {
                // try to leave b loop as early as possible
                if best_tokens.is_some_and(|best| 3 * a + b > best) {
                    break;
                }
                if best_tokens.is_none_or(|best| (3 * a + b) < best)
                    && (a_a_x + b * b_x == prize_x)
                    && (a_a_y + b * b_y == prize_y)
                {
                    best_tokens = Some(3 * a + b);
                }
            }
        }
        fewest_tokens += best_tokens.unwrap_or(0)
    }
    fewest_tokens as u64
}

/// Solve the equation system:
///
/// ```text
/// A*a + B*b = X
/// A*c + B*d = Y
/// ```
/// where `a = a_x, b = b_y, c = a_y, d = b_y, X = prize_x, Y = prize_y`.
/// and all variables are integers.
///
/// ```text
/// A = (dX - bY)/(ad - cb)
/// B = (X - aA)/b = (Y - Ac)/d
/// ```
/// We don't need to minimise for `3a+b` since these equations have either no solution
/// or a single solution for `a` and `b`.
///
fn solve_part2(input: &str) -> u64 {
    let mut machines = prepare(input);
    for machine in machines.iter_mut() {
        machine.prize_x += 10000000000000;
        machine.prize_y += 10000000000000;
    }
    let mut fewest_tokens = 0i64;
    for Machine {
        a_x,
        a_y,
        b_x,
        b_y,
        prize_x,
        prize_y,
    } in machines
    {
        let denominator = a_x * b_y - b_x * a_y;
        let a_numerator = b_y * prize_x - b_x * prize_y;
        if a_numerator.rem_euclid(denominator) == 0 {
            let a = a_numerator.div_euclid(denominator);
            if b_y * (prize_x - a * a_x) == b_x * (prize_y - a * a_y) {
                let b_numerator = prize_x - a * a_x;
                if b_numerator.rem_euclid(b_x) == 0 {
                    let b = b_numerator.div_euclid(b_x);
                    fewest_tokens += 3 * a + b;
                }
            }
        }
    }
    fewest_tokens as u64
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 480);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 875318608908);
    }
}
