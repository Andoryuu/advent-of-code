#![feature(test)]

use std::fs;

use num::Integer;

fn main() {
    let input = fs::read_to_string("./crates/day02/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    input
        .lines()
        .find(|line| !line.is_empty())
        .unwrap()
        .split(',')
        .filter_map(|r| r.split_once('-'))
        .flat_map(|(a, b)| get_invalid(a, b))
        .sum::<u64>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    input
        .lines()
        .find(|line| !line.is_empty())
        .unwrap()
        .split(',')
        .filter_map(|r| r.split_once('-'))
        .flat_map(|(a, b)| get_invalid_multiple(a, b))
        .sum::<u64>()
        .to_string()
}

fn get_invalid(from: &str, to: &str) -> Vec<u64> {
    let from = from.parse::<u64>().unwrap();
    let to = to.parse::<u64>().unwrap();

    (from..=to)
        .filter(|i| {
            let size = i.ilog10() + 1;
            test_split(*i, size, 2)
        })
        .collect()
}

fn get_invalid_multiple(from: &str, to: &str) -> Vec<u64> {
    let from = from.parse::<u64>().unwrap();
    let to = to.parse::<u64>().unwrap();

    (from..=to)
        .filter(|i| {
            let size = i.ilog10() + 1;
            (2..=size).any(|p| test_split(*i, size, p))
        })
        .collect()
}

fn test_split(n: u64, size: u32, parts: u32) -> bool {
    if !size.is_multiple_of(parts) {
        return false;
    }

    let splitter = 10u64.pow(size / parts);

    let (mut d, mut r) = n.div_rem(&splitter);

    while d > 0 {
        let (d2, r2) = d.div_rem(&splitter);

        if r != r2 {
            return false;
        }

        d = d2;
        r = r2;
    }

    true
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "1227775554")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case("1188511880", "1188511890", 1188511885)]
    #[case("95", "115", 99)]
    #[case("1", "16", 11)]
    #[case("17", "30", 22)]
    fn get_invalid_check(#[case] from: &str, #[case] to: &str, #[case] expected: u64) {
        assert_eq!(expected, *get_invalid(from, to).first().unwrap());
    }

    #[rstest]
    #[case(TEST_CASE, "4174379265")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("2121212118", "2121212124", 2121212121)]
    #[case("1111111", "1111111", 1111111)]
    fn get_invalid_multiple_check(#[case] from: &str, #[case] to: &str, #[case] expected: u64) {
        assert_eq!(expected, *get_invalid_multiple(from, to).first().unwrap());
    }

    #[rstest]
    #[case("64215794229")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("85513235135")]
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
