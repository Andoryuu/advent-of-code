use std::fs;

use indicatif::ProgressIterator;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let nums = parse(input);
    let mut field = nums.iter().map(|(id, _)| *id).collect_vec();

    let size = field.len();
    let rem_size = size - 1;
    let i_rem_size = rem_size as isize;
    let mut zero_id = None;

    for (id, val) in nums.iter() {
        let val = *val;
        let id = *id;

        if val == 0 {
            zero_id = Some(id);
            continue;
        }

        let (ix, _) = field.iter().find_position(|&&v| v == id).unwrap();
        field.remove(ix);

        let new_ix = ((ix + rem_size) as isize + (val % i_rem_size)) as usize % rem_size;
        field.insert(new_ix, id);
    }

    let zero_id = zero_id.unwrap();
    let (zero_ix, _) = field.iter().find_position(|&&v| v == zero_id).unwrap();
    field
        .iter()
        .cycle()
        .skip(zero_ix)
        .step_by(1_000)
        .take(4) // first element is the zero
        .map(|id| nums.iter().find(|(v_id, _)| v_id == id).unwrap().1)
        .sum::<isize>()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    const KEY: isize = 811_589_153;

    let base_nums = parse(input);
    let nums = base_nums.iter().map(|(id, n)| (*id, n * KEY)).collect_vec();
    let mut field = nums.iter().map(|(id, _)| *id).collect_vec();

    let size = field.len();
    let rem_size = size - 1;
    let i_rem_size = rem_size as isize;
    let mut zero_id = None;

    for _ in (0..10).progress() {
        for (id, val) in nums.iter() {
            let val = *val;
            let id = *id;

            if val == 0 {
                zero_id = Some(id);
                continue;
            }

            let (ix, _) = field.iter().find_position(|&&v| v == id).unwrap();
            field.remove(ix);

            let new_ix = ((ix + rem_size) as isize + (val % i_rem_size)) as usize % rem_size;
            field.insert(new_ix, id);
        }
    }

    let zero_id = zero_id.unwrap();
    let (zero_ix, _) = field.iter().find_position(|&&v| v == zero_id).unwrap();
    field
        .iter()
        .cycle()
        .skip(zero_ix)
        .step_by(1_000)
        .take(4) // first element is the zero
        .map(|id| nums.iter().find(|(v_id, _)| v_id == id).unwrap().1)
        .sum::<isize>()
        .to_string()
}

fn parse(input: String) -> Vec<(usize, isize)> {
    input
        .lines()
        .enumerate()
        .map(|(ix, line)| (ix, line.parse::<isize>().unwrap()))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
1
2
-3
3
-2
0
4
";

    #[rstest]
    #[case(TEST_CASE, "3")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned()));
    }

    #[rstest]
    #[case(TEST_CASE, "1623178306")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned()));
    }
}
