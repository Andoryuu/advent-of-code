#![feature(test)]

use std::{cmp::Ordering, collections::HashSet, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day05/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let (rules, updates) = parse(input);

    updates
        .into_iter()
        .filter(|update| is_ordered(update, &rules))
        .filter_map(|update| update.get(update.len() / 2).copied())
        .sum::<usize>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let (rules, updates) = parse(input);

    updates
        .into_iter()
        .filter(|update| !is_ordered(update, &rules))
        .filter_map(|mut update| {
            update.sort_by(|&a, &b| {
                if rules.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            update.get(update.len() / 2).copied()
        })
        .sum::<usize>()
        .to_string()
}

fn is_ordered(update: &[usize], rules: &HashSet<(usize, usize)>) -> bool {
    update.iter().enumerate().all(|(i, &curr)| {
        update
            .iter()
            .take(i)
            .all(|&prev| !rules.contains(&(curr, prev)))
    })
}

fn parse(input: &str) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    let mut iterator = input.lines();
    (
        HashSet::from_iter(
            iterator
                .by_ref()
                .skip_while(|line| line.is_empty())
                .take_while(|line| !line.is_empty())
                .filter_map(|line| {
                    line.split('|')
                        .filter_map(|p| p.parse::<usize>().ok())
                        .collect_tuple()
                }),
        ),
        iterator
            .take_while(|line| !line.is_empty())
            .map(|line| {
                line.split(',')
                    .filter_map(|p| p.parse::<usize>().ok())
                    .collect_vec()
            })
            .collect_vec(),
    )
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "143")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "123")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("5091")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("4681")]
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
