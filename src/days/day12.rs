use crate::{Grid, Point, Solution, SolutionPair};
use partitions::PartitionVec;
use std::ops::Add;

type Farm = Grid<char>;

fn prepare(input: &str) -> Farm {
    Grid::new(input)
}

/// Compute regions by computing the equivalence class of touching farm plots growing the same
/// type of plant. Compute the number of fences for each farm's plot.
fn compute_regions_and_fences(
    farm: &Farm,
) -> (PartitionVec<Point>, std::collections::BTreeMap<Point, u64>) {
    let mut regions = PartitionVec::new();
    let mut plot_fences = std::collections::BTreeMap::new();
    farm.for_each_with_position(|plot, _| regions.push(plot));
    farm.for_each_with_position(|plot, &plant| {
        // at most four fences
        let mut fences = 4;
        farm.for_each_neighbour(&plot, |neigh, neigh_plant| {
            if *neigh_plant == plant {
                // neighboor in same region
                regions.union(farm.unchecked_index(&plot), farm.unchecked_index(&neigh));
                // no fence needed with that neighboor
                fences -= 1;
            }
        });
        plot_fences.insert(plot, fences);
    });
    (regions, plot_fences)
}

fn solve_part1(input: &str) -> u64 {
    let farm = prepare(input);
    let (regions, plot_fences) = compute_regions_and_fences(&farm);
    regions
        .all_sets()
        .map(|region| {
            let mut area: u64 = 0;
            let mut perimeter: u64 = 0;
            for (_, plot) in region {
                area += 1;
                perimeter += plot_fences[plot];
            }
            area * perimeter
        })
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    let farm = prepare(input);
    let (regions, _) = compute_regions_and_fences(&farm);
    let mut corners = std::collections::BTreeMap::<Point, u64>::new();
    farm.for_each_with_index(|index, _| {
        let pos = farm.unchecked_position(index);

        let not_same_region = |delta| {
            let other = pos.add(delta);
            farm.checked_index(&other)
                .is_none_or(|other_index| regions.other_sets(index, other_index))
        };

        let same_region = |delta| {
            let other = pos.add(delta);
            farm.checked_index(&other)
                .is_some_and(|other_index| regions.same_set(index, other_index))
        };

        let mut corns: u64 = 0;
        // detect the following 8 corner patterns:
        //
        // .x  x.  .?  ?.
        // ?.  .?  x.  .x
        //
        // xX  Xx  x.  .x
        // .x  x.  Xx  xX
        //
        // each pattern count as 1 corner for the region X, where `.` is not part of region
        // X and `?` is of any region.
        //
        corns += (not_same_region(Point::WEST) && not_same_region(Point::SOUTH)) as u64;
        corns += (not_same_region(Point::EAST) && not_same_region(Point::SOUTH)) as u64;
        corns += (not_same_region(Point::NORTH) && not_same_region(Point::EAST)) as u64;
        corns += (not_same_region(Point::NORTH) && not_same_region(Point::WEST)) as u64;

        corns += (same_region(Point::WEST)
            && same_region(Point::SOUTH)
            && not_same_region(Point::SOUTH_WEST)) as u64;
        corns += (same_region(Point::EAST)
            && same_region(Point::SOUTH)
            && not_same_region(Point::SOUTH_EAST)) as u64;
        corns += (same_region(Point::NORTH)
            && same_region(Point::EAST)
            && not_same_region(Point::NORTH_EAST)) as u64;
        corns += (same_region(Point::NORTH)
            && same_region(Point::WEST)
            && not_same_region(Point::NORTH_WEST)) as u64;
        corners.insert(pos, corns);
    });

    regions
        .all_sets()
        .map(|region| {
            let mut area: u64 = 0;
            let mut sides: u64 = 0;
            for (_, plot) in region {
                area += 1;
                sides += corners.get(&plot).unwrap_or(&0u64);
            }
            area * sides
        })
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

    const EXAMPLE_INPUT: &str = "RRRRIICCFF
      RRRRIICCCF
      VVRRRCCFFF
      VVRCCCJFFF
      VVVVCJJCFE
      VVIVCCJJEE
      VVIIICJJEE
      MIIIIIJJEE
      MIIISIJEEE
      MMMISSJEEE";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 1930);
    }

    #[test]
    fn example_part2() {
        assert_eq!(
            solve_part2(
                "AAAA
        BBCD
        BBCC
        EEEC"
            ),
            80
        );
        assert_eq!(
            solve_part2(
                "EEEEE
        EXXXX
        EEEEE
        EXXXX
        EEEEE"
            ),
            236
        );
        assert_eq!(
            solve_part2(
                "AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA"
            ),
            368
        );
        assert_eq!(solve_part2(EXAMPLE_INPUT), 1206);
    }
}
