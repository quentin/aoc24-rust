use crate::{Solution, SolutionPair};

#[derive(Debug, PartialEq)]
struct Machine {
    /// register A
    a: u64,
    /// register B
    b: u64,
    /// register C
    c: u64,
    /// instruction pointer
    ip: usize,
    /// program
    program: Vec<u8>,
}

fn prepare(input: &str) -> Machine {
    let re = regex::Regex::new(r"[0-9]+").unwrap();
    let mut caps = re.captures_iter(input);
    let a = caps
        .next()
        .expect("missing register A")
        .get(0)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let b = caps
        .next()
        .expect("missing register B")
        .get(0)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let c = caps
        .next()
        .expect("missing register C")
        .get(0)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let program = caps
        .map(|cap| cap.get(0).unwrap().as_str().parse().unwrap())
        .collect();

    Machine {
        a,
        b,
        c,
        ip: 0,
        program,
    }
}

// opcodes
const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

fn execute(machine: &mut Machine) -> Vec<u8> {
    let mut out = vec![];
    let end_ip = machine.program.len();
    while machine.ip < end_ip {
        let op = machine.program[machine.ip];
        let arg = machine.program[machine.ip + 1];
        machine.ip += 2;

        let literal = || arg as u64;

        let combo = || match arg {
            0 | 1 | 2 | 3 => arg as u64,
            4 => machine.a,
            5 => machine.b,
            6 => machine.c,
            7 => unreachable!("reserved"),
            _ => panic!("unexpected combo value"),
        };

        match op {
            // 0
            ADV => {
                machine.a = machine.a >> combo();
            }
            // 1
            BXL => {
                machine.b = machine.b ^ literal();
            }
            // 2
            BST => {
                machine.b = combo() & 0x7;
            }
            // 3
            JNZ => {
                if machine.a > 0 {
                    machine.ip = literal() as usize;
                }
            }
            // 4
            BXC => {
                machine.b = machine.b ^ machine.c;
            }
            // 5
            OUT => {
                out.push((combo() & 0x7) as u8);
            }
            // 6
            BDV => {
                machine.b = machine.a >> combo();
            }
            // 7
            CDV => {
                machine.c = machine.a >> combo();
            }
            _ => panic!("unexpected opcode"),
        }
    }
    out
}

