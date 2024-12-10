#![feature(test)]

use std::{
    collections::{HashMap, HashSet},
    fs,
};

use cached::proc_macro::cached;

fn main() {
    let input = fs::read_to_string("./day10/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let ParseResult { trailheads, map } = parse(input);

    trailheads
        .into_iter()
        .map(|head| count_trail_unique(head, 0, &map).len())
        .sum::<usize>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let ParseResult { trailheads, map } = parse(input);

    trailheads
        .into_iter()
        .map(|head| count_trail_all(head, 0, &map))
        .sum::<usize>()
        .to_string()
}

type HeightMap = HashMap<(isize, isize), u32>;

// key map by address to avoid conflicts
// when running multiple tests at once
#[cached(
    key = "(isize, isize, String)",
    convert = r#"{(row, col, format!("{map:p}"))}"#
)]
fn count_trail_unique(
    (row, col): (isize, isize),
    height: u32,
    map: &HeightMap,
) -> HashSet<(isize, isize)> {
    let mut res = HashSet::new();
    for neigh in [
        (row + 1, col),
        (row - 1, col),
        (row, col + 1),
        (row, col - 1),
    ] {
        if let Some(&n_height) = map.get(&neigh) {
            if height + 1 != n_height {
                continue;
            }

            if n_height == 9 {
                res.insert(neigh);
            } else {
                res.extend(&count_trail_unique(neigh, n_height, map));
            }
        }
    }
    res
}

#[cached(
    key = "(isize, isize, String)",
    convert = r#"{(row, col, format!("{map:p}"))}"#
)]
fn count_trail_all((row, col): (isize, isize), height: u32, map: &HeightMap) -> usize {
    let mut res = 0;
    for neigh in [
        (row + 1, col),
        (row - 1, col),
        (row, col + 1),
        (row, col - 1),
    ] {
        if let Some(&n_height) = map.get(&neigh) {
            if height + 1 != n_height {
                continue;
            }

            if n_height == 9 {
                res += 1;
            } else {
                res += &count_trail_all(neigh, n_height, map);
            }
        }
    }
    res
}

struct ParseResult {
    trailheads: Vec<(isize, isize)>,
    map: HeightMap,
}

fn parse(input: &str) -> ParseResult {
    let mut trailheads = vec![];
    let mut map = HashMap::new();

    for (coords, height) in input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices().filter_map(move |(col, ch)| {
                ch.to_digit(10).map(|h| ((row as isize, col as isize), h))
            })
        })
    {
        if height == 0 {
            trailheads.push(coords);
        }

        map.insert(coords, height);
    }

    ParseResult { trailheads, map }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "36")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "81")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("566")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("1324")]
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
