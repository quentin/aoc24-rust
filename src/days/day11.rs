use crate::{Solution, SolutionPair};

type Stones = Vec<u64>;

fn prepare(input: &str) -> Stones {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

/// stone evolution after a single blink
fn blink_once(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        (1, None)
    } else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let mut left = stone;
            let mut right = 0;
            for dec in 0..(digits / 2) {
                right = right + 10u64.pow(dec) * (left % 10);
                left = left / 10;
            }
            (left, Some(right))
        } else {
            (2024 * stone, None)
        }
    }
}

/// stones evolution after a single blink
fn blink_all(stones: &Stones) -> Stones {
    let mut result = Stones::with_capacity(stones.len() * 2);
    for stone in stones {
        let (left, maybe_right) = blink_once(*stone);
        result.push(left);
        if let Some(right) = maybe_right {
            result.push(right);
        }
    }
    result
}

fn solve_part1(input: &str, blinks_times: usize) -> usize {
    let stones = prepare(input);
    stones
        .iter()
        .map(|seed| {
            let mut v = vec![*seed];
            for _ in 0..blinks_times {
                let vprime = blink_all(&v);
                v = vprime;
            }
            v.len()
        })
        .sum()
}

/// Memoization datastructure.
///
/// `Memo[i][j] -> count` is the associative mapping from a single stone with number `j`
/// to the number of stones after `i` blinks.
///
type Memo<const N: usize> = [std::collections::BTreeMap<u64, usize>; N];

/// Recursive count the number of stones after remaining number of blinks using memoization.
fn fast_blink_all<const N: usize>(
    memo: &mut [std::collections::BTreeMap<u64, usize>; N],
    stone: u64,
    remaining_blinks: usize,
) -> usize {
    if remaining_blinks == 0 {
        return 1;
    }

    if let Some(count) = memo[remaining_blinks].get(&stone) {
        // memoized
        return *count;
    }

    // compute and memoize one blink
    let (left, maybe_right) = blink_once(stone);
    let count = fast_blink_all(memo, left, remaining_blinks - 1)
        + maybe_right.map_or(0, |right| fast_blink_all(memo, right, remaining_blinks - 1));
    memo[remaining_blinks].insert(stone, count);
    count
}

fn solve_part2(input: &str, blinks_times: usize) -> usize {
    let stones = prepare(input);
    if blinks_times >= 100 {
        unimplemented!("hardcoded for up to 100 blinks")
    }

    let mut memo: Memo<100> = std::array::from_fn(|_| Default::default());

    stones
        .iter()
        .map(|&stone| fast_blink_all(&mut memo, stone, blinks_times))
        .sum()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input, 25);
    let sol2 = solve_part2(&input, 75);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "125 17";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT, 6), 22);
        assert_eq!(solve_part1(EXAMPLE_INPUT, 25), 55312);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT, 1), 3);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 2), 4);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 3), 5);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 4), 9);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 6), 22);
        assert_eq!(solve_part2(EXAMPLE_INPUT, 25), 55312);
    }
}
