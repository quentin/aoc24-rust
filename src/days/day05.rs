use crate::{Solution, SolutionPair};
use std::collections::BTreeSet;

type Page = u32;
type PageOrdering = BTreeSet<[Page; 2]>;
type Updates = Vec<Vec<Page>>;

fn prepare(input: &str) -> (PageOrdering, Updates) {
    let empty_line = input.find("\n\n").unwrap();
    let (orderings, updates) = input.split_at(empty_line);
    let orderings = orderings
        .split_ascii_whitespace()
        .map(|ordering| {
            let mut two = ordering
                .split('|')
                .map(|x| x.parse::<Page>().unwrap())
                .take(2);
            [two.next().unwrap(), two.next().unwrap()]
        })
        .collect();
    let updates = updates
        .split_ascii_whitespace()
        .map(|x| x.split(',').map(|x| x.parse::<Page>().unwrap()).collect())
        .collect();
    (orderings, updates)
}

fn check_update(orderings: &PageOrdering, update: &Vec<Page>) -> bool {
    for i in 0..(update.len() - 1) {
        for j in (i + 1)..update.len() {
            if !orderings.contains(&[update[i], update[j]]) {
                return false;
            }
        }
    }
    true
}

pub fn solve_part1(input: &str) -> usize {
    let (orderings, updates) = prepare(input);
    updates
        .iter()
        .filter(|&update| check_update(&orderings, update))
        .map(|update| update[update.len() / 2])
        .sum::<u32>()
        .try_into()
        .unwrap()
}

fn reorder_update(orderings: &PageOrdering, mut update: Vec<Page>) -> Vec<Page> {
    update.sort_by(|a,b| {
        if orderings.contains(&[*a,*b]) {
            std::cmp::Ordering::Less
        } else if orderings.contains(&[*b,*a]) {
            std::cmp::Ordering::Greater
        } else {
            assert_eq!(a,b);
            std::cmp::Ordering::Equal
        }
    });
    update
}

pub fn solve_part2(input: &str) -> usize {
    let (orderings, updates) = prepare(input);
    updates
        .iter()
        .filter(|&update| !check_update(&orderings, update))
        .map(|update| reorder_update(&orderings, update.clone()))
        .map(|update| update[update.len() / 2])
        .sum::<u32>()
        .try_into()
        .unwrap()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 143);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 123);
    }

    #[test]
    fn preparation() {
        let (orderings, updates) = prepare(EXAMPLE_INPUT);
        assert!(orderings.contains(&[97, 13]));
        assert!(orderings.contains(&[53, 13]));
        assert_eq!(orderings.len(), 21);
        assert!(updates.contains(&vec![61, 13, 29]));
    }
}
