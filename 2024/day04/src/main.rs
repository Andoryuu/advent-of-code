#![feature(test)]

use std::{collections::HashMap, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day04/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let size = input.lines().find(|line| !line.is_empty()).unwrap().len();

    let mut horz: Vec<String> = vec![];
    let mut vert: Vec<String> = vec![];
    let mut right: HashMap<usize, String> = HashMap::new();
    let mut left: Vec<String> = vec![];

    for (y, line) in input.lines().enumerate() {
        horz.push(line.to_owned());

        for (x, c) in line.char_indices() {
            if let Some(s) = vert.get_mut(x) {
                s.push(c);
            } else {
                vert.push(c.to_string());
            }

            right
                .entry(size + y - x)
                .and_modify(|s| s.push(c))
                .or_insert(c.to_string());

            if let Some(s) = left.get_mut(x + y) {
                s.push(c);
            } else {
                left.push(c.to_string());
            }
        }
    }

    [horz, vert, right.into_values().collect(), left]
        .concat()
        .into_iter()
        .map(|line| {
            line.matches("XMAS").collect_vec().len() + line.matches("SAMX").collect_vec().len()
        })
        .sum::<usize>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let max = input.lines().find(|line| !line.is_empty()).unwrap().len() - 1;
    let field: HashMap<(usize, usize), char> = HashMap::from_iter(
        input
            .lines()
            .filter(|line| !line.is_empty())
            .enumerate()
            .flat_map(|(y, line)| line.char_indices().map(move |(x, c)| ((x, y), c))),
    );
    let mut x_masses = 0;

    for ((x, y), c) in field.iter() {
        if *c != 'A' || !(1..max).contains(x) || !(1..max).contains(y) {
            continue;
        }

        let lu = field[&(x - 1, y - 1)];
        let ru = field[&(x + 1, y - 1)];
        let ld = field[&(x - 1, y + 1)];
        let rd = field[&(x + 1, y + 1)];

        if (lu == 'M' && rd == 'S' || lu == 'S' && rd == 'M')
            && (ru == 'M' && ld == 'S' || ru == 'S' && ld == 'M')
        {
            x_masses += 1;
        }
    }

    x_masses.to_string()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "18")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "9")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("2530")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("1921")]
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
