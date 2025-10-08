use crate::{Solution, SolutionPair};

struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}

type Equations = Vec<Equation>;

fn prepare(input: &str) -> Equations {
    let mut eqs = Equations::default();
    let mut eq: Option<Equation> = None;

    for item in input.split_ascii_whitespace() {
        if let Some(x) = item.strip_suffix(':') {
            if let Some(eq) = eq {
                eqs.push(eq);
            }
            eq = Some(Equation {
                test_value: x.parse().unwrap(),
                operands: Vec::default(),
            });
        } else {
            eq.as_mut().unwrap().operands.push(item.parse().unwrap());
        }
    }
    if let Some(eq) = eq {
        eqs.push(eq);
    }
    eqs
}

fn solve_equation_rec(
    operations: &[&dyn Fn(u64, u64) -> u64],
    test_value: u64,
    lhs: u64,
    operands: &[u64],
) -> bool {
    if lhs > test_value {
        return false;
    }
    if let Some((first, rest)) = operands.split_first() {
        operations
            .iter()
            .any(|op| solve_equation_rec(operations, test_value, op(lhs, *first), rest))
    } else {
        lhs == test_value
    }
}

fn solve_equation(operations: &[&dyn Fn(u64, u64) -> u64], eq: &Equation) -> bool {
    solve_equation_rec(operations, eq.test_value, 0, eq.operands.as_slice())
}

fn add(x: u64, y: u64) -> u64 {
    x + y
}
fn mul(x: u64, y: u64) -> u64 {
    x * y
}

fn solve_part1(input: &str) -> u64 {
    let eqs = prepare(input);
    let operations: &[&dyn Fn(u64, u64) -> u64] = &[
        &add, &mul,
    ];
    eqs.iter()
        .filter(|eq| solve_equation(operations, eq))
        .map(|eq| eq.test_value)
        .sum()
}

fn con(x: u64, y: u64) -> u64 {
    format!("{}{}", x, y).parse().unwrap()
}

fn solve_part2(input: &str) -> u64 {
    let eqs = prepare(input);
    let operations: &[&dyn Fn(u64, u64) -> u64] = &[&add, &mul, &con];
    eqs.iter()
        .filter(|eq| solve_equation(operations, eq))
        .map(|eq| eq.test_value)
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

    const EXAMPLE_INPUT: &str = "190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 3749);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 11387);
    }

    #[test]
    fn preparation() {
        let eqs = prepare(EXAMPLE_INPUT);
        assert_eq!(eqs[0].test_value, 190);
        assert_eq!(eqs[0].operands, vec![10, 19]);
        assert_eq!(eqs[8].test_value, 292);
        assert_eq!(eqs[8].operands, vec![11, 6, 16, 20]);
    }
}
