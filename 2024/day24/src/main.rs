#![feature(test)]

use std::fs;

use cached::proc_macro::cached;
use fxhash::FxHashMap;
use itertools::Itertools;
use num::Integer;

fn main() {
    let input = fs::read_to_string("./day24/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let (inputs, gates) = parse(input);

    let output = (0..)
        .map_while(|i| find_output(&format!("z{i:02}")[..], &inputs, &gates))
        .collect_vec();

    to_decimal(output).to_string()
}

fn process_part_2(input: &str) -> String {
    let (inputs, gates) = parse(input);

    let x = inputs
        .iter()
        .filter(|(k, _)| k.starts_with('x'))
        .sorted()
        .map(|(_, &v)| v)
        .collect_vec();
    let x = to_decimal(x);

    let y = inputs
        .iter()
        .filter(|(k, _)| k.starts_with('y'))
        .sorted()
        .map(|(_, &v)| v)
        .collect_vec();
    let y = to_decimal(y);

    let expected = to_binary(x + y);

    let output = (0..)
        .map_while(|i| find_output(&format!("z{i:02}")[..], &inputs, &gates))
        .collect_vec();

    for (i, (e, o)) in expected.into_iter().zip(output).enumerate() {
        println!("{i}: {e} - {o}");
    }

    // TODO: dunno, use the diff as a basis to search for mismatches or smth

    "".to_owned()
}

fn to_decimal(input: Vec<bool>) -> usize {
    input.into_iter().rev().fold(0usize, |mut acc, b| {
        acc <<= 1;
        if b {
            acc.inc();
        }
        acc
    })
}

fn to_binary(mut input: usize) -> Vec<bool> {
    let mut res = vec![];
    while input > 0 {
        res.push((input & 1) == 1);
        input >>= 1;
    }
    res
}

#[cached(
    key = "(String, String, String)",
    convert = r#"{(out.to_owned(), format!("{inputs:p}"), format!("{gates:p}"))}"#
)]
fn find_output(
    out: &str,
    inputs: &FxHashMap<String, bool>,
    gates: &FxHashMap<String, Gate>,
) -> Option<bool> {
    if let Some(&b) = inputs.get(out) {
        return Some(b);
    }

    if let Some(gate) = gates.get(out) {
        if let Some(in1) = find_output(&gate.in1, inputs, gates) {
            if let Some(in2) = find_output(&gate.in2, inputs, gates) {
                return Some(gate.apply(in1, in2));
            }
        }
    }

    None
}

#[derive(Debug)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Gate {
    op: Op,
    in1: String,
    in2: String,
}

impl Gate {
    fn apply(&self, input1: bool, input2: bool) -> bool {
        match self.op {
            Op::And => input1 && input2,
            Op::Or => input1 || input2,
            Op::Xor => input1 != input2,
        }
    }
}

fn parse(input: &str) -> (FxHashMap<String, bool>, FxHashMap<String, Gate>) {
    let mut lines = input.lines().skip_while(|line| line.is_empty());
    let lines = lines.by_ref();

    let inputs = lines
        .map_while(|line| line.split_once(':'))
        .map(|(s, b)| (s.to_owned(), b.trim() == "1"))
        .collect();

    let gates = lines
        .filter_map(|line| line.split_ascii_whitespace().collect_tuple())
        .map(|(s1, op, s2, _, s3)| {
            (
                s3.to_owned(),
                Gate {
                    op: match op {
                        "AND" => Op::And,
                        "OR" => Op::Or,
                        "XOR" => Op::Xor,
                        _ => panic!("unknown operation"),
                    },
                    in1: s1.to_owned(),
                    in2: s2.to_owned(),
                },
            )
        })
        .collect();

    (inputs, gates)
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE_1: &str = "
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const TEST_CASE_2: &str = "
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE_1, "4")]
    #[case(TEST_CASE_2, "2024")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE_1, "")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("61886126253040")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("")]
    fn part_2_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(&input));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE_1));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_1(&input));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE_1));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_2(&input));
    }
}
