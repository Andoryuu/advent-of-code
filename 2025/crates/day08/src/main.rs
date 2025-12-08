#![feature(test)]

use std::fs;

use itertools::Itertools;
use num::integer::Roots;
use partitions::PartitionVec;

fn main() {
    let input = fs::read_to_string("./crates/day08/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input, 1_000));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str, count: usize) -> String {
    let junctions: Vec<Junction> = input
        .lines()
        .filter_map(|line| line.try_into().ok())
        .collect_vec();

    let mut sets = PartitionVec::from_iter(junctions.iter());

    junctions
        .iter()
        .enumerate()
        .tuple_combinations()
        .sorted_by_key(|(a, b)| a.1.distance(b.1))
        .take(count)
        .for_each(|(a, b)| sets.union(a.0, b.0));

    sets.all_sets()
        .map(|s| s.count())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let junctions: Vec<Junction> = input
        .lines()
        .filter_map(|line| line.try_into().ok())
        .collect_vec();

    let mut sets = PartitionVec::from_iter(junctions.iter());

    for (a, b) in junctions
        .iter()
        .enumerate()
        .tuple_combinations()
        .sorted_by_key(|(a, b)| a.1.distance(b.1))
    {
        sets.union(a.0, b.0);

        if sets.amount_of_sets() == 1 {
            return (a.1.x * b.1.x).to_string();
        }
    }

    unreachable!()
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Junction {
    x: usize,
    y: usize,
    z: usize,
}

impl TryFrom<&str> for Junction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .split(',')
            .filter_map(|p| p.parse().ok())
            .next_tuple()
            .map(|(x, y, z)| Junction { x, y, z })
            .ok_or(value.to_owned())
    }
}

impl Junction {
    fn distance(&self, other: &Junction) -> usize {
        let x = self.x.abs_diff(other.x);
        let y = self.y.abs_diff(other.y);
        let z = self.z.abs_diff(other.z);

        (x * x + y * y + z * z).sqrt()
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "40")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input, 10));
    }

    #[rstest]
    #[case(TEST_CASE, "25272")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("181584")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input, 1_000));
    }

    #[rstest]
    #[case("8465902405")]
    fn part_2_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(&input));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE, 10));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_1(&input, 1_000));
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
