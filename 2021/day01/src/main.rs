#![feature(let_else)]

use std::fs;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let res = input.lines().fold((0, Option::<u32>::None), |t, line| {
        let new = line.trim().parse::<u32>().unwrap();

        let new_count = if let (c, Some(old)) = t {
            if old < new {
                c + 1
            } else {
                c
            }
        } else {
            0
        };

        (new_count, Some(new))
    });

    res.0.to_string()
}

fn process_data_adv(input: String) -> String {
    input
        .lines()
        .fold(
            (
                0,
                Option::<u32>::None,
                Option::<u32>::None,
                Option::<u32>::None,
            ),
            |t, line| {
                let new = line.trim().parse::<u32>().unwrap();

                let Some(old1) = t.3 else {
                    return (0, Option::<u32>::None, Option::<u32>::None, Some(new));
                };

                let Some(old2) = t.2 else {
                    return (0, Option::<u32>::None, Some(old1 + new), Some(new));
                };

                let Some(old3) = t.1 else {
                    return (0, Some(old2 + new), Some(old1 + new), Some(new));
                };

                let new_count = if old3 < (old2 + new) { t.0 + 1 } else { t.0 };

                (new_count, Some(old2 + new), Some(old1 + new), Some(new))
            },
        )
        .0
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::{process_data, process_data_adv};

    #[test]
    fn base_check() {
        let test_case = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";

        assert_eq!("7", process_data(test_case.to_string()));
    }

    #[test]
    fn adv_check() {
        let test_case = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";

        assert_eq!("5", process_data_adv(test_case.to_string()));
    }
}
