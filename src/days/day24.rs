use crate::{Solution, SolutionPair};
use std::collections::{BTreeMap, VecDeque};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Ord, PartialOrd)]
enum Wire {
    Other(String),
    X(u64),
    Y(u64),
    Z(u64),
}

impl ToString for Wire {
    fn to_string(&self) -> String {
        match self {
            Self::X(i) => format!("x{i:02}"),
            Self::Y(i) => format!("y{i:02}"),
            Self::Z(i) => format!("z{i:02}"),
            Self::Other(name) => name.to_string(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Gate {
    op: Op,
    lhs: Wire,
    rhs: Wire,
    out: Wire,
}

type WireValueMap = BTreeMap<Wire, bool>;
type GateVec = VecDeque<Gate>;

fn make_wire(name: &str) -> Wire {
    if name.starts_with('z') {
        Wire::Z(name[1..3].parse().unwrap())
    } else if name.starts_with('x') {
        Wire::X(name[1..3].parse().unwrap())
    } else if name.starts_with('y') {
        Wire::Y(name[1..3].parse().unwrap())
    } else {
        Wire::Other(name.to_string())
    }
}

fn prepare(input: &str) -> (WireValueMap, GateVec) {
    let mut gates: GateVec = Default::default();
    let mut available: WireValueMap = Default::default();

    let mut lines = input.lines().map(|line| line.trim());
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let name = &line[0..3];
        let signal = if &line[5..6] == "0" { false } else { true };
        available.insert(make_wire(name), signal);
    }

    while let Some(line) = lines.next() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let a = make_wire(parts[0]);
        let b = make_wire(parts[2]);
        let (lhs, rhs) = if a < b { (a, b) } else { (b, a) };
        let out = make_wire(parts[4]);
        let op = match parts[1] {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => unreachable!(),
        };
        gates.push_back(Gate { op, lhs, rhs, out });
    }

    (available, gates)
}

/// Evaluate gates based on availability of their input signals.
///
/// Maintains a mapping of available wire signals, and a worklist of gates.
///
/// When both input signals of a gate remaining in the worklist are available, the gate is removed
/// from the worklist, evaluated, and the signal on the output wire becomes available. When not all
/// input signals are available, the gate evaluation is postponed.
///
/// Return `None` if the circuit is not well-formed.
///
fn evaluate_circuit(mut available: WireValueMap, mut gates: GateVec) -> Option<u64> {
    let mut postpones = 0;
    while let Some(gate) = gates.pop_front() {
        if let (Some(lhs), Some(rhs)) = (available.get(&gate.lhs), available.get(&gate.rhs)) {
            let out = match gate.op {
                Op::And => lhs & rhs,
                Op::Or => lhs | rhs,
                Op::Xor => lhs ^ rhs,
            };
            available.insert(gate.out, out);
            postpones = 0;
        } else {
            postpones += 1;
            gates.push_back(gate);
        }
        if postpones > gates.len() {
            return None;
        }
    }

    Some(
        available
            .iter()
            .filter_map(|(wire, signal)| match wire {
                Wire::Z(bit) => Some(if *signal { 1 << bit } else { 0 }),
                _ => None,
            })
            .sum(),
    )
}

fn solve_part1(input: &str) -> u64 {
    let (available, gates) = prepare(input);
    evaluate_circuit(available, gates).unwrap()
}

fn match_op(gate: &Gate, w1: &Wire, w2: &Wire, op: Op) -> bool {
    gate.op == op && ((gate.lhs == *w1 && gate.rhs == *w2) || (gate.lhs == *w2 && gate.rhs == *w1))
}

fn match_xor(gate: &Gate, w1: &Wire, w2: &Wire) -> bool {
    match_op(gate, w1, w2, Op::Xor)
}

fn match_or(gate: &Gate, w1: &Wire, w2: &Wire) -> bool {
    match_op(gate, w1, w2, Op::Or)
}

fn match_and(gate: &Gate, w1: &Wire, w2: &Wire) -> bool {
    match_op(gate, w1, w2, Op::And)
}

fn match_out(gate: &Gate, out: &Wire) -> bool {
    gate.out == *out
}

/// Find permutations that fix the adder circuit.
///
/// It's a semi-automatic solution. The circuit is a classical adder with carry.
/// So we do concistency checks of every expected gates and discover permuted gate outputs.
///
/// Probably not fixing all possible permutations, but it's enough for my input of the problem.
///
fn solve_part2(input: &str, input_len: u64) -> String {
    let (_available, mut gates) = prepare(input);

    let mut permuted: Vec<Wire> = Default::default();

    let _x0_xor_y0 = gates
        .iter()
        .find(|g| match_xor(g, &Wire::X(0), &Wire::Y(0)) && match_out(g, &Wire::Z(0)))
        .unwrap();

    let mut carry_out = gates
        .iter()
        .find(|g| match_and(g, &Wire::X(0), &Wire::Y(0)))
        .unwrap()
        .clone();

    for i in 1..input_len {
        let _ipred = i - 1;
        let x = Wire::X(i);
        let y = Wire::Y(i);
        let z = Wire::Z(i);

        let x_xor_y = gates.iter().find(|g| match_xor(g, &x, &y)).unwrap().clone();

        // expect: `(xi ^ yi) ^ carry -> zi`
        let x_xor_y_xor_cin = gates
            .iter()
            .find(|g| match_xor(g, &x_xor_y.out, &carry_out.out));

        if let Some(x_xor_y_xor_cin) = x_xor_y_xor_cin {
            let x_xor_y_xor_cin = x_xor_y_xor_cin.clone();

            if x_xor_y_xor_cin.out != z {
                // found `(xi ^ y1) ^ carry -> not zi`
                //
                //println!("PERMUTE {z:?} with {:?}", x_xor_y_xor_cin.out);
                permuted.push(z.clone());
                permuted.push(x_xor_y_xor_cin.out.clone());
                gates.iter_mut().for_each(|g| {
                    if g.out == z {
                        g.out = x_xor_y_xor_cin.out.clone();
                    } else if g.out == x_xor_y_xor_cin.out {
                        g.out = z.clone();
                    }
                });
            }
        } else {
            // cannot find `(xi^yi)^carry` at all.
            //
            // so... let's search `k^carry -> zi`
            // and then permute output of `k` with output of `(xi^yi)`.
            let k_and_carry = gates
                .iter()
                .find(|g| {
                    g.op == Op::Xor
                        && (g.lhs == carry_out.out || g.rhs == carry_out.out)
                        && g.out == z
                })
                .unwrap()
                .clone();

            let k = if k_and_carry.lhs == carry_out.out {
                k_and_carry.rhs
            } else if k_and_carry.rhs == carry_out.out {
                k_and_carry.lhs
            } else {
                unreachable!()
            };
            //println!("PERMUTE {k:?} with {:?}", x_xor_y.out);
            permuted.push(k.clone());
            permuted.push(x_xor_y.out.clone());
            gates.iter_mut().for_each(|g| {
                if g.out == k {
                    g.out = x_xor_y.out.clone();
                } else if g.out == x_xor_y.out {
                    g.out = k.clone();
                }
            });
        }
        // reload `xi^yi` since may have permuted its output wire.
        let x_xor_y = gates.iter().find(|g| match_xor(g, &x, &y)).unwrap().clone();

        let x_and_y = gates.iter().find(|g| match_and(g, &x, &y)).unwrap();
        let x_xor_y_and_carry = gates
            .iter()
            .find(|g| match_and(g, &x_xor_y.out, &carry_out.out))
            .unwrap();
        let x_and_y_or_x_xor_y_and_carry = gates
            .iter()
            .find(|g| match_or(g, &x_and_y.out, &x_xor_y_and_carry.out))
            .unwrap();

        // new carry out
        carry_out = x_and_y_or_x_xor_y_and_carry.clone();
    }

    permuted.sort();
    permuted
        .iter()
        .map(|w| w.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input, 45);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "x00: 1
    x01: 1
    x02: 1
    y00: 0
    y01: 1
    y02: 0

    x00 AND y00 -> z00
    x01 XOR y01 -> z01
    x02 OR y02 -> z02";

    #[test]
    fn test_make_wire() {
        assert_eq!(make_wire("z00"), Wire::Z(0));
        assert_eq!(make_wire("z01"), Wire::Z(1));
        assert_eq!(make_wire("z24"), Wire::Z(24));
    }

    #[test]
    fn test_prepare() {
        let (available, gates) = prepare(EXAMPLE_INPUT);
        let x00 = Wire::X(0);
        let x01 = Wire::X(1);
        let x02 = Wire::X(2);
        let y00 = Wire::Y(0);
        let y01 = Wire::Y(1);
        let y02 = Wire::Y(2);
        let z00 = Wire::Z(0);
        let z01 = Wire::Z(1);
        let z02 = Wire::Z(2);
        assert_eq!(
            available,
            WireValueMap::from([
                (x00.clone(), true),
                (x01.clone(), true),
                (x02.clone(), true),
                (y00.clone(), false),
                (y01.clone(), true),
                (y02.clone(), false),
            ])
        );
        assert_eq!(
            gates,
            vec![
                Gate {
                    op: Op::And,
                    lhs: x00,
                    rhs: y00,
                    out: z00
                },
                Gate {
                    op: Op::Xor,
                    lhs: x01,
                    rhs: y01,
                    out: z01
                },
                Gate {
                    op: Op::Or,
                    lhs: x02,
                    rhs: y02,
                    out: z02
                }
            ]
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 4);
    }
}
