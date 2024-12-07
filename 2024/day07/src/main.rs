#![feature(test)]
#![feature(unsigned_is_multiple_of)]

use std::{fs, ops::Div};

use itertools::Itertools;
use num::Integer;

fn main() {
    let input = fs::read_to_string("./day07/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    parse(input)
        .into_iter()
        .filter_map(|(res, mut parts)| {
            parts.reverse();
            if can_be_combined_add_mult(res, &parts) {
                Some(res)
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    parse(input)
        .into_iter()
        .filter_map(|(res, mut parts)| {
            parts.reverse();
            if can_be_combined_full(res, &parts) {
                Some(res)
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string()
}

fn can_be_combined_add_mult(res: usize, parts: &[usize]) -> bool {
    if let Some((&head_part, tail_parts)) = parts.split_first() {
        (res.is_multiple_of(head_part) && can_be_combined_add_mult(res.div(head_part), tail_parts))
            || (res >= head_part && can_be_combined_add_mult(res - head_part, tail_parts))
    } else {
        res == 0
    }
}

fn can_be_combined_full(res: usize, parts: &[usize]) -> bool {
    if let Some((&head_part, tail_parts)) = parts.split_first() {
        if res >= head_part {
            let head_pow = 10usize.pow(head_part.ilog10() + 1);
            let (res_trimed, res_rem) = (res - head_part).div_rem(&head_pow);

            if res_rem == 0 && can_be_combined_full(res_trimed, tail_parts) {
                return true;
            }
        }

        (res.is_multiple_of(head_part) && can_be_combined_full(res.div(head_part), tail_parts))
            || (res >= head_part && can_be_combined_full(res - head_part, tail_parts))
    } else {
        res == 0
    }
}

fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .filter_map(|line| line.split_once(':'))
        .filter_map(|(res, nums)| {
            res.parse::<usize>().ok().map(|r| {
                (
                    r,
                    nums.split_ascii_whitespace()
                        .filter_map(|n| n.parse::<usize>().ok())
                        .collect_vec(),
                )
            })
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "3749")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "11387")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("6392012777720")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("61561126043536")]
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
