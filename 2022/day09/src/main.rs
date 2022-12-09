use std::{collections::BTreeSet, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let moves = input.lines().map(Move::from);
    let mut visited = BTreeSet::<Pos>::new();
    let mut head = Pos::new();
    let mut tail = Pos::new();

    visited.insert(tail);

    for step in moves {
        for sub_step in step.get_partial_steps(head) {
            head = sub_step;
            tail = drag_tail(tail, head);
            visited.insert(tail);
        }
    }

    visited.len().to_string()
}

fn process_data_adv(input: String) -> String {
    let moves = input.lines().map(Move::from);
    let mut visited = BTreeSet::<Pos>::new();
    let mut head = Pos::new();
    let mut parts = vec![Pos::new(); 8];
    let mut tail = Pos::new();

    visited.insert(tail);

    for step in moves {
        for sub_step in step.get_partial_steps(head) {
            head = sub_step;

            let mut new_parts = vec![];
            let mut prev_part = head;
            for part in parts {
                prev_part = drag_tail(part, prev_part);
                new_parts.push(prev_part);
            }
            parts = new_parts;

            tail = drag_tail(tail, prev_part);
            visited.insert(tail);
        }
    }

    visited.len().to_string()
}

fn drag_tail(tail: Pos, head: Pos) -> Pos {
    let row_diff = head.row - tail.row;
    let col_diff = head.col - tail.col;

    if row_diff.abs() > 1 || col_diff.abs() > 1 {
        Pos {
            row: tail.row + row_diff.signum(),
            col: tail.col + col_diff.signum(),
        }
    } else {
        tail
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn new() -> Self {
        Pos { row: 0, col: 0 }
    }
}

enum Move {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let parts = value.split_whitespace().next_tuple::<(_, _)>().unwrap();
        let amount = parts.1.parse::<i32>().unwrap();

        match parts.0 {
            "U" => Move::Up(amount),
            "R" => Move::Right(amount),
            "D" => Move::Down(amount),
            "L" => Move::Left(amount),
            _ => panic!("unexpected move: {}", parts.0),
        }
    }
}

impl Move {
    fn get_partial_steps(self, pos: Pos) -> Vec<Pos> {
        match self {
            Move::Up(amount) => (1..=amount)
                .map(|s| Pos {
                    row: pos.row - s,
                    ..pos
                })
                .collect_vec(),
            Move::Right(amount) => (1..=amount)
                .map(|s| Pos {
                    col: pos.col + s,
                    ..pos
                })
                .collect_vec(),
            Move::Down(amount) => (1..=amount)
                .map(|s| Pos {
                    row: pos.row + s,
                    ..pos
                })
                .collect_vec(),
            Move::Left(amount) => (1..=amount)
                .map(|s| Pos {
                    col: pos.col - s,
                    ..pos
                })
                .collect_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const TEST_CASE_2: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[rstest]
    fn base_check() {
        assert_eq!("13", process_data(TEST_CASE.to_string()));
    }

    #[rstest]
    #[case(TEST_CASE, "1")]
    #[case(TEST_CASE_2, "36")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned()));
    }
}
