#![feature(test)]

use std::fs;

use fancy_regex::{Captures, Error, Regex};

fn main() {
    let input = fs::read_to_string("./day02/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let gamergx = Regex::new(r"Game (\d+):").unwrap();
    let redrgx = Regex::new(r" (\d+) red").unwrap();
    let greenrgx = Regex::new(r" (\d+) green").unwrap();
    let bluergx = Regex::new(r" (\d+) blue").unwrap();

    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }

            if let Some(rs) = redrgx
                .captures_iter(line)
                .filter_map(capture_to_number)
                .max()
            {
                if rs > 12 {
                    return None;
                }
            }

            if let Some(gs) = greenrgx
                .captures_iter(line)
                .filter_map(capture_to_number)
                .max()
            {
                if gs > 13 {
                    return None;
                }
            }

            if let Some(bs) = bluergx
                .captures_iter(line)
                .filter_map(capture_to_number)
                .max()
            {
                if bs > 14 {
                    return None;
                }
            }

            gamergx
                .captures_iter(line)
                .next()
                .and_then(capture_to_number)
        })
        .sum::<u32>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let redrgx = Regex::new(r" (\d+) red").unwrap();
    let greenrgx = Regex::new(r" (\d+) green").unwrap();
    let bluergx = Regex::new(r" (\d+) blue").unwrap();

    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }

            let reds = redrgx
                .captures_iter(line)
                .filter_map(capture_to_number)
                .max()
                .unwrap_or(0);

            let greens = greenrgx
                .captures_iter(line)
                .filter_map(capture_to_number)
                .max()
                .unwrap_or(0);

            let blues = bluergx
                .captures_iter(line)
                .filter_map(capture_to_number)
                .max()
                .unwrap_or(0);

            Some(reds * blues * greens)
        })
        .sum::<u32>()
        .to_string()
}

fn capture_to_number(cap: Result<Captures<'_>, Error>) -> Option<u32> {
    cap.ok()
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok())
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[rstest]
    #[case(TEST_CASE, "8")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "2286")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("2476")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("54911")]
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
