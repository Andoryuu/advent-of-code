#![feature(test)]

use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day25/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
}

fn process_part_1(input: &str) -> String {
    let mut keys = Vec::with_capacity(input.len() / 8);
    let mut locks = Vec::with_capacity(input.len() / 8);

    for mut chunk in input
        .lines()
        .filter(|line| !line.is_empty())
        .chunks(7)
        .into_iter()
    {
        let is_key = chunk.next().is_some_and(|s| s == ".....");
        let values = chunk.take(5).fold(vec![0, 0, 0, 0, 0], |mut acc, s| {
            for (a, c) in acc.iter_mut().zip(s.chars()) {
                if c == '#' {
                    *a += 1
                }
            }
            acc
        });
        if is_key {
            keys.push(values);
        } else {
            locks.push(values);
        }
    }

    locks
        .into_iter()
        .flat_map(|lock| {
            keys.iter()
                .map(move |key| lock.iter().zip(key).all(|(l, k)| l + k <= 5))
                .filter(|&x| x)
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "3")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case("3483")]
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
