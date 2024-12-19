#![feature(test)]

use std::fs;

use cached::proc_macro::cached;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day19/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let (towels, patterns) = parse(input);

    patterns
        .into_iter()
        .filter(|pattern| count_designs(pattern.to_owned(), &towels) > 0)
        .count()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let (towels, patterns) = parse(input);

    patterns
        .into_iter()
        .map(|pattern| count_designs(pattern, &towels))
        .sum::<usize>()
        .to_string()
}

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = input.lines().skip_while(|line| line.is_empty());
    let lines = lines.by_ref();

    let towels = lines
        .next()
        .map(|line| line.split(',').map(|p| p.trim().to_owned()).collect_vec())
        .unwrap();

    let patterns = lines
        .filter(|line| !line.is_empty())
        .map(|line| line.to_owned())
        .collect_vec();

    (towels, patterns)
}

#[cached(
    key = "(String, String)",
    convert = r#"{(pattern.clone(), format!("{towels:p}"))}"#
)]
fn count_designs(pattern: String, towels: &[String]) -> usize {
    towels
        .iter()
        .map(|towel| {
            if &pattern == towel {
                1
            } else if pattern.starts_with(towel) {
                count_designs(pattern[towel.len()..].to_owned(), towels)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "6")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "16")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("209")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("777669668613191")]
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
