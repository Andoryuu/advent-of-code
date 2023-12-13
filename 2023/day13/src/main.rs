#![feature(test)]

use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day13/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    parse(input)
        .into_iter()
        .map(|room| room.get_summary())
        .sum::<usize>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    parse(input)
        .into_iter()
        .map(|room| room.get_smudged_summary())
        .sum::<usize>()
        .to_string()
}

fn parse(input: &str) -> Vec<Room> {
    input
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(empty, _)| !empty)
        .map(|(_, g)| g.collect_vec())
        .map_into()
        .collect_vec()
}

fn find_reflection(room: Vec<String>) -> Option<usize> {
    room.iter()
        .enumerate()
        .tuple_windows()
        .filter(|(a, b)| a.1 == b.1)
        .map(|(a, _)| a.0)
        .find(|c| {
            room.iter()
                .take(*c + 1)
                .rev()
                .zip(room.iter().skip(c + 1))
                .all(|(a, b)| a == b)
        })
        .map(|c| c + 1)
}

fn find_smudged_reflection(room: Vec<String>) -> Option<usize> {
    room.iter()
        .enumerate()
        .tuple_windows()
        .filter(|(a, b)| loose_1_eq(a.1, b.1))
        .map(|(a, _)| a.0)
        .find(|c| {
            room.iter()
                .take(*c + 1)
                .rev()
                .zip(room.iter().skip(c + 1))
                .filter(|(a, b)| a != b)
                .exactly_one()
                .map_or(false, |(a, b)| loose_1_eq(a, b))
        })
        .map(|c| c + 1)
}

fn loose_1_eq(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).filter(|(ac, bc)| ac != bc).count() < 2
}

#[derive(Debug)]
struct Room {
    vert: Vec<String>,
    horz: Vec<String>,
}

impl Room {
    fn get_summary(self) -> usize {
        if let Some(v) = find_reflection(self.vert) {
            v
        } else if let Some(h) = find_reflection(self.horz) {
            h * 100
        } else {
            0
        }
    }

    fn get_smudged_summary(self) -> usize {
        if let Some(v) = find_smudged_reflection(self.vert) {
            v
        } else if let Some(h) = find_smudged_reflection(self.horz) {
            h * 100
        } else {
            0
        }
    }
}

impl From<Vec<&str>> for Room {
    fn from(value: Vec<&str>) -> Self {
        Room {
            vert: value
                .iter()
                .flat_map(|s| s.char_indices().collect_vec())
                .sorted_by_key(|(i, _)| *i)
                .group_by(|(i, _)| *i)
                .into_iter()
                .map(|(_, g)| g.map(|(_, c)| c).collect::<String>())
                .collect_vec(),
            horz: value.into_iter().map_into().collect_vec(),
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
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[rstest]
    #[case(TEST_CASE, "405")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "400")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("36041")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("35915")]
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
