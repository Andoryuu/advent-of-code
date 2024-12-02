#![feature(test)]

use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day02/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    parse(input)
        .into_iter()
        .filter(|report| is_safe(report))
        .count()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    parse(input)
        .into_iter()
        .filter(|report| {
            report
                .iter()
                .combinations(report.len() - 1)
                .map(|subreport| subreport.into_iter().copied().collect_vec())
                .any(|subreport| is_safe(&subreport))
        })
        .count()
        .to_string()
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|p| p.parse::<i32>().ok())
                .collect_vec()
        })
        .collect_vec()
}

fn is_safe(report: &[i32]) -> bool {
    report.iter().tuple_windows().all(|(a, b, c)| {
        (b - a).signum() == (c - b).signum()
            && (1..=3).contains(&a.abs_diff(*b))
            && (1..=3).contains(&b.abs_diff(*c))
    })
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "2")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "4")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("591")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("621")]
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
