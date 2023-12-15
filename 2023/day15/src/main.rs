#![feature(test)]

use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day15/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    input
        .trim()
        .split(',')
        .map(get_hash)
        .sum::<usize>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let mut boxes = vec![Vec::<(String, usize)>::new(); 256];

    for step in input.trim().split(',') {
        match step.into() {
            Step::Remove(label) => {
                let box_i = get_hash(&label);
                let boxx = boxes.get_mut(box_i).unwrap();
                if let Some((lens_i, _)) = boxx.iter().find_position(|(lens, _)| *lens == label) {
                    boxx.remove(lens_i);
                }
            }
            Step::Set((label, focal)) => {
                let box_i = get_hash(&label);
                let boxx = boxes.get_mut(box_i).unwrap();
                if let Some((lens_i, _)) = boxx.iter().find_position(|(lens, _)| *lens == label) {
                    boxx[lens_i] = (label, focal);
                } else {
                    boxx.push((label, focal));
                }
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(box_i, boxx)| {
            boxx.into_iter()
                .enumerate()
                .map(move |(lens_i, lens)| (box_i + 1) * (lens_i + 1) * lens.1)
        })
        .sum::<usize>()
        .to_string()
}

fn get_hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}

enum Step {
    Remove(String),
    Set((String, usize)),
}

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        let groups = value
            .chars()
            .group_by(|c| *c != '-' && *c != '=')
            .into_iter()
            .map(|(_, g)| g.collect::<String>())
            .collect_vec();

        match groups.as_slice() {
            [label, _] => Step::Remove(label.to_owned()),
            [label, _, focal] => Step::Set((label.to_owned(), focal.parse::<usize>().unwrap())),
            x => panic!("wrong parse: {:?}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[rstest]
    fn hash_check() {
        assert_eq!(52, get_hash("HASH"));
    }

    #[rstest]
    #[case(TEST_CASE, "1320")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "145")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("511257")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("239484")]
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
