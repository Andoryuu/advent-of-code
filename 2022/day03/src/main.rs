use std::{char, fs, str::Chars};

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
        .map(|line| line.chars())
        .filter_map(get_outlier)
        .map(get_priority)
        .sum::<u32>()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    input
        .lines()
        .map(|line| line.chars())
        .tuples::<(_, _, _)>()
        .filter_map(get_common)
        .map(get_priority)
        .sum::<u32>()
        .to_string()
}

fn get_common((first, second, third): (Chars, Chars, Chars)) -> Option<char> {
    let snd_rucks = second.collect_vec();
    let trd_rucks = third.collect_vec();

    first
        .filter(|ch| snd_rucks.contains(ch))
        .find(|ch| trd_rucks.contains(ch))
}

fn get_outlier(chars: Chars) -> Option<char> {
    let chars_vec = chars.collect_vec();
    let split = chars_vec.len() / 2;
    let (first_comp, second_comp) = chars_vec.split_at(split);

    first_comp
        .iter()
        .find(|ch| second_comp.contains(ch))
        .copied()
}

fn get_priority(ch: char) -> u32 {
    const BASE_LOWER: u32 = ('a' as u32) - 1;
    const BASE_UPPER: u32 = ('A' as u32) - 27;

    (ch as u32)
        - if ch.is_lowercase() {
            BASE_LOWER
        } else {
            BASE_UPPER
        }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[rstest]
    fn base_check() {
        assert_eq!("157", process_data(TEST_CASE.to_string()));
    }

    #[rstest]
    fn adv_check() {
        assert_eq!("70", process_data_adv(TEST_CASE.to_string()));
    }
}
