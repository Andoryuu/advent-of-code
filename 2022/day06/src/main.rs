use std::{fs, collections::VecDeque};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let packet_marker = get_first_unduplication(input, 4);

    (packet_marker + 1).to_string()
}

fn process_data_adv(input: String) -> String {
    let message_marker = get_first_unduplication(input, 14);

    (message_marker + 1).to_string()
}

fn get_first_unduplication(input: String, window: usize) -> usize {
    let mut sliding_window = VecDeque::<char>::new();
    input
        .chars()
        .enumerate()
        .find(|(_, ch)| {
            sliding_window.push_back(*ch);
            if sliding_window.len() > window {
                sliding_window.pop_front();
            }
            sliding_window.iter().unique().count() == window
        })
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "7")]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", "5")]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", "6")]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10")]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_string()));
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "19")]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", "23")]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", "23")]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "29")]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "26")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_string()));
    }
}
