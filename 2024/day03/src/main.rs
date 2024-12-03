#![feature(test)]

use std::fs;

use fancy_regex::{Captures, Regex};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day03/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    parse(input)
        .into_iter()
        .filter_map(|inst| {
            if let Instruction::Mul(a, b) = inst {
                Some(a * b)
            } else {
                None
            }
        })
        .sum::<i32>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let mut sum = 0;
    let mut enabled = true;

    for inst in parse(input).into_iter() {
        match inst {
            Instruction::Mul(a, b) => {
                if enabled {
                    sum += a * b
                }
            }
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
        }
    }

    sum.to_string()
}

fn parse(input: &str) -> Vec<Instruction> {
    Regex::new(r"(mul|do|don't)\((?:(\d+),(\d+))?\)")
        .unwrap()
        .captures_iter(input)
        .map_while(|r| {
            r.map_err(|e| e.to_string())
                .and_then(Instruction::try_from)
                .ok()
        })
        .collect_vec()
}

enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

impl TryFrom<Captures<'_>> for Instruction {
    type Error = String;

    fn try_from(value: Captures<'_>) -> Result<Self, Self::Error> {
        let inst = value.get(1).map(|m| m.as_str());
        let param1 = value.get(2).and_then(|m| m.as_str().parse::<i32>().ok());
        let param2 = value.get(3).and_then(|m| m.as_str().parse::<i32>().ok());

        match (inst, param1, param2) {
            (Some("mul"), Some(a), Some(b)) => Result::Ok(Instruction::Mul(a, b)),
            (Some("do"), None, None) => Result::Ok(Instruction::Do),
            (Some("don't"), None, None) => Result::Ok(Instruction::Dont),
            _ => Result::Err(format!(
                "Unknown capture: {:?}, {:?}, {:?}",
                inst, param1, param2
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_CASE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "161")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE_2, "48")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("159892596")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("92626942")]
    fn part_2_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(&input));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_1(&input));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_2(&input));
    }
}
