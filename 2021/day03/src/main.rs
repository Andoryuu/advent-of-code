use std::fs;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let res: Vec<bool> = input
        .lines()
        .map(into_bool_vec)
        .fold(Vec::new(), |sum: Vec<i32>, line| {
            if sum.len() < line.len() {
                return line.iter().map(|b| if *b { 1 } else { -1 }).collect();
            }

            sum.iter()
                .zip(line)
                .map(|(s, b)| if b { *s + 1 } else { *s - 1 })
                .collect()
        })
        .iter()
        .map(|v| *v >= 0)
        .collect();

    let order = res.len();
    let gamma = bool_to_dec(&res);
    let epsilon = 2u32.pow(order.try_into().unwrap()) - 1 - gamma;

    (gamma * epsilon).to_string()
}

fn process_data_adv(input: String) -> String {
    let values: Vec<Vec<bool>> = input.lines().map(into_bool_vec).collect();

    let oxy = bool_to_dec(&oxy_filter(values.clone(), 0));
    let co2 = bool_to_dec(&co2_filter(values, 0));

    (oxy * co2).to_string()
}

fn oxy_filter(lines: Vec<Vec<bool>>, index: usize) -> Vec<bool> {
    if lines.len() <= 1 {
        return lines.get(0).unwrap().clone();
    }

    oxy_filter(most_common_for_index(&lines, index), index + 1)
}

fn co2_filter(lines: Vec<Vec<bool>>, index: usize) -> Vec<bool> {
    if lines.len() <= 1 {
        return lines.get(0).unwrap().clone();
    }

    co2_filter(least_common_for_index(&lines, index), index + 1)
}

fn total_for_index(lines: &[Vec<bool>], index: usize) -> i32 {
    lines.iter().fold(0, |sum, line| {
        if *line.get(index).unwrap() {
            sum + 1
        } else {
            sum - 1
        }
    })
}

fn filter_by_index_value(lines: &[Vec<bool>], index: usize, value: bool) -> Vec<Vec<bool>> {
    lines
        .iter()
        .filter(|line| *line.get(index).unwrap() == value)
        .cloned()
        .collect()
}

fn most_common_for_index(lines: &[Vec<bool>], index: usize) -> Vec<Vec<bool>> {
    filter_by_index_value(lines, index, total_for_index(lines, index) >= 0)
}

fn least_common_for_index(lines: &[Vec<bool>], index: usize) -> Vec<Vec<bool>> {
    filter_by_index_value(lines, index, total_for_index(lines, index) < 0)
}

fn into_bool_vec(input: &str) -> Vec<bool> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '1' => true,
            '0' => false,
            _ => false,
        })
        .collect()
}

fn bool_to_dec(input: &[bool]) -> u32 {
    input
        .iter()
        .rev()
        .fold((1u32, 0u32), |(order, sum), bit| {
            (order * 2, if *bit { sum + order } else { sum })
        })
        .1
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn total_for_index_check() {
        let test_case = "00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010";

        assert_eq!(
            2,
            total_for_index(
                &test_case
                    .lines()
                    .map(into_bool_vec)
                    .collect::<Vec<Vec<bool>>>(),
                0
            )
        )
    }

    #[test]
    fn into_bool_vec_check() {
        let test_case = "   10110";
        let result: Vec<bool> = [true, false, true, true, false].into();

        let pairs = result.iter().zip(into_bool_vec(test_case));

        for (expected, actual) in pairs {
            assert_eq!(*expected, actual);
        }
    }

    #[test]
    fn bool_to_dec_check() {
        let test_case = [true, false, true, true, false];

        assert_eq!(22, bool_to_dec(&test_case));
    }

    #[test]
    fn base_check() {
        let test_case = "00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010";

        assert_eq!("198", process_data(test_case.to_string()));
    }

    #[test]
    fn adv_check() {
        let test_case = "00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010";

        assert_eq!("230", process_data_adv(test_case.to_string()));
    }
}
