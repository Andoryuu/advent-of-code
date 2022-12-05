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
    let (mut state, steps) = parse(input);

    for step in steps {
        apply_step(&mut state, step);
    }

    get_top_values(&state).iter().collect::<String>()
}

fn process_data_adv(input: String) -> String {
    let (mut state, steps) = parse(input);

    for step in steps {
        apply_step_preserve(&mut state, step);
    }

    get_top_values(&state).iter().collect::<String>()
}

fn get_top_values(state: &[Vec<char>]) -> Vec<char> {
    state
        .iter()
        .filter_map(|vec| vec.last())
        .copied()
        .collect_vec()
}

fn apply_step(state: &mut [Vec<char>], step: Step) {
    for _ in 0..step.count {
        let el = state.get_mut(step.from).and_then(|vec| vec.pop()).unwrap();
        if let Some(vec) = state.get_mut(step.to) {
            vec.push(el)
        }
    }
}

fn apply_step_preserve(state: &mut [Vec<char>], step: Step) {
    let mut buffer = Vec::<char>::new();

    for _ in 0..step.count {
        let el = state.get_mut(step.from).and_then(|vec| vec.pop()).unwrap();
        buffer.push(el);
    }

    buffer.reverse();

    if let Some(vec) = state.get_mut(step.to) {
        for ch in buffer {
            vec.push(ch);
        }
    }
}

fn parse(input: String) -> (Vec<Vec<char>>, Vec<Step>) {
    // we _could_ rotate the input by hand and write it as lines first, but...
    let mut state = Vec::<Vec<char>>::new();
    for line in input
        .lines()
        .take_while(|line| !line.is_empty())
        .collect_vec()
        .iter()
        .rev()
        .skip(1)
    {
        for (ix, chunk) in line.chars().chunks(4).into_iter().enumerate() {
            for &ch in chunk
                .into_iter()
                .nth(1)
                .iter()
                .filter(|ch| !ch.is_whitespace())
            {
                if let Some(vec) = state.get_mut(ix) {
                    vec.push(ch);
                } else {
                    state.push(vec![ch]);
                }
            }
        }
    }

    let steps = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .map(Step::from)
        .collect_vec();

    (state, steps)
}

#[derive(Debug)]
struct Step {
    count: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        let (count, from, to) = value
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .filter_map(|part| part.parse::<usize>().ok())
            .next_tuple()
            .unwrap();

        Step {
            count,
            from: from - 1,
            to: to - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[rstest]
    fn base_check() {
        assert_eq!("CMZ", process_data(TEST_CASE.to_string()));
    }

    #[rstest]
    fn adv_check() {
        assert_eq!("MCD", process_data_adv(TEST_CASE.to_string()));
    }
}
