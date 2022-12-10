use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: \n{}", adv_output);
}

fn process_data(input: String) -> String {
    let mut log = Vec::<i32>::new();
    let mut x_reg = 1i32;

    for op in parse(input) {
        match op {
            Op::Noop => log.push(x_reg),
            Op::AddX(v) => {
                log.push(x_reg);
                log.push(x_reg);
                x_reg += v;
            }
        }
    }

    log.iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(ix, val)| ((ix + 1) as i32, val))
        .map(|(ix, val)| val * ix)
        .sum::<i32>()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    let mut log = Vec::<i32>::new();
    let mut x_reg = 1i32;

    for op in parse(input) {
        match op {
            Op::Noop => log.push(x_reg),
            Op::AddX(v) => {
                log.push(x_reg);
                log.push(x_reg);
                x_reg += v;
            }
        }
    }

    log.iter()
        .enumerate()
        .map(|(ix, val)| ((ix % 40) as i32, val))
        .map(|(ix, val)| {
            if ((val - 1)..=(val + 1)).contains(&ix) {
                '#'
            } else {
                '.'
            }
        })
        .chunks(40)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n")
}

fn parse(input: String) -> Vec<Op> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
        .filter_map(|split| match split.as_slice() {
            ["noop"] => Some(Op::Noop),
            ["addx", v] => Some(Op::AddX(v.parse::<i32>().unwrap())),
            [] => None,
            _ => panic!("unknown op: {:?}", split),
        })
        .collect_vec()
}

enum Op {
    Noop,
    AddX(i32),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[rstest]
    #[case(TEST_CASE, "13140")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned()));
    }

    #[rstest]
    #[case(
        TEST_CASE,
        "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    )]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned()));
    }
}
