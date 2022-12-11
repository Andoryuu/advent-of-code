use std::{collections::VecDeque, fs};

use itertools::Itertools;
use num::Integer;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let mut monkeys = parse(input);
    let monkey_count = monkeys.len();

    for _ in 0..20 {
        for mix in 0..monkey_count {
            let new_items = {
                let monkey = monkeys.get_mut(mix).unwrap();
                let mut new_items = Vec::<(usize, u64)>::new();

                while let Some(item) = monkey.items.pop_front() {
                    let new_item = match monkey.operation {
                        Operation::Add(val) => item + val,
                        Operation::Mult(val) => item * val,
                        Operation::Square => item * item,
                    } / 3;

                    if new_item.is_multiple_of(&monkey.div_test) {
                        new_items.push((monkey.if_true, new_item));
                    } else {
                        new_items.push((monkey.if_false, new_item));
                    }

                    monkey.inspects += 1;
                }

                new_items
            };

            for (ix, item) in new_items {
                if let Some(m) = monkeys.get_mut(ix) {
                    m.items.push_back(item)
                }
            }
        }
    }

    monkeys
        .iter()
        .sorted_by_key(|m| m.inspects)
        .rev()
        .next_tuple()
        .map(|(a, b)| a.inspects * b.inspects)
        .unwrap()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    let mut monkeys = parse(input);
    let monkey_count = monkeys.len();
    let common_div = monkeys.iter().map(|m| m.div_test).product::<u64>();

    for _ in 0..10_000 {
        for mix in 0..monkey_count {
            let new_items = {
                let monkey = monkeys.get_mut(mix).unwrap();
                let mut new_items = Vec::<(usize, u64)>::new();

                while let Some(item) = monkey.items.pop_front() {
                    let new_item = match monkey.operation {
                        Operation::Add(val) => item + val,
                        Operation::Mult(val) => item * val,
                        Operation::Square => item * item,
                    } % common_div;

                    if new_item.is_multiple_of(&monkey.div_test) {
                        new_items.push((monkey.if_true, new_item));
                    } else {
                        new_items.push((monkey.if_false, new_item));
                    }

                    monkey.inspects += 1;
                }

                new_items
            };

            for (ix, item) in new_items {
                if let Some(m) = monkeys.get_mut(ix) {
                    m.items.push_back(item)
                }
            }
        }
    }

    monkeys
        .iter()
        .sorted_by_key(|m| m.inspects)
        .rev()
        .next_tuple()
        .map(|(a, b)| a.inspects * b.inspects)
        .unwrap()
        .to_string()
}

fn parse(input: String) -> Vec<Monkey> {
    input
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)| parse_monkey(group.collect_vec()))
        .collect_vec()
}

fn parse_monkey(lines: Vec<&str>) -> Monkey {
    let mut lines = lines.iter().skip(1);
    Monkey {
        items: lines
            .next()
            .and_then(|line| line.split(": ").last())
            .map(|items| {
                items
                    .split(", ")
                    .map(|item| item.parse::<u64>().unwrap())
                    .collect_vec()
            })
            .map(VecDeque::from)
            .unwrap(),
        operation: lines
            .next()
            .and_then(|line| line.split("= old ").last())
            .map(|op| op.split_whitespace().collect_vec())
            .map(|parts| match parts.as_slice() {
                ["*", "old"] => Operation::Square,
                ["*", val] => Operation::Mult(val.parse::<u64>().unwrap()),
                ["+", val] => Operation::Add(val.parse::<u64>().unwrap()),
                _ => panic!("unexpected operation: {:?}", parts),
            })
            .unwrap(),
        div_test: lines
            .next()
            .and_then(|line| line.split("by ").last())
            .and_then(|val| val.parse::<u64>().ok())
            .unwrap(),
        if_true: lines
            .next()
            .and_then(|line| line.split("monkey ").last())
            .and_then(|val| val.parse::<usize>().ok())
            .unwrap(),
        if_false: lines
            .next()
            .and_then(|line| line.split("monkey ").last())
            .and_then(|val| val.parse::<usize>().ok())
            .unwrap(),
        inspects: 0,
    }
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    div_test: u64,
    if_true: usize,
    if_false: usize,
    inspects: u64,
}

enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[rstest]
    #[case(TEST_CASE, "10605")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned()));
    }

    #[rstest]
    #[case(TEST_CASE, "2713310158")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned()));
    }
}
