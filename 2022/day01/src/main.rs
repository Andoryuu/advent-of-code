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
        .group_by(|&line| line.is_empty())
        .into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)| {
            group
                .filter_map(|line| line.parse::<u32>().ok())
                .sum::<u32>()
        })
        .sorted()
        .rev()
        .next()
        .unwrap()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    input
        .lines()
        .group_by(|&line| line.is_empty())
        .into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)| {
            group
                .filter_map(|line| line.parse::<u32>().ok())
                .sum::<u32>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[rstest]
    fn base_check() {
        assert_eq!("24000", process_data(TEST_CASE.to_string()));
    }

    #[rstest]
    fn adv_check() {
        assert_eq!("45000", process_data_adv(TEST_CASE.to_string()));
    }
}
