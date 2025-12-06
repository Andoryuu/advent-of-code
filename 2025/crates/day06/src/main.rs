#![feature(test)]

use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./crates/day06/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_ascii_whitespace().collect_vec())
        .collect_vec();

    (0..lines.first().unwrap().len())
        .map(|i| process_by_group(&lines, i))
        .sum::<u64>()
        .to_string()
}

fn process_by_group(lines: &Vec<Vec<&str>>, i: usize) -> u64 {
    let is_mult = lines
        .last()
        .and_then(|line| line.get(i))
        .unwrap()
        .trim_ascii()
        == "*";

    let init = if is_mult { 1 } else { 0 };

    (0..(lines.len() - 1)).fold(init, |acc, l| {
        let value = lines
            .get(l)
            .and_then(|line| line.get(i))
            .and_then(|s| s.trim_ascii().parse::<u64>().ok())
            .unwrap();

        if is_mult {
            acc * value
        } else {
            acc + value
        }
    })
}

fn process_part_2(input: &str) -> String {
    let lines = input.lines().filter(|line| !line.is_empty()).collect_vec();

    let mut pos = 0;
    let ops = lines.last().unwrap().char_indices().collect_vec();

    let mut sum = 0;

    while let Some((ix, _)) = ops
        .iter()
        .skip(pos + 1)
        .find(|(_, c)| *c == '+' || *c == '*')
    {
        sum += process_by_column(&lines, pos, *ix - pos - 1);
        pos = *ix;
    }

    sum += process_by_column(&lines, pos, 9);

    sum.to_string()
}

fn process_by_column(lines: &Vec<&str>, pos: usize, len: usize) -> u64 {
    let slices = lines
        .iter()
        .filter_map(|line| line.get(pos..(line.len().min(pos + len))))
        .collect_vec();

    let is_mult = slices.last().unwrap().trim_ascii() == "*";
    let mut res = if is_mult { 1 } else { 0 };

    for i in 0..len {
        let _ = slices
            .iter()
            .take(slices.len() - 1)
            .filter_map(|line| line.chars().nth(i))
            .join("")
            .trim_ascii()
            .parse::<u64>()
            .map(|n| {
                if is_mult {
                    res *= n;
                } else {
                    res += n;
                }
            });
    }

    res
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "4277556")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "3263827")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("4719804927602")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("9608327000261")]
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
