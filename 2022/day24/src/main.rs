#![feature(test)]

use std::{
    collections::HashSet,
    fs,
};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let field = parse(input);
    let (rows, cols) = field.size;
    let start = (0, 0);
    let target = (rows - 1, cols - 1);
    let mut minute = 0usize;
    let mut positions = Vec::new();

    'l: loop {
        minute += 1;

        if positions.is_empty() {
            if field.is_empty(start, minute) {
                positions.push(start);
            }

            continue;
        }

        let mut candidates = Vec::new();
        for pos in positions.iter() {
            for n in get_neighbs(pos, rows, cols) {
                if field.is_empty(n, minute) {
                    if n == target {
                        break 'l;
                    }
                    candidates.push(n);
                }
            }
            if field.is_empty(*pos, minute) {
                candidates.push(*pos);
            }
        }

        candidates.sort();
        candidates.dedup();

        positions = candidates;
    }

    (minute + 1).to_string()
}

fn process_data_adv(input: String) -> String {
    let field = parse(input);
    let (rows, cols) = field.size;
    let start = (0, 0);
    let target = (rows - 1, cols - 1);
    let mut minute = 0usize;
    let mut positions = Vec::new();
    let mut phase = 0;

    'l: loop {
        minute += 1;

        if positions.is_empty() {
            match phase {
                0 | 2 => {
                    if field.is_empty(start, minute) {
                        positions.push(start);
                    }
                }
                1 => {
                    if field.is_empty(target, minute) {
                        positions.push(target);
                    }
                }
                _ => {}
            }

            continue;
        }

        let mut candidates = Vec::new();
        for pos in positions.iter() {
            for n in get_neighbs(pos, rows, cols) {
                if field.is_empty(n, minute) {
                    match phase {
                        0 => {
                            if n == target {
                                positions.clear();
                                minute += 1;
                                phase = 1;
                                continue 'l;
                            }
                        }
                        1 => {
                            if n == start {
                                positions.clear();
                                minute += 1;
                                phase = 2;
                                continue 'l;
                            }
                        }
                        2 => {
                            if n == target {
                                break 'l;
                            }
                        }
                        _ => {}
                    }

                    candidates.push(n);
                }
            }

            if field.is_empty(*pos, minute) {
                candidates.push(*pos);
            }
        }

        candidates.sort();
        candidates.dedup();

        positions = candidates;
    }

    (minute + 1).to_string()
}

fn parse(input: String) -> Field {
    let blizzes = input
        .lines()
        .skip(1)
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .skip(1)
                .enumerate()
                .filter_map(|(col, ch)| match ch {
                    '>' => Some(Blizz::Right(row, col)),
                    '^' => Some(Blizz::Up(row, col)),
                    'v' => Some(Blizz::Down(row, col)),
                    '<' => Some(Blizz::Left(row, col)),
                    _ => None,
                })
                .collect_vec()
        })
        .collect_vec();

    let rows = input.lines().count() - 2;
    let cols = input.lines().next().unwrap().len() - 2;

    Field {
        size: (rows, cols),
        up: HashSet::from_iter(blizzes.iter().filter_map(|b| {
            if let Blizz::Up(row, col) = b {
                Some((*row, *col))
            } else {
                None
            }
        })),
        right: HashSet::from_iter(blizzes.iter().filter_map(|b| {
            if let Blizz::Right(row, col) = b {
                Some((*row, *col))
            } else {
                None
            }
        })),
        down: HashSet::from_iter(blizzes.iter().filter_map(|b| {
            if let Blizz::Down(row, col) = b {
                Some((*row, *col))
            } else {
                None
            }
        })),
        left: HashSet::from_iter(blizzes.iter().filter_map(|b| {
            if let Blizz::Left(row, col) = b {
                Some((*row, *col))
            } else {
                None
            }
        })),
    }
}

fn get_neighbs((row, col): &(usize, usize), max_row: usize, max_col: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let row = *row;
    let col = *col;

    if row > 0 {
        res.push((row - 1, col));
    }
    if col > 0 {
        res.push((row, col - 1));
    }
    if row < (max_row - 1) {
        res.push((row + 1, col));
    }
    if col < (max_col - 1) {
        res.push((row, col + 1));
    }

    res
}

enum Blizz {
    Up(usize, usize),
    Right(usize, usize),
    Down(usize, usize),
    Left(usize, usize),
}

#[derive(Debug)]
struct Field {
    size: (usize, usize),
    up: HashSet<(usize, usize)>,
    right: HashSet<(usize, usize)>,
    down: HashSet<(usize, usize)>,
    left: HashSet<(usize, usize)>,
}

impl Field {
    fn is_empty(&self, (row, col): (usize, usize), minute: usize) -> bool {
        let (rows, cols) = self.size;

        !self.up.contains(&((row + minute) % rows, col))
            && !self
                .right
                .contains(&(row, (col + cols - (minute % cols)) % cols))
            && !self
                .down
                .contains(&((row + rows - (minute % rows)) % rows, col))
            && !self.left.contains(&(row, (col + minute) % cols))
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    #[rstest]
    #[case(TEST_CASE, "18")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned()));
    }

    #[rstest]
    #[case(TEST_CASE, "54")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned()));
    }

    #[bench]
    fn base_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_data(input.to_owned()));
    }
}
