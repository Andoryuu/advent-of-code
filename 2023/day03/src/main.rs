#![feature(test)]

use std::{collections::HashMap, fs, ops::Range};

use fancy_regex::Regex;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day03/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let schema = parse_schematic(input);

    schema
        .numbers
        .iter()
        .map(|(n, _)| *n)
        .sum::<u32>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let schema = parse_schematic(input);

    schema
        .numbers
        .iter()
        .sorted_by_key(|(_, s)| s)
        .group_by(|(_, s)| s)
        .into_iter()
        .filter_map(|(key, group)| {
            schema.symbols.get(key).and_then(|v| {
                if *v != '*' {
                    return None;
                }

                let ns = group.map(|(n, _)| *n).collect_vec();
                if ns.len() == 2 {
                    Some(ns.iter().product::<u32>())
                } else {
                    None
                }
            })
        })
        .sum::<u32>()
        .to_string()
}

fn parse_schematic(input: &str) -> Schema {
    let width = input
        .lines()
        .find(|line| !line.is_empty())
        .map(|line| line.len() as isize)
        .unwrap();

    let mut symbols = HashMap::<isize, char>::new();

    let numrgx = Regex::new(r"(?:^|[^\d])(\d+)(?=$|[^\d])").unwrap();

    let numbers = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            let row = row as isize;

            for (col, c) in line.chars().enumerate() {
                if !(c == '.' || c.is_ascii_digit()) {
                    symbols.insert(row * width + col as isize, c);
                }
            }

            numrgx.captures_iter(line).filter_map(move |r| {
                r.ok().and_then(|c| c.get(1)).map(|m| Number {
                    val: m.as_str().parse::<u32>().unwrap(),
                    pos: (row * width + m.range().start as isize)
                        ..(row * width + m.range().end as isize),
                })
            })
        })
        // collect because we need to search in mutated symbols map
        .collect_vec()
        .iter()
        .filter_map(|n| {
            n.get_neighs(width)
                .iter()
                .find(|p| symbols.contains_key(p))
                .map(|s| (n.val, *s))
        })
        .collect_vec();

    Schema { symbols, numbers }
}

struct Schema {
    symbols: HashMap<isize, char>,
    numbers: Vec<(u32, isize)>,
}

struct Number {
    val: u32,
    pos: Range<isize>,
}

impl Number {
    fn get_neighs(&self, width: isize) -> Vec<isize> {
        let mut r = self
            .pos
            .clone()
            .flat_map(|col| vec![col - width, col + width])
            .collect_vec();

        if (self.pos.start % width) != 0 {
            r.append(&mut vec![
                self.pos.start - 1 - width,
                self.pos.start - 1,
                self.pos.start - 1 + width,
            ]);
        }

        if (self.pos.end % width) != 0 {
            r.append(&mut vec![
                self.pos.end - width,
                self.pos.end,
                self.pos.end + width,
            ]);
        }

        r
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[rstest]
    #[case(TEST_CASE, "4361")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "467835")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("522726")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("81721933")]
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
