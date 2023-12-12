#![feature(test)]

use std::fs;

use cached::proc_macro::cached;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = fs::read_to_string("./day12/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .collect_vec()
        .par_iter()
        .map(|line| line.split_ascii_whitespace().next_tuple().unwrap())
        .map(|(pat, nums)| {
            generate_poss(
                nums.split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_vec(),
                pat.chars().map_into().collect_vec(),
            )
        })
        .sum::<usize>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .collect_vec()
        .par_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .next_tuple()
                .map(|(pat, nums)| ([pat].repeat(5).join("?"), [nums].repeat(5).join(",")))
                .unwrap()
        })
        .map(|(pat, nums)| {
            generate_poss(
                nums.split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_vec(),
                pat.chars().map_into().collect_vec(),
            )
        })
        .sum::<usize>()
        .to_string()
}

#[cached]
fn generate_poss(nums: Vec<usize>, filter: Vec<Spring>) -> usize {
    if let Some((head, tail)) = nums.split_first() {
        let rem = tail.iter().sum::<usize>() + tail.len();
        let len = filter.len();
        let max = len - rem - head;

        if tail.is_empty() {
            (0..=max)
                .filter(|i| matches_filter(*i, *head, len - i - head, &filter))
                .count()
        } else {
            (0..=max)
                .filter(|i| (i + head + rem) <= len)
                .filter(|i| matches_filter(*i, *head, 1, &filter))
                .collect_vec()
                .par_iter()
                .map(|i| generate_poss(tail.to_owned(), filter[(i + head + 1)..].to_owned()))
                .sum::<usize>()
        }
    } else {
        0
    }
}

fn matches_filter(prefix: usize, group: usize, suffix: usize, filter: &[Spring]) -> bool {
    [
        vec![Spring::Operational; prefix],
        vec![Spring::Damaged; group],
        vec![Spring::Operational; suffix],
    ]
    .concat()
    .into_iter()
    .zip(filter)
    .all(|(a, b)| *b == Spring::Unknown || *b == a)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '?' => Spring::Unknown,
            '#' => Spring::Damaged,
            '.' => Spring::Operational,
            x => panic!("unknown char '{:?}'", x),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[rstest]
    #[case(TEST_CASE, "21")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "525152")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("7017")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("527570479489")]
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
