#![feature(test)]

use std::{fs, iter};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day22/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| line.parse::<isize>().ok())
        .map(|n| (0..2000).fold(n, |prev, _| get_next_secret(prev)))
        .sum::<isize>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let mut map = FxHashMap::default();
    let mut found = FxHashSet::default();

    for mut n in input.lines().filter_map(|line| line.parse::<isize>().ok()) {
        found.clear();

        for (a, b, c, d, e) in iter::repeat_with(|| {
            n = get_next_secret(n);
            n % 10
        })
        .take(2000)
        .tuple_windows()
        {
            let key = (b - a, c - b, d - c, e - d);
            if !found.contains(&key) {
                found.insert(key);
                map.entry(key).and_modify(|v| *v += e).or_insert(e);
            }
        }
    }

    map.into_values().max().unwrap().to_string()
}

fn get_next_secret(mut n: isize) -> isize {
    n ^= n << 6;
    n &= 16777215;
    n ^= n >> 5;
    n ^= n << 11;
    n &= 16777215;
    n
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE_1: &str = "
1
10
100
2024";

    const TEST_CASE_2: &str = "
1
2
3
2024";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE_1, "37327623")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE_2, "23")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("18941802053")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("2218")]
    fn part_2_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(&input));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE_1));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_1(&input));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE_2));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_2(&input));
    }
}