fn solve_part1(input: &str) -> String {
    let mut machine = prepare(input);
    let out = execute(&mut machine);
    out.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn dfs(
    digit_to_ten_bits: &[Vec<u16>; 8],
    expected: &[u8],
    low_seven_bits: Option<u16>,
) -> Option<u64> {
    if expected.is_empty() {
        return Some(low_seven_bits.unwrap().into());
    }

    let has_constraint = low_seven_bits.is_some();
    let low_seven_bits = low_seven_bits.unwrap_or_default();
    let mut best_solution: Option<u64> = None;

    let digit = expected[0];
    for &ten_bits in &digit_to_ten_bits[digit as usize] {
        if !has_constraint || (ten_bits & 0o177) == low_seven_bits {
            if let Some(solution) = dfs(
                digit_to_ten_bits,
                expected.split_at(1).1,
                Some((ten_bits >> 3) & 0o177),
            ) {
                let solution = (solution << 3) + Into::<u64>::into(ten_bits & 0o7);
                if best_solution.is_none_or(|best| best > solution) {
                    best_solution = Some(solution)
                }
            }
        }
    }

    best_solution
}

/// solve my specific problem input by hand.
fn solve_part2(input: &str) -> u64 {
    //
    //          0   2   4   6   8   10  12  14
    //          --- --- --- --- --- --- --- ---
    // Program: 2,4,1,3,7,5,0,3,1,5,4,4,5,5,3,0
    //          --- --- --- --- --- --- --- ---
    //          BST BXL CDV ADV BXL BXC OUT JNZ
    //          (A) (3) (B) (3) (5) (_) (B) (0)
    //
    // entry: A0 = phi(A, A1)
    //     0: B0 = A0 & 7
    //     2: B1 = B0 ^ 3
    //     4: C0 = A0 >> B1
    //     6: A1 = A0 >> 3
    //     8: B2 = B1 ^ 5
    //    10: B3 = B2 ^ C0
    //    12: OUT(B3)
    //    14: if A1 > 0 goto entry
    //        else halt
    //
    // B3 = B2 ^ C0
    //    = (B1 ^ 5) ^ C0
    //    = ((B0 ^ 3) ^ 5) ^ C0
    //    = (((A0 & 7) ^ 3) ^ 5) ^ C0
    //    = (((A0 & 7) ^ 3) ^ 5) ^ (A0 >> B1)
    //    = (((A0 & 7) ^ 3) ^ 5) ^ (A0 >> (B0 ^ 3))
    //    = (((A0 & 7) ^ 3) ^ 5) ^ (A0 >> ((A0 & 7) ^ 3))
    //  B3 & 7 -> out
    //
    //  So each step of the loop reads up to 10 bits of A, consumes 3 bits of A.
    //
    let mut machine = prepare(input);

    // Mapping from next octal digit that the machine would output to the set of possible 10 bits of register A.
    let mut digit_to_ten_bits: [Vec<u16>; 8] = Default::default();

    // Build the mapping from all 10 bits patterns to the next output of the machine.
    for a in 0..(1 << 10) {
        machine.a = a;
        machine.ip = 0;
        let out = execute(&mut machine);
        let first_out = *out.first().unwrap();
        digit_to_ten_bits[first_out as usize].push(a as u16);
    }

    // search the smallest value of A, using the patterns.
    let expected = machine.program.clone();
    let a = dfs(&digit_to_ten_bits, expected.as_slice(), None).expect("did not find solution");
    machine.a = a;
    machine.ip = 0;
    let out = execute(&mut machine);
    assert_eq!(machine.program, out);
    a

}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
    Register A: 729
    Register B: 0
    Register C: 0

    Program: 0,1,5,4,3,0";

    const EXAMPLE_INPUT_2: &str = "
    Register A: 2024
    Register B: 0
    Register C: 0

    Program: 0,3,5,4,3,0";

    #[test]
    fn example_part1() {
        let mut machine = Machine {
            a: 0,
            b: 0,
            c: 9,
            ip: 0,
            program: vec![2, 6],
        };
        execute(&mut machine);
        assert_eq!(1, machine.b);

        let mut machine = Machine {
            a: 10,
            b: 0,
            c: 0,
            ip: 0,
            program: vec![5, 0, 5, 1, 5, 4],
        };
        assert_eq!(vec![0, 1, 2], execute(&mut machine));

        let mut machine = Machine {
            a: 2024,
            b: 0,
            c: 0,
            ip: 0,
            program: vec![0, 1, 5, 4, 3, 0],
        };
        assert_eq!(vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0], execute(&mut machine));
        assert_eq!(0, machine.a);

        let mut machine = Machine {
            a: 0,
            b: 29,
            c: 0,
            ip: 0,
            program: vec![1, 7],
        };
        execute(&mut machine);
        assert_eq!(26, machine.b);

        let mut machine = Machine {
            a: 0,
            b: 2024,
            c: 43690,
            ip: 0,
            program: vec![4, 0],
        };
        execute(&mut machine);
        assert_eq!(44354, machine.b);

        assert_eq!(solve_part1(EXAMPLE_INPUT), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn example_part2() {
        let mut machine = Machine {
            a: 117440,
            b: 0,
            c: 0,
            ip: 0,
            program: vec![0, 3, 5, 4, 3, 0],
        };
        let out = execute(&mut machine);
        assert_eq!(machine.program, out);

        assert_eq!(solve_part2(EXAMPLE_INPUT_2), 117440);
    }

    #[test]
    fn preparation() {
        let machine = prepare(EXAMPLE_INPUT);
        assert_eq!(
            Machine {
                a: 729,
                b: 0,
                c: 0,
                ip: 0,
                program: vec![0, 1, 5, 4, 3, 0]
            },
            machine
        );
    }
}
