#![feature(test)]

use std::{collections::VecDeque, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day09/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|p| p.parse::<isize>().ok())
                .collect_vec()
        })
        .filter_map(|seq| {
            let mut seq = seq;
            let mut lasts = VecDeque::<isize>::new();
            lasts.push_front(*seq.last().unwrap());

            loop {
                seq = seq.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();
                lasts.push_front(*seq.last().unwrap());

                if let Some(fst) = seq.first() {
                    if seq.iter().all(|s| s == fst) {
                        return lasts.into_iter().reduce(|acc, v| acc + v);
                    }
                }
            }
        })
        .sum::<isize>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|p| p.parse::<isize>().ok())
                .collect_vec()
        })
        .filter_map(|seq| {
            let mut seq = seq;
            let mut firsts = VecDeque::<isize>::new();
            firsts.push_front(*seq.first().unwrap());

            loop {
                seq = seq.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();
                firsts.push_front(*seq.first().unwrap());

                if let Some(fst) = seq.first() {
                    if seq.iter().all(|s| s == fst) {
                        return firsts.into_iter().reduce(|acc, v| v - acc);
                    }
                }
            }
        })
        .sum::<isize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[rstest]
    #[case(TEST_CASE, "114")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "2")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("1995001648")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("988")]
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
