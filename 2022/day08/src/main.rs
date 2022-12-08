use std::{collections::BTreeSet, fs};

use itertools::Itertools;
use take_until::TakeUntilExt;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let line_lenght = input.lines().next().unwrap().len();
    let heights = input
        .trim()
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .collect_vec();

    let last_index = heights.len() - 1;

    let mut visible = BTreeSet::<usize>::new();

    for (row, chunk) in heights.iter().chunks(line_lenght).into_iter().enumerate() {
        let mut max: Option<u32> = None;
        for (col, &height) in chunk.enumerate() {
            if max.is_none() || height > max.unwrap() {
                max = Some(height);
                visible.insert(row * line_lenght + col);
            }
        }
    }

    for (row, chunk) in heights
        .iter()
        .rev()
        .chunks(line_lenght)
        .into_iter()
        .enumerate()
    {
        let mut max: Option<u32> = None;
        for (col, &height) in chunk.enumerate() {
            if max.is_none() || height > max.unwrap() {
                max = Some(height);
                visible.insert(last_index - (row * line_lenght + col));
            }
        }
    }

    for col in 0..line_lenght {
        let mut max: Option<u32> = None;
        for (row, &height) in heights.iter().skip(col).step_by(line_lenght).enumerate() {
            if max.is_none() || height > max.unwrap() {
                max = Some(height);
                visible.insert(row * line_lenght + col);
            }
        }
    }

    for col in (0..line_lenght).rev() {
        let mut max: Option<u32> = None;
        for (row, &height) in heights
            .iter()
            .rev()
            .skip(col)
            .step_by(line_lenght)
            .enumerate()
        {
            if max.is_none() || height > max.unwrap() {
                max = Some(height);
                visible.insert(last_index - (row * line_lenght + col));
            }
        }
    }

    visible.len().to_string()
}

fn process_data_adv(input: String) -> String {
    let line_lenght = input.lines().next().unwrap().len();
    let heights = input
        .trim()
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .collect_vec();

    let mut max_scenic = 0;

    for (ix, height) in heights.iter().enumerate() {
        let col = ix % line_lenght;
        let row = ix / line_lenght;

        let right = ((ix + 1)..=((row + 1) * line_lenght - 1))
            .take_until(|&iix| heights.get(iix).unwrap() >= height)
            .count();

        let left = if ix > 0 {
            ((row * line_lenght)..=(ix - 1))
                .rev()
                .take_until(|&iix| heights.get(iix).unwrap() >= height)
                .count()
        } else {
            0
        };

        let up = if row > 0 {
            (0..=(row - 1))
                .rev()
                .take_until(|&rix| heights.get(rix * line_lenght + col).unwrap() >= height)
                .count()
        } else {
            0
        };

        let down = ((row + 1)..line_lenght)
            .take_until(|&rix| heights.get(rix * line_lenght + col).unwrap() >= height)
            .count();

        let new_scenic = right * left * up * down;

        if new_scenic > max_scenic {
            max_scenic = new_scenic;
        }
    }

    max_scenic.to_string()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
30373
25512
65332
33549
35390
";

    #[rstest]
    fn base_check() {
        assert_eq!("21", process_data(TEST_CASE.to_string()));
    }

    #[rstest]
    fn adv_check() {
        assert_eq!("8", process_data_adv(TEST_CASE.to_string()));
    }
}
