#![feature(test)]

use std::fs;

use cached::proc_macro::cached;
use itertools::Itertools;
use rustc_hash::FxHashMap;

fn main() {
    let input = fs::read_to_string("./crates/day11/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let map = parse(input);
    traverse_you_out("you", &map).to_string()
}

fn process_part_2(input: &str) -> String {
    let map = parse(input);
    traverse_svr_out("svr", false, false, &map).to_string()
}

fn parse(input: &str) -> FxHashMap<&str, Vec<&str>> {
    input
        .lines()
        .filter_map(|line| line.split_once(':'))
        .map(|(input, outputs)| (input, outputs.split_ascii_whitespace().collect_vec()))
        .collect()
}

fn traverse_you_out(from: &str, map: &FxHashMap<&str, Vec<&str>>) -> usize {
    if from == "out" {
        return 1;
    }

    if let Some(outs) = map.get(from) {
        outs.iter().map(|o| traverse_you_out(o, map)).sum()
    } else {
        0
    }
}

#[cached(
    key = "(String, bool, bool, String)",
    convert = r#"{(from.to_owned(), has_dac, has_fft, format!("{map:p}"))}"#
)]
fn traverse_svr_out(
    from: &str,
    mut has_dac: bool,
    mut has_fft: bool,
    map: &FxHashMap<&str, Vec<&str>>,
) -> usize {
    if from == "out" {
        return if has_dac && has_fft { 1 } else { 0 };
    }

    has_dac |= from == "dac";
    has_fft |= from == "fft";

    if let Some(outs) = map.get(from) {
        outs.iter()
            .map(|o| traverse_svr_out(o, has_dac, has_fft, map))
            .sum()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const TEST_CASE_2: &str = "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "5")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE_2, "2")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("764")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("462444153119850")]
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
        b.iter(|| process_part_2(TEST_CASE_2));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_2(&input));
    }
}
