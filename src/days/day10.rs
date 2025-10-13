use crate::{Grid, Position, Solution, SolutionPair};

type Map = Grid<u32>;

fn prepare(input: &str) -> Map {
    let map = Grid::new(input).new_from(|c| c.to_digit(10).unwrap());
    map
}

fn solve_part1(input: &str) -> usize {
    let map = prepare(input);
    let mut score = 0;

    // breadth-first search from each zero-cell to every reachable nine-cell.
    map.for_each_with_position(|root, &level| {
        if level == 0 {
            let mut s = std::collections::HashSet::new();
            s.insert(root);

            for target in 1..10 {
                let mut snext = std::collections::HashSet::new();
                for pos in s {
                    map.for_each_neighbour(&pos, |neigh, &lvl| {
                        if lvl == target {
                            snext.insert(neigh);
                        }
                    });
                }
                s = snext;
            }
            score += s.len();
        }
    });

    score
}

type Ratings = std::collections::HashMap<Position, usize>;

fn dfs(map: &Map, ratings: &mut Ratings, pos: &Position, level: u32) -> usize {
    if let Some(rating) = ratings.get(pos) {
        *rating
    } else if *map.unchecked_get(pos) == 9 {
        ratings.insert(*pos, 1);
        1
    }else {
        ratings.insert(*pos, 0);
        let mut rating = 0;
        map.for_each_neighbour(pos, |neigh, &lvl| {
            if lvl == level + 1 {
                rating += dfs(map, ratings, &neigh, lvl);
            }
        });
        ratings.insert(*pos, rating);
        rating
    }
}

fn solve_part2(input: &str) -> usize {
    // depth-first search from each cell to every reachable nine-cell
    let map = prepare(input);
    let mut ratings = std::collections::HashMap::new();
    let mut total = 0;
    map.for_each_with_position(|root, &level| {
        let rating = dfs(&map, &mut ratings, &root, level);
        if level == 0 {
            total += rating;
        }
    });
    total
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "89010123
  78121874
  87430965
  96549874
  45678903
  32019012
  01329801
  10456732";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 36);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 81);
    }
}
