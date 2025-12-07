#![feature(test)]

use std::fs;

use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    let input = fs::read_to_string("./crates/day07/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let (splitters, start) = parse(input);

    let mut beams = FxHashSet::default();
    beams.insert(start);

    let mut splits = 0usize;

    for line in splitters {
        let mut new_beams = FxHashSet::default();
        for beam in beams {
            if line.contains(&beam) {
                new_beams.insert(beam - 1);
                new_beams.insert(beam + 1);
                splits += 1;
            } else {
                new_beams.insert(beam);
            }
        }
        beams = new_beams;
    }

    splits.to_string()
}

fn process_part_2(input: &str) -> String {
    let (splitters, start) = parse(input);

    let mut beams = FxHashMap::default();
    beams.insert(start, 1usize);

    for line in splitters {
        let mut new_beams = FxHashMap::default();
        for (beam, count) in beams {
            if line.contains(&beam) {
                new_beams
                    .entry(beam - 1)
                    .and_modify(|b| *b += count)
                    .or_insert(count);
                new_beams
                    .entry(beam + 1)
                    .and_modify(|b| *b += count)
                    .or_insert(count);
            } else {
                new_beams
                    .entry(beam)
                    .and_modify(|b| *b += count)
                    .or_insert(count);
            }
        }
        beams = new_beams;
    }

    beams.into_values().sum::<usize>().to_string()
}

fn parse(input: &str) -> (Vec<FxHashSet<usize>>, usize) {
    let start = input
        .lines()
        .find(|line| !line.is_empty())
        .unwrap()
        .find('S')
        .unwrap();

    let splitters = input
        .lines()
        .filter(|line| !line.is_empty())
        .skip(2)
        .step_by(2)
        .map(|line| {
            line.char_indices()
                .filter_map(|(ix, c)| if c == '^' { Some(ix) } else { None })
                .collect()
        })
        .collect();

    (splitters, start)
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "21")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "40")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("1658")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("53916299384254")]
    fn part_2_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(&input));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_1(&input));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_2(&input));
    }
}
