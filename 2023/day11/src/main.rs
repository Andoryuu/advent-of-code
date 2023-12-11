#![feature(cmp_minmax)]
#![feature(test)]

use std::{cmp, collections::HashSet, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day11/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input, 1_000_000));
}

fn process_part_1(input: &str) -> String {
    process(input, 2)
}

fn process_part_2(input: &str, expand_ratio: isize) -> String {
    process(input, expand_ratio)
}

fn process(input: &str, expand_ratio: isize) -> String {
    let field = input
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices().filter_map(move |(col, c)| {
                if c == '#' {
                    Some((row as isize, col as isize))
                } else {
                    None
                }
            })
        })
        .collect_vec();

    let rows = input.lines().filter(|l| !l.is_empty()).count() as isize;
    let cols = input.lines().find(|l| !l.is_empty()).unwrap().len() as isize;
    let mut empty_rows = HashSet::<isize>::from_iter(0..rows);
    let mut empty_cols = HashSet::<isize>::from_iter(0..cols);

    for (row, col) in field.iter() {
        empty_rows.remove(row);
        empty_cols.remove(col);
    }

    let empty_rows = empty_rows.into_iter().sorted().collect_vec();
    let empty_cols = empty_cols.into_iter().sorted().collect_vec();

    field
        .iter()
        .enumerate()
        .flat_map(|(i, v1)| field.iter().skip(i + 1).map(|v2| (*v1, *v2)))
        .map(|(v1, v2)| get_dist(v1, v2, &empty_rows, &empty_cols, expand_ratio))
        .sum::<isize>()
        .to_string()
}

fn get_dist(
    v1: (isize, isize),
    v2: (isize, isize),
    empty_rows: &[isize],
    empty_cols: &[isize],
    expand_ratio: isize,
) -> isize {
    let (row1, col1) = v1;
    let (row2, col2) = v2;
    let [row1, row2] = cmp::minmax(row1, row2);
    let [col1, col2] = cmp::minmax(col1, col2);

    let r_dist = row2 - row1
        + (expand_ratio - 1)
            * (empty_rows
                .iter()
                .skip_while(|r| **r < row1)
                .take_while(|r| **r < row2)
                .count() as isize);

    let c_dist = col2 - col1
        + (expand_ratio - 1)
            * (empty_cols
                .iter()
                .skip_while(|r| **r < col1)
                .take_while(|r| **r < col2)
                .count() as isize);

    r_dist + c_dist
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[rstest]
    #[case(TEST_CASE, "374")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, 10, "1030")]
    #[case(TEST_CASE, 100, "8410")]
    fn part_2_check(#[case] input: &str, #[case] expand_ratio: isize, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input, expand_ratio));
    }

    #[rstest]
    #[case("9681886")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("791134099634")]
    fn part_2_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_2(&input, 1_000_000));
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
        b.iter(|| process_part_2(TEST_CASE, 100));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_2(&input, 1_000_000));
    }
}
