#![feature(test)]

mod helpers;

use std::fs;

use helpers::{hailstone::Hailstone, linesegment::Intersectable};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day24/_data/input.txt").expect("oh noes");

    println!(
        "Part 1 result is: {}",
        process_part_1(&input, 200_000_000_000_000, 400_000_000_000_000)
    );
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str, min: isize, max: isize) -> String {
    let stones = input
        .trim()
        .lines()
        .map(Hailstone::from)
        .filter_map(|s| s.clamped_xy(min, max))
        .collect_vec();

    stones
        .into_iter()
        .tuple_combinations()
        .filter(|(a, b)| a.intersects(*b))
        .count()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let stones = input.trim().lines().map(Hailstone::from).collect_vec();

    // should it be possible to create a line from a small subset of stones, like 3-4 tops?

    "".to_owned()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[rstest]
    #[case(TEST_CASE, 7, 27, "2")]
    fn part_1_check(
        #[case] input: &str,
        #[case] min: isize,
        #[case] max: isize,
        #[case] expected: &str,
    ) {
        assert_eq!(expected, process_part_1(input, min, max));
    }

    #[rstest]
    #[case(TEST_CASE, "47")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("13892")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(
            expected,
            process_part_1(&input, 200_000_000_000_000, 400_000_000_000_000)
        );
    }

    #[rstest]
    #[case("")]
    fn part_2_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_2(&input));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE, 7, 27));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_1(&input, 200_000_000_000_000, 400_000_000_000_000));
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
