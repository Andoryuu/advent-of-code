#![feature(test)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use itertools::Itertools;

const BROADCASTER: &str = "broadcaster";
const BUTTON: &str = "button";
const TARGET: &str = "rx";

fn main() {
    let input = fs::read_to_string("./day20/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input, TARGET));
}

fn process_part_1(input: &str) -> String {
    let mut modules = parse(input);

    let mut low_pulses = 0isize;
    let mut high_pulses = 0isize;

    for _ in 0..1_000 {
        let mut signals = VecDeque::new();
        signals.push_back((Signal::Low, BUTTON.to_owned(), BROADCASTER.to_owned()));

        while let Some((signal, from, target)) = signals.pop_front() {
            match signal {
                Signal::Low => low_pulses += 1,
                Signal::High => high_pulses += 1,
            }

            modules.entry(target.to_owned()).and_modify(|m| {
                for (s, next) in m.send_signal(signal, from) {
                    signals.push_back((s, target.to_owned(), next));
                }
            });
        }
    }

    (low_pulses * high_pulses).to_string()
}

fn process_part_2(input: &str, fin: &str) -> String {
    let mut modules = parse(input);

    // handpicked from callers of 'rx'
    // took me six hours of debugging to realize I missed one of them...
    let mut sus = HashSet::<_>::from_iter(vec![
        "jg".to_owned(),
        "rh".to_owned(),
        "jm".to_owned(),
        "hf".to_owned(),
    ]);

    let mut done = vec![];

    'outer: for i in 0.. {
        let mut signals = VecDeque::new();
        signals.push_back((Signal::Low, BUTTON.to_owned(), BROADCASTER.to_owned()));

        while let Some((signal, from, target)) = signals.pop_front() {
            if target == fin && signal == Signal::Low {
                return (i + 1).to_string();
            }

            if let Some(m) = modules.get_mut(&target) {
                let mut is_high = false;

                for (s, next) in m.send_signal(signal, from) {
                    if s == Signal::High {
                        is_high = true;
                    }

                    signals.push_back((s, target.to_owned(), next));
                }

                if is_high && sus.contains(&target) {
                    done.push(i + 1);
                    sus.remove(&target);

                    if sus.is_empty() {
                        break 'outer;
                    }
                }
            }
        }
    }

    done.into_iter().product::<isize>().to_string()
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut conjs = HashSet::new();
    let mut callers = HashMap::new();

    let mut modules = HashMap::<_, _>::from_iter(input.trim().lines().map(|line| {
        let (module, outputs) = line.split(" -> ").collect_tuple().unwrap();
        let outputs = outputs.split(", ").map(|o| o.to_owned()).collect_vec();

        let (mod_type, mod_name) = match module.split_at(1) {
            ("b", "roadcaster") => (
                Module::Broadcast {
                    outputs: outputs.clone(),
                },
                BROADCASTER.to_owned(),
            ),
            ("%", x) => (
                Module::FlipFlow {
                    state: false,
                    outputs: outputs.clone(),
                },
                x.to_owned(),
            ),
            ("&", x) => {
                conjs.insert(x.to_owned());

                (
                    Module::Conjunction {
                        inputs: HashMap::new(),
                        outputs: outputs.clone(),
                    },
                    x.to_owned(),
                )
            }
            (_, x) => (Module::Generic, x.to_owned()),
        };

        for out in outputs.iter() {
            callers
                .entry(out.to_owned())
                .and_modify(|v: &mut Vec<String>| v.push(mod_name.to_owned()))
                .or_insert_with(|| vec![mod_name.to_owned()]);
        }

        (mod_name, mod_type)
    }));

    for conj in conjs {
        modules.entry(conj.to_owned()).and_modify(|m| {
            if let Module::Conjunction { inputs, .. } = m {
                if let Some(callers) = callers.get(&conj) {
                    for caller in callers {
                        inputs.insert(caller.to_owned(), Signal::Low);
                    }
                }
            }
        });
    }

    modules
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Signal {
    Low,
    High,
}

#[derive(Debug)]
enum Module {
    Broadcast {
        outputs: Vec<String>,
    },
    Conjunction {
        inputs: HashMap<String, Signal>,
        outputs: Vec<String>,
    },
    FlipFlow {
        state: bool,
        outputs: Vec<String>,
    },
    Generic,
}

impl Module {
    fn send_signal(&mut self, signal: Signal, input: String) -> Vec<(Signal, String)> {
        match self {
            Module::Broadcast { outputs } => {
                outputs.iter().map(|o| (signal, o.to_owned())).collect_vec()
            }
            Module::Conjunction { inputs, outputs } => {
                inputs.entry(input).and_modify(|i| *i = signal);

                let signal = if inputs.values().all(|v| *v == Signal::High) {
                    Signal::Low
                } else {
                    Signal::High
                };

                outputs.iter().map(|o| (signal, o.to_owned())).collect_vec()
            }
            Module::FlipFlow {
                ref mut state,
                outputs,
            } => {
                if signal == Signal::Low {
                    *state = !*state;
                    let signal = if *state { Signal::High } else { Signal::Low };

                    outputs.iter().map(|o| (signal, o.to_owned())).collect_vec()
                } else {
                    vec![]
                }
            }
            Module::Generic => panic!("unspecified"),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const TEST_CASE_2: &str = "
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[rstest]
    #[case(TEST_CASE, "32000000")]
    #[case(TEST_CASE_2, "11687500")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE_2, "output", "1")]
    fn part_2_check(#[case] input: &str, #[case] fin: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input, fin));
    }

    #[rstest]
    #[case("821985143")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("240853834793347")]
    fn part_2_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_2(&input, TARGET));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_1(&input));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE_2, "output"));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_2(&input, TARGET));
    }
}
