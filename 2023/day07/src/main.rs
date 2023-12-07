#![feature(test)]

mod joker_hands;
mod regular_hands;

use std::fs;

use itertools::Itertools;
use joker_hands::{JCard, JHand};
use regular_hands::{Card, Hand};

fn main() {
    let input = fs::read_to_string("./day07/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|line| line.split_ascii_whitespace().next_tuple())
        .map(|(h, b)| {
            let cards: Vec<Card> = h.chars().map(|c| c.into()).collect_vec();
            Hand {
                htype: cards.clone().into(),
                cards,
                bid: b.parse::<usize>().unwrap(),
            }
        })
        .sorted()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum::<usize>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|line| line.split_ascii_whitespace().next_tuple())
        .map(|(h, b)| {
            let cards: Vec<JCard> = h.chars().map(|c| c.into()).collect_vec();
            JHand {
                htype: cards.clone().into(),
                cards,
                bid: b.parse::<usize>().unwrap(),
            }
        })
        .sorted()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[rstest]
    #[case(TEST_CASE, "6440")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "5905")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("249204891")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("249666369")]
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
