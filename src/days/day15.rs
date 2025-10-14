use crate::{Grid, Point, Solution, SolutionPair};

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    /// a free space
    Free,

    /// a narrow box
    Pack,

    /// a wall
    Wall,

    /// the left part of a wide box
    BoxLeft,
    /// the right part of a wide box
    BoxRight,
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Free => f.write_str(" "),
            Self::Pack => f.write_str("O"),
            Self::Wall => f.write_str("#"),
            Self::BoxLeft => f.write_str("["),
            Self::BoxRight => f.write_str("]"),
        }
    }
}

type Map = Grid<Cell>;

type Moves = Vec<Point>;

/// Read the grid of cells, the robot starting point and the moves.
fn prepare(input: &str) -> (Map, Point, Moves) {
    let (grid, moves) = input
        .split_once("\n\n")
        .expect("missing grid/moves separator");

    let grid = Grid::new(grid);

    let start = grid
        .position(|c| *c == '@')
        .expect("missing robot in input grid");

    let map = grid.new_from(|c| match c {
        '.' | '@' => Cell::Free,
        'O' => Cell::Pack,
        '#' => Cell::Wall,
        '[' => Cell::BoxLeft,
        ']' => Cell::BoxRight,
        _ => unreachable!("unexpected char {c:?} in input grid"),
    });

    let moves = moves
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Point::WEST),
            '^' => Some(Point::NORTH),
            '>' => Some(Point::EAST),
            'v' => Some(Point::SOUTH),
            _ => None,
        })
        .collect();
    (map, start, moves)
}

/// Some changes to be applied to a map.
#[derive(Default)]
struct Changes {
    free: Vec<Point>,
    update: Vec<(Point, Cell)>,
}

impl Changes {
    /// Append some changes to this one.
    fn append(&mut self, other: &mut Changes) {
        self.free.append(&mut other.free);
        self.update.append(&mut other.update);
    }

    /// Apply the changes to the given map.
    ///
    /// First apply the `free` changes and then the updates.
    fn apply(&self, map: &mut Map) {
        for point in &self.free {
            *map.get_mut(&point).unwrap() = Cell::Free;
        }
        for (point, cell) in &self.update {
            *map.get_mut(point).unwrap() = *cell;
        }
    }
}

/// Try to make `target` free by pushing in given direction.
///
/// Return the changes to apply to the grid when possible.
fn try_make_free(grid: &Map, direction: &Point, target: &Point) -> Option<Changes> {
    match grid.get(target) {
        Some(Cell::Free) => return Some(Changes::default()),
        Some(Cell::Wall) | None => return None,
        Some(Cell::Pack) => {
            let next_target = *target + *direction;
            try_make_free(grid, direction, &next_target).map(|mut changes| {
                changes.update.push((next_target, Cell::Pack));
                changes.free.push(*target);
                changes
            })
        }
        Some(c @ Cell::BoxLeft) | Some(c @ Cell::BoxRight)
            if *direction == Point::WEST || *direction == Point::EAST =>
        {
            // east or west pushes are trivial
            let next_target = *target + *direction;
            try_make_free(grid, direction, &next_target).map(|mut changes| {
                changes.update.push((next_target, *c));
                changes.free.push(*target);
                changes
            })
        }
        Some(c @ Cell::BoxLeft) | Some(c @ Cell::BoxRight) => {
            // north and south pushes
            let left = if *c == Cell::BoxLeft {
                *target
            } else {
                *target + Point::WEST
            };
            let right = left + Point::EAST;
            let left_target = left + *direction;
            let right_target = right + *direction;

            if let Some(mut left_changes) = try_make_free(grid, direction, &left_target) {
                if let Some(mut right_changes) = try_make_free(grid, direction, &right_target) {
                    let mut changes = Changes::default();
                    changes.append(&mut left_changes);
                    changes.append(&mut right_changes);
                    changes.update.push((left_target, Cell::BoxLeft));
                    changes.update.push((right_target, Cell::BoxRight));
                    changes.free.push(left);
                    changes.free.push(right);
                    return Some(changes);
                }
            }
            None
        }
    }
}

fn try_move(grid: &mut Map, direction: &Point, robot: &mut Point) -> bool {
    if let Some(target) = grid.step(robot, direction) {
        if let Some(changes) = try_make_free(grid, direction, &target) {
            changes.apply(grid);
            *robot = target;
            return true;
        }
    }
    return false;
}

fn compute_score(grid: &Map) -> u64 {
    let mut score = 0;
    grid.for_each_with_position(|pos, cell| {
        score += match cell {
            Cell::Pack | Cell::BoxLeft => 100 * pos.0 + pos.1,
            _ => 0,
        }
    });
    score.try_into().unwrap()
}

fn solve_part1(input: &str) -> u64 {
    let (mut grid, mut robot, moves) = prepare(input);
    for m in moves {
        try_move(&mut grid, &m, &mut robot);
    }
    compute_score(&grid)
}

fn solve_part2(input: &str) -> u64 {
    let input = input
        .chars()
        .map(|c| {
            match c {
                '#' => "##".to_owned(),
                'O' => "[]".to_owned(),
                '.' => "..".to_owned(),
                '@' => "@.".to_owned(),
                _ => c.to_string(),
            }
            .to_owned()
        })
        .collect::<String>();
    let (mut grid, mut robot, moves) = prepare(&input);
    for m in moves {
        try_move(&mut grid, &m, &mut robot);
    }
    compute_score(&grid)
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALLER_EXAMPLE_INPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const EXAMPLE_INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(SMALLER_EXAMPLE_INPUT), 2028);
        assert_eq!(solve_part1(EXAMPLE_INPUT), 10092);
    }

    const EXAMPLE_INPUT_2: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT_2), 105 + 207 + 306);
        assert_eq!(solve_part2(EXAMPLE_INPUT), 9021);
    }

    #[test]
    fn units() {
        let u: &str = "#######
#.....#
#..O..#
#..@..#
#######

^";
        assert_eq!(solve_part2(u), 106);

        let u: &str = "#######
#.....#
#..O..#
#..@..#
#######

>^";
        assert_eq!(solve_part2(u), 106);

        let u: &str = "#######
#..@..#
#..O..#
#.....#
#######

v";
        assert_eq!(solve_part2(u), 306);

        let u: &str = "#######
#..@..#
#..O..#
#.....#
#######

>v";
        assert_eq!(solve_part2(u), 306);

        let u: &str = "#######
#.....#
#.OO.@#
#.....#
#######

<<<<<";
        assert_eq!(solve_part2(u), 406);

        let u: &str = "#######
#.....#
#@.OO.#
#.....#
#######

>>>>>";
        assert_eq!(solve_part2(u), 418);

        let u: &str = "######
#....#
#OOO.#
#O.O.#
#OOO.#
#.OO@#
#.O..#
#....#
######

<vv<<^";
        let expect: &str = "######
#O.O.#
#OOO.#
#OOO.#
#.OO@#
#.O..#
#....#
######

<";

        assert_eq!(solve_part2(u), solve_part2(expect));
    }
}
