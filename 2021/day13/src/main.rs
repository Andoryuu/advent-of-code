use std::{collections::BTreeSet, fs};

use lazy_static::lazy_static;
use regex::{Match, Regex};

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let (mut dots, folds) = parse(input);

    apply_fold(&mut dots, folds.get(0).unwrap());

    dots.sort_unstable();
    dots.dedup();

    dots.len().to_string()
}

fn process_data_adv(input: String) -> String {
    let (mut dots, folds) = parse(input);

    for fold in folds.iter() {
        apply_fold(&mut dots, fold);
    }

    let dot_set = BTreeSet::from_iter(dots.iter());

    println!();

    for y in 0u32..6 {
        let line = (0u32..39)
            .map(|x| if dot_set.contains(&(x, y)) { '#' } else { ' ' })
            .collect::<String>();

        println!("{}", line);
    }

    println!();

    "printed".to_owned()
}

fn apply_fold(dots: &mut Vec<(u32, u32)>, fold: &Fold) {
    match fold {
        Fold::X(x) => apply_fold_x(dots, x),
        Fold::Y(y) => apply_fold_y(dots, y),
    }
}

fn apply_fold_x(dots: &mut Vec<(u32, u32)>, x: &u32) {
    for dot in dots.iter_mut() {
        if dot.0 > *x {
            dot.0 = 2 * x - dot.0
        }
    }
}

fn apply_fold_y(dots: &mut Vec<(u32, u32)>, y: &u32) {
    for dot in dots.iter_mut() {
        if dot.1 > *y {
            dot.1 = 2 * y - dot.1
        }
    }
}

fn parse(input: String) -> (Vec<(u32, u32)>, Vec<Fold>) {
    lazy_static! {
        static ref DOT_RE: Regex = Regex::new("^(\\d+),(\\d+)$").unwrap();
        static ref FOLD_RE: Regex = Regex::new("^fold along ([xy])=(\\d+)").unwrap();
    }

    let mut dots = Vec::new();
    let mut folds = Vec::new();

    for line in input.trim().lines().map(|l| l.trim()) {
        if let Some(dot) = DOT_RE.captures(line) {
            dots.push((
                parse_capture(dot.get(1)).unwrap(),
                parse_capture(dot.get(2)).unwrap(),
            ));
        } else if let Some(fold) = FOLD_RE.captures(line) {
            let direction = fold.get(1).map(|m| m.as_str()).unwrap();
            let axis = parse_capture(fold.get(2)).unwrap();
            folds.push(match direction {
                "x" => Fold::X(axis),
                "y" => Fold::Y(axis),
                _ => panic!("I don't even"),
            });
        }
    }

    (dots, folds)
}

fn parse_capture(mat: Option<Match>) -> Option<u32> {
    mat.map(|m| m.as_str()).and_then(|s| s.parse::<u32>().ok())
}

enum Fold {
    X(u32),
    Y(u32),
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0

    fold along y=7
    fold along x=5
    ";

    #[test]
    fn base_check() {
        assert_eq!("17", process_data(TEST_CASE.to_string()));
    }
}
