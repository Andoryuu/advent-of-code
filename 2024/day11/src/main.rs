#![feature(test)]

use std::{collections::HashMap, fs};

use itertools::Itertools;
use num::Integer;

fn main() {
    let input = fs::read_to_string("./day11/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    count_n_blinks(input, 25).to_string()
}

fn process_part_2(input: &str) -> String {
    count_n_blinks(input, 75).to_string()
}

fn count_n_blinks(input: &str, count: u32) -> usize {
    let mut stones = parse(input);

    for _ in 0..count {
        stones = blink(stones);
    }

    stones.into_values().sum::<usize>()
}

fn blink(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut res = HashMap::with_capacity(stones.len() * 2);
    for (stone, count) in stones {
        if stone == 0 {
            res.entry(1).and_modify(|c| *c += count).or_insert(count);
            continue;
        }

        let width = stone.ilog10() + 1;
        if width.is_even() {
            let (l, r) = stone.div_rem(&10usize.pow(width / 2));
            res.entry(l).and_modify(|c| *c += count).or_insert(count);
            res.entry(r).and_modify(|c| *c += count).or_insert(count);
        } else {
            res.entry(stone * 2024)
                .and_modify(|c| *c += count)
                .or_insert(count);
        }
    }
    res
}

fn parse(input: &str) -> HashMap<usize, usize> {
    HashMap::from_iter(
        input
            .lines()
            .find(|line| !line.is_empty())
            .map(|line| {
                line.split_ascii_whitespace()
                    .flat_map(|p| p.parse::<usize>().ok())
                    .sorted()
                    .chunk_by(|&p| p)
                    .into_iter()
                    .map(|(key, group)| (key, group.count()))
                    .collect_vec()
            })
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "125 17";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "55312")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "65601038650482")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("199986")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("236804088748754")]
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
