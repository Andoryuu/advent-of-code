use std::{collections::BTreeSet, fs};

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    input
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .split('|')
                .skip(1)
                .map(|p| {
                    p.split(' ')
                        .map(|ip| ip.len())
                        .filter(|&len| len == 2 || len == 3 || len == 4 || len == 7)
                        .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    input
        .trim()
        .lines()
        .map(parse_line)
        .map(|(i, t)| solve_line(i, t))
        .sum::<u32>()
        .to_string()
}

fn parse_line(line: &str) -> (Vec<Segment>, Vec<Segment>) {
    let segments: Vec<Segment> = line
        .trim()
        .split('|')
        .flat_map(|s| s.trim().split(' '))
        .map(|s| {
            let mut wires = BTreeSet::new();
            for c in s.chars() {
                wires.insert(c);
            }

            Segment {
                size: s.len(),
                wires,
            }
        })
        .collect();

    let inputs = segments.get(0..10).unwrap();
    let outputs = segments.get(10..).unwrap();

    (inputs.to_vec(), outputs.to_vec())
}

fn solve_line(inputs: Vec<Segment>, targets: Vec<Segment>) -> u32 {
    let &one = inputs
        .iter()
        .filter(|&s| s.size == 2)
        .collect::<Vec<&Segment>>()
        .get(0)
        .unwrap();

    let &four = inputs
        .iter()
        .filter(|&s| s.size == 4)
        .collect::<Vec<&Segment>>()
        .get(0)
        .unwrap();

    let mut number = 0u32;

    for (index, target) in targets.iter().rev().enumerate() {
        let sans_one = target.wires.difference(&one.wires).count();
        let sans_four = target.wires.difference(&four.wires).count();

        let num = match (target.size, sans_one, sans_four) {
            (2, _, _) => 1,
            (3, _, _) => 7,
            (4, _, _) => 4,
            (7, _, _) => 8,
            (5, 4, 3) => 2,
            (5, 3, 2) => 3,
            (5, 4, 2) => 5,
            (6, 4, 3) => 0,
            (6, 5, 3) => 6,
            (6, 4, 2) => 9,
            (x, y, z) => panic!("{}, {}, {}", x, y, z),
        };

        number += num * 10u32.pow(index.try_into().unwrap());
    }

    number
}

#[derive(Debug, Clone)]
struct Segment {
    size: usize,
    wires: BTreeSet<char>,
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    ";

    #[test]
    fn solve_line_check() {
        let line =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

        let (inputs, outputs) = parse_line(line);

        assert_eq!(5353, solve_line(inputs, outputs));
    }

    #[test]
    fn base_check() {
        assert_eq!("26", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("61229", process_data_adv(TEST_CASE.to_string()));
    }
}
