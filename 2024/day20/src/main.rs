#![feature(test)]

use std::fs;

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use num::Integer;

fn main() {
    let input = fs::read_to_string("./day20/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input, 100));
    println!("Part 2 result is: {}", process_part_2(&input, 100));
}

fn process_part_1(input: &str, to_save: isize) -> String {
    let track = parse(input);
    let mut worth_save = 0;

    for (coords, curr_v) in track.iter() {
        for (n, n2) in [
            ((coords.0 - 1, coords.1), (coords.0 - 2, coords.1)),
            ((coords.0 + 1, coords.1), (coords.0 + 2, coords.1)),
            ((coords.0, coords.1 - 1), (coords.0, coords.1 - 2)),
            ((coords.0, coords.1 + 1), (coords.0, coords.1 + 2)),
        ] {
            if !track.contains_key(&n) {
                if let Some(new_v) = track.get(&n2) {
                    if new_v - curr_v - 2 >= to_save {
                        worth_save.inc();
                    }
                }
            }
        }
    }

    worth_save.to_string()
}

fn process_part_2(input: &str, to_save: isize) -> String {
    parse(input)
        .into_iter()
        .sorted_by_key(|t| t.1)
        .tuple_combinations()
        .filter(|((from_c, from_v), (to_c, to_v))| {
            let dist = (from_c.0.abs_diff(to_c.0) + from_c.1.abs_diff(to_c.1)) as isize;
            dist <= 20 && to_v - from_v - dist >= to_save
        })
        .count()
        .to_string()
}

fn parse(input: &str) -> FxHashMap<(isize, isize), isize> {
    let mut start = None;
    let mut end = None;
    let mut track = FxHashSet::default();

    for (coords, ch) in input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .map(move |(col, ch)| ((row as isize, col as isize), ch))
        })
    {
        match ch {
            'S' => {
                track.insert(coords);
                start = Some(coords);
            }
            'E' => {
                track.insert(coords);
                end = Some(coords);
            }
            '.' => {
                track.insert(coords);
            }
            _ => {}
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();

    let mut pos = start;
    let mut value = 0;

    let mut visited = FxHashSet::default();
    visited.insert(pos);

    let mut mapped_track = FxHashMap::default();
    mapped_track.insert(pos, value);

    while pos != end {
        for n in [
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ] {
            if !visited.contains(&n) && track.contains(&n) {
                pos = n;
                value.inc();
                visited.insert(pos);
                mapped_track.insert(pos, value);
            }
        }
    }

    mapped_track
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "10")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input, 10));
    }

    #[rstest]
    #[case(TEST_CASE, "285")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input, 50));
    }

    #[rstest]
    #[case("1381")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input, 100));
    }

    #[rstest]
    #[case("982124")]
    fn part_2_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(&input, 100));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE, 10));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_1(&input, 100));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE, 50));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_2(&input, 100));
    }
}
