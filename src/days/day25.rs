use crate::{Solution, SolutionPair};

type Heights = [i32; 5];
type Locks = Vec<Heights>;
type Keys = Vec<Heights>;

fn prepare(input: &str) -> (Locks, Keys) {
    let mut locks = Locks::default();
    let mut keys = Locks::default();

    let mut lines = input.lines();
    while let Some(top) = lines.next() {
        let mut heights: Heights = [0i32; 5];
        for _ in 0..5 {
            let line = lines.next().unwrap().trim();
            for i in 0..5 {
                if line.chars().nth(i).unwrap() == '#' {
                    heights[i] += 1;
                }
            }
        }

        if top.trim() == "....." {
            keys.push(heights);
        } else {
            locks.push(heights);
        };

        // skip last
        lines.next();

        // skip empty
        lines.next();
    }

    (locks, keys)
}

fn solve_part1(input: &str) -> u64 {
    let mut fits = 0;
    let (locks, keys) = prepare(input);
    for lock in &locks {
        for key in &keys {
            if lock.iter().zip(key).all(|(l,k)| l+k < 6) {
                fits += 1;
            }
        }
    }
    fits
}

fn solve_part2(input: &str) -> () {
    ()
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "#####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn example_part2() {
        unimplemented!()
        //assert_eq!(solve_part2(EXAMPLE_INPUT), ());
    }
}
