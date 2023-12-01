#![feature(test)]

use std::fs;

use fancy_regex::Regex;

fn main() {
    let input = fs::read_to_string("./day01/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    input
        .lines()
        .filter_map(|l| {
            let nums = l
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            nums.first().zip(nums.last()).map(|(f, l)| f * 10 + l)
        })
        .sum::<u32>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let rgx =
        Regex::new(r"on(?=e)|tw(?=o)|thre(?=e)|four|fiv(?=e)|six|seve(?=n)|eigh(?=t)|nin(?=e)|\d")
            .unwrap();

    let lls = input
        .lines()
        .filter_map(|l| {
            let nums = rgx
                .captures_iter(l)
                .filter_map(|c| {
                    c.ok().and_then(|cc| {
                        cc.get(0).and_then(|m| {
                            m.as_str()
                                .replace("on", "1")
                                .replace("tw", "2")
                                .replace("thre", "3")
                                .replace("four", "4")
                                .replace("fiv", "5")
                                .replace("six", "6")
                                .replace("seve", "7")
                                .replace("eigh", "8")
                                .replace("nin", "9")
                                .parse::<u32>()
                                .ok()
                        })
                    })
                })
                .collect::<Vec<u32>>();

            nums.first().zip(nums.last()).map(|(f, l)| f * 10 + l)
        })
        .collect::<Vec<u32>>();

    lls.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE_1: &str = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const TEST_CASE_2: &str = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[rstest]
    #[case(TEST_CASE_1, "142")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE_2, "281")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("54951")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("55218")]
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
        b.iter(|| process_part_2(TEST_CASE_2));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_2(&input));
    }
}
