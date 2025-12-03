#![feature(test)]

use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./crates/day03/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    calculate_joltage(input, 2).to_string()
}

fn process_part_2(input: &str) -> String {
    calculate_joltage(input, 12).to_string()
}

fn calculate_joltage(input: &str, size: usize) -> u64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| get_max_joltage(line, size))
        .sum()
}

fn get_max_joltage(line: &str, size: usize) -> u64 {
    let values = line
        .char_indices()
        .filter_map(|(ix, c)| c.to_digit(10).map(|v| (ix, v as u64)))
        .collect_vec();

    (0..size)
        .rev()
        .fold((0, 0), |(from_ix, res), n| {
            values
                .iter()
                .skip(from_ix)
                .take(values.len() - n - from_ix)
                .rev()
                .max_by_key(|(_, v)| v)
                .map(|(ix, value)| (ix + 1, res * 10 + value))
                .unwrap()
        })
        .1
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
987654321111111
811111111111119
234234234234278
818181911112111";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "357")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "3121910778619")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("17430")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("171975854269367")]
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
