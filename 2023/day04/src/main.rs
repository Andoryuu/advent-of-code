#![feature(test)]

use std::{collections::HashMap, fs, ops::AddAssign};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day04/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    get_winnings(input)
        .iter()
        .map(|w| {
            if *w > 0 {
                2u32.pow((w - 1).try_into().unwrap())
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let mut temp_counts = HashMap::<usize, usize>::new();
    let mut counts = Vec::<usize>::new();

    for (i, w) in get_winnings(input).iter().enumerate() {
        let e = *temp_counts.get(&i).unwrap_or(&1);
        counts.push(e);

        for j in (i + 1)..=(i + w) {
            temp_counts.entry(j).or_insert(1).add_assign(e);
        }
    }

    counts.iter().sum::<usize>().to_string()
}

fn get_winnings(input: &str) -> Vec<usize> {
    input
        .lines()
        .filter_map(|line| {
            line.split(':')
                .nth(1)
                .and_then(|l| l.split('|').next_tuple::<(&str, &str)>())
        })
        .map(|(win_s, have_s)| {
            let win = parse_numbers(win_s);

            parse_numbers(have_s)
                .iter()
                .filter(|n| win.contains(n))
                .count()
        })
        .collect_vec()
}

fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .chars()
        .group_by(|c| c.is_ascii_digit())
        .into_iter()
        .filter(|(k, _)| *k)
        .filter_map(|(_, g)| g.collect::<String>().parse::<u32>().ok())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[rstest]
    #[case(TEST_CASE, "13")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "30")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("18653")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("5921508")]
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
