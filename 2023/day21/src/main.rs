#![feature(test)]

use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use itertools::Itertools;
use num::Integer;

fn main() {
    let input = fs::read_to_string("./day21/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input, 64));
    println!("Part 2 result is: {}", process_part_2(&input, 26501365));
}

fn process_part_1(input: &str, steps: usize) -> String {
    let (field, start) = parse(input);

    calculate_steps(&field, start, steps).to_string()
}

fn process_part_2(input: &str, steps: usize) -> String {
    "".to_owned()
}

fn parse(input: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let mut start = None;

    (
        input
            .trim()
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.char_indices()
                    .map(|(col, c)| {
                        if c == 'S' {
                            start = Some((row, col));
                            return true;
                        }

                        c == '.'
                    })
                    .collect_vec()
            })
            .collect_vec(),
        start.unwrap(),
    )
}

fn calculate_steps(field: &[Vec<bool>], start: (usize, usize), steps: usize) -> usize {
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut queue = VecDeque::new();
    queue.push_back((start, steps));

    let mut total = 0usize;

    while let Some(((row, col), rem)) = queue.pop_front() {
        if rem.is_even() {
            total += 1;
        }

        if rem == 0 {
            continue;
        }

        if row > 0 {
            let up = (row - 1, col);
            if field[row - 1][col] && !visited.contains(&up) {
                queue.push_back((up, rem - 1));
                visited.insert(up);
            }
        }

        let down = (row + 1, col);
        if let Some(r) = field.get(row + 1) {
            if r[col] && !visited.contains(&down) {
                queue.push_back((down, rem - 1));
                visited.insert(down);
            }
        }

        if col > 0 {
            let left = (row, col - 1);
            if field[row][col - 1] && !visited.contains(&left) {
                queue.push_back((left, rem - 1));
                visited.insert(left);
            }
        }

        let right = (row, col + 1);
        if let Some(c) = field[row].get(col + 1) {
            if *c && !visited.contains(&right) {
                queue.push_back((right, rem - 1));
                visited.insert(right);
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[rstest]
    #[case(TEST_CASE, 6, "16")]
    fn part_1_check(#[case] input: &str, #[case] steps: usize, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input, steps));
    }

    #[rstest]
    #[case(TEST_CASE, 6, "16")]
    #[case(TEST_CASE, 10, "50")]
    #[case(TEST_CASE, 50, "1594")]
    #[case(TEST_CASE, 100, "6536")]
    #[case(TEST_CASE, 500, "167004")]
    #[case(TEST_CASE, 1000, "668697")]
    #[case(TEST_CASE, 5000, "16733044")]
    fn part_2_check(#[case] input: &str, #[case] steps: usize, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input, steps));
    }

    #[rstest]
    #[case("3615")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input, 64));
    }

    #[rstest]
    #[case("")]
    fn part_2_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_2(&input, 26501365));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE, 6));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_1(&input, 64));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE, 500));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_2(&input, 26501365));
    }
}
