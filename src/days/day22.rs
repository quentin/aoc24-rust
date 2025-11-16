use crate::{Solution, SolutionPair};

fn prepare(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn next_secret(secret: u32) -> u32 {
    let secret_prime = ((secret << 6) ^ secret) & 0xffffff;
    let secret_prime = ((secret_prime >> 5) ^ secret_prime) & 0xffffff;
    ((secret_prime << 11) ^ secret_prime) & 0xffffff
}

fn solve_part1(input: &str) -> u64 {
    let secrets = prepare(input);
    secrets
        .iter()
        .copied()
        .map(|mut secret| -> u64 {
            for _ in 0..2000 {
                secret = next_secret(secret)
            }
            secret.into()
        })
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    let secrets = prepare(input);
    let buyer_price_and_changes = secrets
        .iter()
        .copied()
        .map(|mut secret| {
            let mut price_and_changes = Vec::<(i32, i32)>::new();
            for _ in 0..2000 {
                let price = (secret % 10) as i32;
                let secret_prime = next_secret(secret);
                let price_prime = (secret_prime % 10) as i32;
                price_and_changes.push((price_prime, price_prime - price));
                secret = secret_prime;
            }
            price_and_changes
        })
        .collect::<Vec<Vec<(i32, i32)>>>();

    let mut signal_price_sum: std::collections::HashMap<[i32; 4], u64> = Default::default();
    for price_and_changes in buyer_price_and_changes {
        let mut seen_signal: std::collections::HashSet<[i32; 4]> = Default::default();
        for win in price_and_changes.windows(4) {
            let signal: [i32; 4] = [win[0].1, win[1].1, win[2].1, win[3].1];
            if seen_signal.insert(signal) {
                let price: u64 = win[3].0.try_into().unwrap();
                *signal_price_sum.entry(signal).or_default() += price;
            }
        }
    }

    let best_signal = signal_price_sum.into_iter().max_by(|a, b| a.1.cmp(&b.1));
    best_signal.unwrap().1
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1
    10
    100
    2024";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 37327623);
    }

    const EXAMPLE_INPUT_2: &str = "1
        2
        3
        2024";

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT_2), 23);
    }
}
