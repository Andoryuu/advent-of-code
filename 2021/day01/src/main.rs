use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    input
        .lines()
        .map(|line| line.trim().parse::<u32>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    input
        .lines()
        .map(|line| line.trim().parse::<u32>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
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
