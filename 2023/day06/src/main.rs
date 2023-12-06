#![feature(test)]

use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day06/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|part| part.parse::<u32>().ok())
        })
        .next_tuple()
        .map(|(times, dists)| {
            times
                .zip(dists)
                .filter_map(|(t, d)| (1..t).find(|i| (i * (t - i)) > d).map(|i| t + 1 - 2 * i))
                .product::<u32>()
        })
        .unwrap()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<usize>()
                .ok()
        })
        .next_tuple()
        .and_then(|(t, d)| (1..t).find(|i| (i * (t - i)) > d).map(|i| t + 1 - 2 * i))
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
Time:      7  15   30
Distance:  9  40  200";

    #[rstest]
    #[case(TEST_CASE, "288")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "71503")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("2344708")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("30125202")]
    fn part_2_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_2(&input));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_1(&input));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_2(&input));
    }
}
