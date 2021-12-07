use std::fs;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let positions = parse(input);

    let target = *positions.get(positions.len() / 2).unwrap();

    positions
        .iter()
        .map(|p| (target - p).abs())
        .sum::<i32>()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    let positions = parse(input);

    let average = positions.iter().sum::<i32>() as f64 / positions.len() as f64;
    let rough_target = average.round() as i32;

    [
        get_true_sum_for_target(&positions, rough_target - 1),
        get_true_sum_for_target(&positions, rough_target),
        get_true_sum_for_target(&positions, rough_target + 1),
    ]
    .iter()
    .min()
    .unwrap()
    .to_string()
}

fn parse(input: String) -> Vec<i32> {
    let mut positions: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    positions.sort_unstable();

    positions
}

fn get_true_sum_for_target(positions: &[i32], target: i32) -> i32 {
    positions
        .iter()
        .map(|p| (target - p).abs())
        .map(|d| (0..=d).sum::<i32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "16,1,2,0,4,2,7,1,2,14
    ";

    #[test]
    fn base_check() {
        assert_eq!("37", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("168", process_data_adv(TEST_CASE.to_string()));
    }
}
