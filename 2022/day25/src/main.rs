#![feature(int_log)]

use std::fs;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input);

    println!("Result is: {}", output);
}

fn process_data(input: String) -> String {
    to_snafu(input.lines().map(from_snafu).sum::<i64>())
}

fn from_snafu(input: &str) -> i64 {
    input
        .chars()
        .rev()
        .map(|c| match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("unknown snafu digit: {c}"),
        })
        .enumerate()
        .map(|(ix, d)| 5i64.pow(ix as u32) * d)
        .sum::<i64>()
}

fn to_snafu(input: i64) -> String {
    let mut base5 = vec![0];
    let mut rem = input;
    for ix in (0..=input.ilog(5)).rev() {
        let o = 5i64.pow(ix);
        base5.push(rem / o);
        rem %= o;
    }

    let mut snafu = vec![0; base5.len() * 2];
    for (ix, d) in base5.iter().rev().enumerate() {
        let d = d + snafu[ix];
        if d < 3 {
            snafu[ix] = d;
        } else {
            snafu[ix] = d - 5;
            snafu[ix + 1] = 1;
        }
    }

    snafu
        .iter()
        .rev()
        .skip_while(|d| **d == 0)
        .map(|d| match d {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!("unexpected snafu value: {d}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    #[rstest]
    #[case(TEST_CASE, "2=-1=0")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned()));
    }

    #[rstest]
    #[case(1, "1")]
    #[case(2, "2")]
    #[case(3, "1=")]
    #[case(4, "1-")]
    #[case(5, "10")]
    #[case(6, "11")]
    #[case(7, "12")]
    #[case(8, "2=")]
    #[case(9, "2-")]
    #[case(10, "20")]
    #[case(15, "1=0")]
    #[case(20, "1-0")]
    #[case(2022, "1=11-2")]
    #[case(12345, "1-0---0")]
    #[case(314159265, "1121-1110-1=0")]
    fn to_snafu_conversion(#[case] input: i64, #[case] expected: &str) {
        assert_eq!(expected, to_snafu(input));
    }

    #[rstest]
    #[case("1=-0-2", 1747)]
    #[case("12111", 906)]
    #[case("2=0=", 198)]
    #[case("21", 11)]
    #[case("2=01", 201)]
    #[case("111", 31)]
    #[case("20012", 1257)]
    #[case("112", 32)]
    #[case("1=-1=", 353)]
    #[case("1-12", 107)]
    #[case("12", 7)]
    #[case("1=", 3)]
    #[case("122", 37)]
    fn from_snafu_conversion(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(expected, from_snafu(input));
    }
}
