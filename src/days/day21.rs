use crate::{Solution, SolutionPair};
use petgraph::algo::dijkstra;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::prelude::EdgeIndex;
use std::collections::HashMap;

type Code = [NumericalKey; 4];

fn prepare(input: &str) -> Vec<Code> {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            [
                line.chars().nth(0).unwrap().into(),
                line.chars().nth(1).unwrap().into(),
                line.chars().nth(2).unwrap().into(),
                line.chars().nth(3).unwrap().into(),
            ]
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash, Debug)]
enum DirectionalKey {
    Up,
    Down,
    Left,
    Right,
    #[default]
    Actionate,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash, Debug)]
enum NumericalKey {
    Digit(u8),
    #[default]
    Actionate,
}

impl From<char> for NumericalKey {
    fn from(value: char) -> Self {
        match value {
            '0' => Digit(0),
            '1' => Digit(1),
            '2' => Digit(2),
            '3' => Digit(3),
            '4' => Digit(4),
            '5' => Digit(5),
            '6' => Digit(6),
            '7' => Digit(7),
            '8' => Digit(8),
            '9' => Digit(9),
            'A' => NumericalKey::Actionate,
            _ => unreachable!("unexpected char in code: {value}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash, Debug)]
struct State {
    directional_keypad1: DirectionalKey,
    directional_keypad2: DirectionalKey,
    numerical_keypad: NumericalKey,
}

use DirectionalKey::*;
use NumericalKey::Digit;

/// If the action is possible from the given position, return the couple of the updated position
/// and optional performed action on the directional keypad.
///
/// ```text
///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
/// ```
fn directional_keypad_action(
    position: DirectionalKey,
    action: DirectionalKey,
) -> Option<(DirectionalKey, Option<DirectionalKey>)> {
    let res = match (position, action) {
        (_, Actionate) => Some((position, Some(position))),
        (Up, Right) => Some((Actionate, None)),
        (Up, Down) => Some((Down, None)),
        (Left, Right) => Some((Down, None)),
        (Down, Up) => Some((Up, None)),
        (Down, Left) => Some((Left, None)),
        (Down, Right) => Some((Right, None)),
        (Right, Up) => Some((Actionate, None)),
        (Right, Left) => Some((Down, None)),
        (Actionate, Left) => Some((Up, None)),
        (Actionate, Down) => Some((Right, None)),
        // action forbidden from current state
        _ => None,
    };
    res
}

/// If the action is possible from the given position, return the couple of the updated position
/// and optional performed action on the numerical keypad.
///
/// ```text
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
/// ```
fn numerical_keypad_action(
    position: NumericalKey,
    action: DirectionalKey,
) -> Option<(NumericalKey, Option<NumericalKey>)> {
    let res = match (position, action) {
        (_, Actionate) => Some((position, Some(position))),
        // 0
        (Digit(0), Right) => Some((NumericalKey::Actionate, None)),
        (Digit(0), Up) => Some((Digit(2), None)),
        // 1
        (Digit(1), Right) => Some((Digit(2), None)),
        (Digit(1), Up) => Some((Digit(4), None)),
        // 2
        (Digit(2), Right) => Some((Digit(3), None)),
        (Digit(2), Up) => Some((Digit(5), None)),
        (Digit(2), Down) => Some((Digit(0), None)),
        (Digit(2), Left) => Some((Digit(1), None)),
        // 3
        (Digit(3), Up) => Some((Digit(6), None)),
        (Digit(3), Down) => Some((NumericalKey::Actionate, None)),
        (Digit(3), Left) => Some((Digit(2), None)),
        // 4
        (Digit(4), Right) => Some((Digit(5), None)),
        (Digit(4), Up) => Some((Digit(7), None)),
        (Digit(4), Down) => Some((Digit(1), None)),
        // 5
        (Digit(5), Right) => Some((Digit(6), None)),
        (Digit(5), Up) => Some((Digit(8), None)),
        (Digit(5), Down) => Some((Digit(2), None)),
        (Digit(5), Left) => Some((Digit(4), None)),
        // 6
        (Digit(6), Up) => Some((Digit(9), None)),
        (Digit(6), Down) => Some((Digit(3), None)),
        (Digit(6), Left) => Some((Digit(5), None)),
        // 7
        (Digit(7), Right) => Some((Digit(8), None)),
        (Digit(7), Down) => Some((Digit(4), None)),
        // 8
        (Digit(8), Down) => Some((Digit(5), None)),
        (Digit(8), Left) => Some((Digit(7), None)),
        (Digit(8), Right) => Some((Digit(9), None)),
        // 9
        (Digit(9), Left) => Some((Digit(8), None)),
        (Digit(9), Down) => Some((Digit(6), None)),
        // A
        (NumericalKey::Actionate, Left) => Some((Digit(0), None)),
        (NumericalKey::Actionate, Up) => Some((Digit(3), None)),
        // action forbidden from current state
        _ => None,
    };
    res
}

/// Apply a transition the whole system state.
fn transition(state: &State, action: DirectionalKey) -> Option<(State, Option<NumericalKey>)> {
    // action on the top directional keypad translate
    match directional_keypad_action(state.directional_keypad1, action) {
        None => None,
        Some((directional_keypad1, None)) => Some((
            State {
                directional_keypad1,
                directional_keypad2: state.directional_keypad2,
                numerical_keypad: state.numerical_keypad,
            },
            None,
        )),
        Some((directional_keypad1, Some(action))) => {
            match directional_keypad_action(state.directional_keypad2, action) {
                None => None,
                Some((directional_keypad2, None)) => Some((
                    State {
                        directional_keypad1,
                        directional_keypad2,
                        numerical_keypad: state.numerical_keypad,
                    },
                    None,
                )),
                Some((directional_keypad2, Some(action))) => {
                    match numerical_keypad_action(state.numerical_keypad, action) {
                        None => None,
                        Some((numerical_keypad, action)) => Some((
                            State {
                                directional_keypad1,
                                directional_keypad2,
                                numerical_keypad,
                            },
                            action,
                        )),
                    }
                }
            }
        }
    }
}

type SystemGraph = Graph<State, (DirectionalKey, Option<NumericalKey>)>;

fn build_system() -> SystemGraph {
    // build whole system graph, each edge is a keystroke on the human-actionable directional
    // keypad.
    let mut g = SystemGraph::new();
    let mut states: HashMap<State, NodeIndex> = Default::default();
    let mut worklist: Vec<NodeIndex> = vec![];

    let start = State::default();
    let root = g.add_node(start);
    states.insert(start, root);
    worklist.push(root);

    while let Some(from) = worklist.pop() {
        let state = g[from];
        for action in [Up, Down, Left, Right, Actionate] {
            let res = transition(&state, action);
            if let Some((next_state, maybe_output)) = res {
                let to = *states.entry(next_state).or_insert_with(|| {
                    let to = g.add_node(next_state);
                    worklist.push(to);
                    to
                });
                g.add_edge(from, to, (action, maybe_output));
            }
        }
    }

    g
}

/// Return the mapping from a numerical key to the edge in the system graph that
/// would output this numerical key.
///
/// There is a single edge `X ---(Actionate, Some(K))---> X` that output `K` and leave the
/// system state `X` unmodified.
///
fn action_to_edge(g: &SystemGraph) -> HashMap<NumericalKey, EdgeIndex> {
    let mut action_edges: HashMap<NumericalKey, EdgeIndex> = Default::default();
    for e in g.edge_indices() {
        if let Some((action, Some(w))) = g.edge_weight(e) {
            assert_eq!(*action, Actionate);
            action_edges.insert(w.to_owned(), e);
        }
    }
    action_edges
}

/// Build the graph of the whole system state (`11*5*5` different configurations), with each edge
/// being an action on the human-facing directional keypad and optionally an output of the
/// numerical keypad.
///
/// Then accumulate the shortest path length from start configuration to first digit configuration
/// and so on up to the activate key.
///
fn solve_part1(input: &str) -> u64 {
    let codes = prepare(input);
    let g = build_system();
    let a2e = action_to_edge(&g);

    let mut sum_of_complexities = 0u64;
    for code in codes {
        let mut numeric_part = 0u64;
        let mut shortest_sequence_len = 0u64;
        let mut start = petgraph::graph::node_index::<petgraph::graph::DefaultIx>(0);
        for key in code {

            match key {
                Digit(i) => numeric_part = numeric_part * 10 + (i as u64),
                _ => ()
            }

            // find length of the shortest path from current state to state that will output the key
            let output_edge = a2e.get(&key).unwrap().to_owned();
            let (from, end) = g.edge_endpoints(output_edge).unwrap();
            assert_eq!(from, end);
            let shortest_paths = dijkstra(&g, start, Some(end), |_| 1);
            let len = shortest_paths.get(&end).unwrap().to_owned();
            shortest_sequence_len += len;
            shortest_sequence_len += 1; // for the Actionate
            start = end;
        }
        sum_of_complexities += shortest_sequence_len * numeric_part;
    }

    sum_of_complexities
}

fn solve_part2(input: &str) -> u64 {
    1
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "029A
        980A
        179A
        456A
        379A";

    #[test]
    fn test_graph() {
        let g = build_system();
        // system has 11*5*5 configurations
        assert_eq!(g.node_count(), 275);
    }

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 126384);
    }

    #[test]
    fn example_part2() {
        unimplemented!()
        //assert_eq!(solve_part2(EXAMPLE_INPUT), ());
    }
}
