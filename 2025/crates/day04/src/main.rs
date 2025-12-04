#![feature(test)]

use std::fs;

use rustc_hash::FxHashSet;

fn main() {
    let input = fs::read_to_string("./crates/day04/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    get_accessible(&parse(input)).len().to_string()
}

fn process_part_2(input: &str) -> String {
    let mut rolls = parse(input);
    let mut count = 0;

    loop {
        let removed = get_accessible(&rolls);

        if removed.is_empty() {
            return count.to_string();
        }

        count += removed.len();

        for r in removed {
            rolls.remove(&r);
        }
    }
}

fn get_accessible(rolls: &FxHashSet<(i32, i32)>) -> Vec<(i32, i32)> {
    rolls
        .iter()
        .filter(|(row, col)| {
            (-1..=1)
                .flat_map(|x| (-1..=1).map(move |y| (x, y)))
                .filter(|(x, y)| rolls.contains(&(row + x, col + y)))
                .count()
                <= 4
        })
        .map(|(x, y)| (*x, *y))
        .collect()
}

fn parse(input: &str) -> FxHashSet<(i32, i32)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(ix, line)| {
            line.char_indices().filter_map(move |(cx, c)| {
                if c == '@' {
                    Some((ix as i32, cx as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "13")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "43")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("1320")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("8354")]
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
