#![feature(test)]

use std::{fs, ops::RangeInclusive};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./crates/day05/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let (ranges, ingredients) = parse(input);

    ingredients
        .into_iter()
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
        .count()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let (ranges, _) = parse(input);

    ranges
        .into_iter()
        .map(|r| r.count())
        .sum::<usize>()
        .to_string()
}

fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut lines = input.lines().skip_while(|line| line.is_empty());
    let lines = lines.by_ref();

    let ranges = lines
        .take_while(|line| !line.is_empty())
        .filter_map(|line| line.split_once('-'))
        .filter_map(|(a, b)| a.parse::<u64>().ok().zip(b.parse::<u64>().ok()))
        .map(|(a, b)| (a..=b))
        .collect_vec();

    let ranges = reduce_ranges(ranges);

    let ingredients = lines
        .filter_map(|line| line.parse::<u64>().ok())
        .collect_vec();

    (ranges, ingredients)
}

fn reduce_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut res = vec![];

    while let Some(item) = ranges.pop() {
        match ranges
            .iter_mut()
            .find(|r| r.end() >= item.start() && r.start() <= item.end())
        {
            Some(r) => *r = *r.start().min(item.start())..=*r.end().max(item.end()),
            None => res.push(item),
        }
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
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

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
    #[case(TEST_CASE, "14")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("761")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("345755049374932")]
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
