use std::{
    collections::{BTreeMap, VecDeque},
    fs,
};

use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let monkeys = parse(input);
    let mut results = BTreeMap::<String, i64>::new();
    let mut monkey_que = VecDeque::from(monkeys);

    while let Some(monkey) = monkey_que.pop_front() {
        match &monkey.operation {
            Operation::Const(val) => {
                results.insert(monkey.name, *val);
            }
            Operation::Binary(left, right, op) => {
                if let Some((leftval, rightval)) = results.get(left).zip(results.get(right)) {
                    let val = match op {
                        BinaryOp::Add => leftval + rightval,
                        BinaryOp::Sub => leftval - rightval,
                        BinaryOp::Mult => leftval * rightval,
                        BinaryOp::Div => leftval / rightval,
                    };

                    if monkey.name == "root" {
                        return val.to_string();
                    } else {
                        results.insert(monkey.name, val);
                    }
                } else {
                    monkey_que.push_back(monkey);
                }
            }
        }
    }

    panic!("could not resolve root monkey")
}

fn process_data_adv(input: String) -> String {
    let monkeys = parse(input);
    let mut results = BTreeMap::<String, i64>::new();
    let mut monkey_que = VecDeque::from(monkeys);
    let mut root = None;
    let mut last_len = 0;

    while let Some(monkey) = monkey_que.pop_front() {
        if monkey.name == "root" {
            root = Some(monkey);
            continue;
        }

        if monkey.name == "humn" {
            if last_len == monkey_que.len() {
                break;
            } else {
                last_len = monkey_que.len();
                monkey_que.push_back(monkey);
                continue;
            }
        }

        match &monkey.operation {
            Operation::Const(val) => {
                results.insert(monkey.name, *val);
            }
            Operation::Binary(left, right, op) => {
                if let Some((leftval, rightval)) = results.get(left).zip(results.get(right)) {
                    results.insert(
                        monkey.name,
                        match op {
                            BinaryOp::Add => leftval + rightval,
                            BinaryOp::Sub => leftval - rightval,
                            BinaryOp::Mult => leftval * rightval,
                            BinaryOp::Div => leftval / rightval,
                        },
                    );
                } else {
                    monkey_que.push_back(monkey);
                }
            }
        }
    }

    let root = root.unwrap();
    let (left, right) = root.operation.get_pair().unwrap();

    // one of them surely has a value, right? ... right?
    let (start, target) = if results.get(&left).is_none() {
        (left, results.get(&right).unwrap())
    } else {
        (right, results.get(&left).unwrap())
    };

    let remaining_monkeys = BTreeMap::from_iter(monkey_que.iter().map(|m| (m.name.clone(), m)));

    // let's just assume there is always only single unresolved branch
    let mut target = *target;
    let mut from_name = start;

    while from_name != "humn" {
        let monkey = remaining_monkeys.get(&from_name).unwrap();
        match &monkey.operation {
            Operation::Const(_) => panic!("this shouldn't be here"),
            Operation::Binary(left, right, op) => {
                let leftval = results.get(left);
                let rightval = results.get(right);

                if let Some(leftval) = leftval {
                    from_name = right.clone();
                    target = match op {
                        BinaryOp::Add => target - leftval,
                        BinaryOp::Sub => leftval - target,
                        BinaryOp::Mult => target / leftval,
                        BinaryOp::Div => leftval / target,
                    };
                } else {
                    from_name = left.clone();
                    let rightval = rightval.unwrap();
                    target = match op {
                        BinaryOp::Add => target - rightval,
                        BinaryOp::Sub => target + rightval,
                        BinaryOp::Mult => target / rightval,
                        BinaryOp::Div => target * rightval,
                    };
                }
            }
        }
    }

    target.to_string()
}

fn parse(input: String) -> Vec<Monkey> {
    let rgx = Regex::new("(?P<name>[a-z]{4}): (?:(?P<const>\\d+)|((?P<leftref>[a-z]{4}) (?P<op>[-+*/]) (?P<rightref>[a-z]{4})))").unwrap();

    input
        .lines()
        .map(|line| {
            let caps = rgx.captures(line).unwrap();
            let name = caps["name"].to_owned();
            if let Some(val) = caps.name("const") {
                let val = val.as_str().parse::<i64>().unwrap();
                Monkey {
                    name,
                    operation: Operation::Const(val),
                }
            } else {
                let leftref = caps["leftref"].to_owned();
                let rightref = caps["rightref"].to_owned();
                let op = &caps["op"];

                Monkey {
                    name,
                    operation: Operation::new(op, leftref, rightref),
                }
            }
        })
        .collect_vec()
}

struct Monkey {
    name: String,
    operation: Operation,
}

enum Operation {
    Const(i64),
    Binary(String, String, BinaryOp),
}

enum BinaryOp {
    Add,
    Sub,
    Mult,
    Div,
}

impl Operation {
    fn new(op: &str, leftref: String, rightref: String) -> Self {
        match op {
            "+" => Operation::Binary(leftref, rightref, BinaryOp::Add),
            "-" => Operation::Binary(leftref, rightref, BinaryOp::Sub),
            "*" => Operation::Binary(leftref, rightref, BinaryOp::Mult),
            "/" => Operation::Binary(leftref, rightref, BinaryOp::Div),
            _ => panic!("unknow operation: {op}"),
        }
    }

    fn get_pair(&self) -> Option<(String, String)> {
        match self {
            Operation::Const(_) => None,
            Operation::Binary(left, right, _) => Some((left.clone(), right.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[rstest]
    #[case(TEST_CASE, "152")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned()));
    }

    #[rstest]
    #[case(TEST_CASE, "301")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned()));
    }
}
