#![feature(test)]

use std::fs;

use anyhow::bail;
use num::Integer;

fn main() {
    let input = fs::read_to_string("./crates/day01/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let mut count = 0;
    let mut pos = 50;

    for rot in input.lines().filter_map(|line| line.try_into().ok()) {
        pos += match rot {
            Rotate::Left(x) => -x,
            Rotate::Right(x) => x,
        };

        pos = (pos + 100) % 100;

        if pos == 0 {
            count.inc();
        }
    }

    count.to_string()
}

fn process_part_2(input: &str) -> String {
    let mut count = 0;
    let mut pos = 50;

    for rot in input.lines().filter_map(|line| line.try_into().ok()) {
        let (c, r) = match rot {
            Rotate::Left(x) => {
                let (c, r) = x.div_mod_floor(&100);
                (c, -r)
            }
            Rotate::Right(x) => x.div_mod_floor(&100),
        };

        pos = (pos + 100) % 100;
        let not_zero = pos != 0;

        count += c;
        pos += r;

        if not_zero && (pos <= 0 || pos >= 100) {
            count.inc();
        }
    }

    count.to_string()
}

enum Rotate {
    Left(i32),
    Right(i32),
}

impl TryFrom<&str> for Rotate {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(tuple) = value.split_at_checked(1) {
            match tuple {
                ("L", x) => x.parse().map(Rotate::Left).map_err(From::from),
                ("R", x) => x.parse().map(Rotate::Right).map_err(From::from),
                _ => bail!("Unexpected value: {value}"),
            }
        } else {
            bail!("Unexpected value: {value}")
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "3")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "6")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("989")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("5941")]
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
