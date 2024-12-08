#![feature(test)]

mod antinodes_iterator;

use std::{collections::HashMap, fs};

use antinodes_iterator::AntinodesIterator;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day08/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    get_antinodes_count(input, false).to_string()
}

fn process_part_2(input: &str) -> String {
    get_antinodes_count(input, true).to_string()
}

fn get_antinodes_count(input: &str, use_harmonics: bool) -> usize {
    let ParseResult { antennas, size } = parse(input);

    antennas
        .into_iter()
        .flat_map(|(_, values)| {
            values
                .into_iter()
                .tuple_combinations()
                .flat_map(|(a, b)| AntinodesIterator::new(a, b, size, use_harmonics))
        })
        .unique()
        .count()
}

struct ParseResult {
    antennas: HashMap<char, Vec<(isize, isize)>>,
    size: (isize, isize),
}

fn parse(input: &str) -> ParseResult {
    let max_row = input.lines().filter(|line| !line.is_empty()).count();
    let max_col = input.lines().find(|line| !line.is_empty()).unwrap().len();

    ParseResult {
        antennas: HashMap::from_iter(
            input
                .lines()
                .filter(|line| !line.is_empty())
                .enumerate()
                .flat_map(|(row, line)| {
                    line.char_indices()
                        .filter(|(_, ch)| *ch != '.')
                        .map(move |(col, ch)| (ch, (row as isize, col as isize)))
                })
                .into_group_map(),
        ),
        size: (max_row as isize, max_col as isize),
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "14")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "34")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("357")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("1266")]
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
