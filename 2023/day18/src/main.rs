#![feature(test)]

use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day18/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let parser = |line: &str| {
        line.split_ascii_whitespace()
            .next_tuple()
            .map(|(d, n)| (d.into(), n.parse::<isize>().unwrap()))
            .unwrap()
    };

    process(input, parser).to_string()
}

fn process_part_2(input: &str) -> String {
    let parser = |line: &str| {
        line.split('#')
            .nth(1)
            .map(|s| {
                (
                    s[5..6].parse::<u32>().unwrap().into(),
                    isize::from_str_radix(&s[..5], 16).unwrap(),
                )
            })
            .unwrap()
    };

    process(input, parser).to_string()
}

fn process(input: &str, parser: fn(&str) -> (Direction, isize)) -> isize {
    let (poly, sum, _, _) = input.trim().lines().map(parser).fold(
        (vec![], 0isize, 0isize, 0isize),
        |(mut res, sum, mut row, mut col), (dir, num)| {
            match dir {
                Direction::Up => row -= num,
                Direction::Left => col -= num,
                Direction::Down => row += num,
                Direction::Right => col += num,
            };

            res.push((row, col));

            (res, sum + num, row, col)
        },
    );

    // TIL Pick's theorem
    shoelace(poly).abs() + (sum / 2) + 1
}

fn shoelace<T: num::Num + num::Signed + num::NumCast + std::clone::Clone>(poly: Vec<(T, T)>) -> T {
    let poly_size = poly.len();

    poly.into_iter()
        .cycle()
        .tuple_windows()
        .take(poly_size)
        .fold(T::zero(), |acc, (a, b)| acc + (a.0 * b.1) - (a.1 * b.0))
        .div(T::from(2).unwrap())
}

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Direction::Up,
            "L" => Direction::Left,
            "D" => Direction::Down,
            "R" => Direction::Right,
            x => panic!("unknown direction: {}", x),
        }
    }
}

impl From<u32> for Direction {
    fn from(value: u32) -> Self {
        match value {
            3 => Direction::Up,
            2 => Direction::Left,
            1 => Direction::Down,
            0 => Direction::Right,
            x => panic!("unknown direction: {}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[rstest]
    #[case(TEST_CASE, "62")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "952408144115")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("48503")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("148442153147147")]
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
