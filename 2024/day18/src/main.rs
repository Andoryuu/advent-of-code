#![feature(test)]

use std::{cmp::Reverse, collections::BinaryHeap, fs};

use fxhash::FxHashSet;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day18/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input, 1024, 70));
    println!("Part 2 result is: {}", process_part_2(&input, 1024, 70));
}

fn process_part_1(input: &str, limit: usize, max: isize) -> String {
    let corrupted: FxHashSet<(_, _)> = input
        .lines()
        .filter_map(|line| {
            line.split(',')
                .filter_map(|p| p.parse::<isize>().ok())
                .collect_tuple()
        })
        .take(limit)
        .collect();

    try_find_path(&corrupted, max).unwrap().to_string()
}

fn process_part_2(input: &str, start: usize, max: isize) -> String {
    let mut bytes = input.lines().filter_map(|line| {
        line.split(',')
            .filter_map(|p| p.parse::<isize>().ok())
            .collect_tuple()
    });
    let bytes = bytes.by_ref();

    let mut base_corrupted = FxHashSet::default();
    for byte in bytes.take(start) {
        base_corrupted.insert(byte);
    }

    let bytes = bytes.collect_vec();

    // we could remember the previous path and reroute on hit
    // but let's just do a binary search
    let mut target = bytes.len() / 2;
    let mut offset = target;
    let mut temp_corrupted = FxHashSet::default();
    let mut highest_found = 0;

    loop {
        offset = 1.max(offset / 2);
        temp_corrupted.clear();
        base_corrupted.clone_into(&mut temp_corrupted);
        for &byte in bytes.iter().take(target) {
            temp_corrupted.insert(byte);
        }

        if try_find_path(&temp_corrupted, max).is_some() {
            highest_found = highest_found.max(target);
            target += offset;
        } else if target == highest_found + 1 {
            let byte = bytes[target - 1];
            return format!("{},{}", byte.0, byte.1);
        } else {
            target -= offset;
        }
    }
}

fn try_find_path(corrupted: &FxHashSet<(isize, isize)>, max: isize) -> Option<usize> {
    let mut visited = FxHashSet::default();
    let mut queue = BinaryHeap::from([Reverse((0, (0, 0)))]);

    while let Some(Reverse((score, coords))) = queue.pop() {
        if coords.0 == max && coords.1 == max {
            return Some(score);
        }

        if visited.contains(&coords) {
            continue;
        }

        visited.insert(coords);

        for n in [
            (coords.0 - 1, coords.1),
            (coords.0 + 1, coords.1),
            (coords.0, coords.1 - 1),
            (coords.0, coords.1 + 1),
        ] {
            if (0..=max).contains(&n.0) && (0..=max).contains(&n.1) && !corrupted.contains(&n) {
                queue.push(Reverse((score + 1, n)));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "22")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input, 12, 6));
    }

    #[rstest]
    #[case(TEST_CASE, "6,1")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input, 12, 6));
    }

    #[rstest]
    #[case("278")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input, 1024, 70));
    }

    #[rstest]
    #[case("43,12")]
    fn part_2_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(&input, 1024, 70));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE, 12, 6));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_1(&input, 1024, 70));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE, 12, 6));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_2(&input, 1024, 70));
    }
}
