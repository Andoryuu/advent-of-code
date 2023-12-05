#![feature(test)]

use std::{collections::HashSet, fs, ops::Range};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day05/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let seeds = input
        .lines()
        .find(|line| line.starts_with("seeds:"))
        .and_then(|line| line.split(':').nth(1))
        .map(|nums| {
            nums.split_ascii_whitespace()
                .filter_map(|n| n.parse::<isize>().ok())
                .collect_vec()
        })
        .unwrap();

    parse_maps(input)
        .iter()
        .fold(seeds, |acc, map| {
            acc.iter().map(|s| get_dest(*s, map)).collect_vec()
        })
        .iter()
        .min()
        .unwrap()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let seeds = input
        .lines()
        .find(|line| line.starts_with("seeds:"))
        .and_then(|line| line.split(':').nth(1))
        .map(|nums| {
            nums.split_ascii_whitespace()
                .filter_map(|n| n.parse::<isize>().ok())
                .tuples()
                .map(|(start, len)| start..(start + len))
                .collect_vec()
        })
        .unwrap();

    parse_maps(input)
        .iter()
        .fold(seeds, |acc, map| {
            acc.iter()
                .flat_map(|r| get_range_dest(r.clone(), map))
                .collect_vec()
        })
        .iter()
        .map(|r| r.start)
        .min()
        .unwrap()
        .to_string()
}

fn get_dest(source: isize, map: &[(Range<isize>, isize)]) -> isize {
    map.iter()
        .find(|(r, _)| r.contains(&source))
        .map(|(r, d)| source + d - r.start)
        .unwrap_or(source)
}

fn get_range_dest(source: Range<isize>, map: &[(Range<isize>, isize)]) -> Vec<Range<isize>> {
    let mut res = Vec::<Range<isize>>::new();
    let mut ranges = HashSet::<Range<isize>>::new();
    ranges.insert(source.clone());

    for (r, d) in map {
        let shift = d - r.start;

        for range in ranges.clone() {
            if r.end <= range.start || r.start >= range.end {
                continue;
            }

            ranges.remove(&range);
            res.push((r.start.max(range.start) + shift)..(r.end.min(range.end) + shift));

            if r.end < range.end {
                ranges.insert(r.end..range.end);
            }

            if r.start > range.start {
                ranges.insert(range.start..r.start);
            }
        }
    }

    res.extend(ranges);
    res
}

fn parse_maps(input: &str) -> Vec<Vec<(Range<isize>, isize)>> {
    vec![
        parse_map(input, "seed-to-soil"),
        parse_map(input, "soil-to-fertilizer"),
        parse_map(input, "fertilizer-to-water"),
        parse_map(input, "water-to-light"),
        parse_map(input, "light-to-temperature"),
        parse_map(input, "temperature-to-humidity"),
        parse_map(input, "humidity-to-location"),
    ]
}

fn parse_map(input: &str, cat: &str) -> Vec<(Range<isize>, isize)> {
    input
        .lines()
        .skip_while(|line| !line.starts_with(cat))
        .skip(1)
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|n| n.parse::<isize>().ok())
                .collect_tuple()
        })
        .take_while(|o| o.is_some())
        .flatten()
        .map(|(dest, source, len)| (source..(source + len), dest))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[rstest]
    #[case(TEST_CASE, "35")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "46")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("278755257")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("26829166")]
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
