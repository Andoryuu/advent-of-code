#![feature(test)]

use std::fs;

use itertools::Itertools;
use rustc_hash::FxHashSet;

fn main() {
    let input = fs::read_to_string("./crates/day12/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
}

fn process_part_1(input: &str) -> String {
    let Data { shapes, fields } = parse(input);

    // turns out input is actually nice...
    fields.len().to_string()
}

struct Data {
    shapes: Vec<FxHashSet<(usize, usize)>>,
    fields: Vec<((usize, usize), Vec<usize>)>,
}

fn parse(input: &str) -> Data {
    let shapes = input
        .lines()
        .take(30)
        .chunks(5)
        .into_iter()
        .map(|chunk| {
            chunk
                .skip(1)
                .take(3)
                .enumerate()
                .flat_map(|(row, line)| {
                    line.char_indices()
                        .filter_map(move |(col, c)| if c == '#' { Some((row, col)) } else { None })
                })
                .collect()
        })
        .collect_vec();

    let fields = input
        .lines()
        .skip(30)
        .filter_map(|line| line.split_once(':'))
        .map(|(s, p)| {
            (
                s.split_once('x')
                    .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                    .unwrap(),
                p.split_ascii_whitespace()
                    .filter_map(|part| part.parse::<usize>().ok())
                    .collect_vec(),
            )
        })
        // filter out trivial solutions
        .filter(|((x, y), parts)| (x * y) >= parts.iter().map(|a| a * 9).sum())
        .collect_vec();

    Data { shapes, fields }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "2")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case("433")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
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
}
