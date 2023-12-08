#![feature(test)]

use std::{collections::HashMap, fs};

use itertools::Itertools;
use num::Integer;

fn main() {
    let input = fs::read_to_string("./day08/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let (dirs, map) = parse(input);
    let mut pos = "AAA";

    for (i, d) in dirs.iter().cycle().enumerate() {
        let (l, r) = map.get(pos).unwrap();

        pos = if *d { *l } else { *r };

        if pos == "ZZZ" {
            return (i + 1).to_string();
        }
    }

    panic!("no directions found")
}

fn process_part_2(input: &str) -> String {
    let (dirs, map) = parse(input);

    map.iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(k, _)| *k)
        .map(|p: &str| {
            let mut pos = p;
            let mut ofs = None;

            for (step, dir) in dirs.iter().cycle().enumerate() {
                let (l, r) = map.get(pos).unwrap();

                pos = if *dir { *l } else { *r };

                if pos.ends_with('Z') {
                    if let Some(ofs_step) = ofs {
                        return (ofs_step + 1, step - ofs_step);
                    } else {
                        ofs = Some(step);
                    }
                }
            }

            panic!("no directions found")
        })
        .sorted()
        .map(|(x, y)| (x as isize, y as isize))
        .reduce(combine_phases)
        .map(|(ofs, per)| ofs + per)
        .unwrap()
        .to_string()
}

fn parse(input: &str) -> (Vec<bool>, HashMap<&str, (&str, &str)>) {
    (
        input
            .lines()
            .find(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| c == 'L').collect_vec())
            .unwrap(),
        input
            .lines()
            .filter_map(|line| line.split(['=', ',']).next_tuple())
            .map(|(k, l, r)| {
                (
                    k.trim(),
                    (
                        l.trim_matches(|c| [' ', '(', ')'].contains(&c)),
                        r.trim_matches(|c| [' ', '(', ')'].contains(&c)),
                    ),
                )
            })
            .collect(),
    )
}

fn combine_phases((ofs1, per1): (isize, isize), (ofs2, per2): (isize, isize)) -> (isize, isize) {
    let ofs1 = ofs1 % per1;
    let ofs2 = ofs2 % per2;

    let (ofs1, per1, ofs2, per2) = if ofs1 > ofs2 {
        (ofs2, per2, ofs1, per1)
    } else {
        (ofs1, per1, ofs2, per2)
    };

    let egcd = per1.extended_gcd(&per2);
    let (mult, rem) = (ofs1 - ofs2).rem_euclid(per2).div_rem(&egcd.gcd);
    if rem > 0 {
        panic!("won't")
    }

    let comb_per = per1.lcm(&per2);

    let comb_ofs = (egcd.x * mult * per1).rem_euclid(comb_per);
    let comb_ofs = (-comb_ofs).rem_euclid(comb_per);

    (comb_ofs, comb_per)
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE_1: &str = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const TEST_CASE_2: &str = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[rstest]
    #[case(TEST_CASE_1, "6")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE_2, "6")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("21797")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("23977527174353")]
    fn part_2_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_2(&input));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE_1));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_1(&input));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE_1));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_2(&input));
    }
}
