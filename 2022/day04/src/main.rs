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
        .filter_map(|line| line.split(',').map(Range::from).next_tuple())
        .filter(|(left, right)| left.is_full_overlap(right))
        .count()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    input
        .lines()
        .filter_map(|line| line.split(',').map(Range::from).next_tuple())
        .filter(|(left, right)| left.is_partial_overlap(right))
        .count()
        .to_string()
}

struct Range {
    from: u32,
    to: u32,
}

impl Range {
    fn is_full_overlap(self: &Range, other: &Range) -> bool {
        (self.from <= other.from && self.to >= other.to)
            || (self.from >= other.from && self.to <= other.to)
    }

    fn is_partial_overlap(self: &Range, other: &Range) -> bool {
        self.from <= other.to && self.to >= other.from
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        value
            .split('-')
            .filter_map(|val| val.parse::<u32>().ok())
            .next_tuple()
            .map(|(from, to)| Range { from, to })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[rstest]
    fn base_check() {
        assert_eq!("2", process_data(TEST_CASE.to_string()));
    }

    #[rstest]
    fn adv_check() {
        assert_eq!("4", process_data_adv(TEST_CASE.to_string()));
    }
}
