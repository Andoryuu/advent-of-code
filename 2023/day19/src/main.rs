#![feature(test)]

mod helpers;

use std::{collections::HashMap, fs};

use helpers::{part::Part, range_part::RangePart, workflow::Workflow};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day19/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let (wfs, parts) = parse(input);
    let wfs = HashMap::<_, _>::from_iter(wfs.into_iter().map(|wf| (wf.label, wf.ops)));

    parts
        .into_iter()
        .filter(|part| {
            let mut wf = wfs.get("in").unwrap();
            loop {
                for op in wf {
                    if let Some(res) = part.apply_op(op) {
                        match res.as_str() {
                            "A" => return true,
                            "R" => return false,
                            x => {
                                wf = wfs.get(x).unwrap();
                                break;
                            }
                        }
                    }
                }
            }
        })
        .map(Part::get_rating)
        .sum::<u32>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let (wfs, _) = parse(input);
    let wfs = HashMap::<_, _>::from_iter(wfs.into_iter().map(|wf| (wf.label, wf.ops)));

    let mut ranges = vec![(
        RangePart {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
        "in".to_owned(),
    )];

    let mut sum = 0usize;

    while let Some((range, label)) = ranges.pop() {
        let wf = wfs.get(&label).unwrap();
        let mut range = Some(range);

        for op in wf {
            if let Some(r) = range {
                let (matched, unmatched) = r.apply_op(op);
                range = unmatched;

                if let Some(matched) = matched {
                    match matched.1.as_str() {
                        "A" => sum += matched.0.get_combinations(),
                        "R" => {}
                        _ => ranges.push(matched),
                    }
                }
            } else {
                break;
            }
        }
    }

    sum.to_string()
}

fn parse(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let (wfs, _, parts) = input
        .trim()
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .map(|(_, g)| g.into_iter().collect_vec())
        .collect_tuple()
        .unwrap();

    (
        wfs.into_iter().map_into().collect_vec(),
        parts.into_iter().map_into().collect_vec(),
    )
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[rstest]
    #[case(TEST_CASE, "19114")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "167409079868000")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("368964")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("127675188176682")]
    fn part_2_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_2(&input));
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
        b.iter(|| process_part_2(TEST_CASE));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_2(&input));
    }
}
