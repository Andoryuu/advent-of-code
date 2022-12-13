use std::{cmp::Ordering, fs, iter::once};

use itertools::{EitherOrBoth, Itertools};
use nom::{
    branch::alt,
    character::complete::{char, digit0},
    combinator::{map, map_opt},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    input
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)| {
            group
                .into_iter()
                .filter_map(parse_packet)
                .next_tuple()
                .unwrap()
        })
        .enumerate()
        .filter_map(|(ix, (left, right))| match left.cmp(&right) {
            Ordering::Less => Some(ix + 1),
            _ => None,
        })
        .sum::<usize>()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    let fst_divider_packet = parse_packet("[[2]]").unwrap();
    let snd_divider_packet = parse_packet("[[6]]").unwrap();

    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(parse_packet)
        .chain(once(fst_divider_packet.clone()))
        .chain(once(snd_divider_packet.clone()))
        .sorted()
        .enumerate()
        .filter(|(_, packet)| packet == &fst_divider_packet || packet == &snd_divider_packet)
        .map(|(ix, _)| ix + 1)
        .next_tuple()
        .map(|(fst, snd)| fst * snd)
        .unwrap()
        .to_string()
}

fn parse_packet(input: &str) -> Option<PacketPart> {
    parse_packet_part(input)
        .and_then(|(rest, packet)| {
            if !rest.is_empty() {
                panic!("unparsed items: {}", rest)
            } else {
                Ok(packet)
            }
        })
        .ok()
}

fn parse_packet_part(input: &str) -> IResult<&str, PacketPart, ()> {
    let map_num = map_opt(digit0, |s: &str| s.parse::<u32>().map(PacketPart::Num).ok());

    let map_list = map(
        delimited(
            char('['),
            separated_list0(char(','), parse_packet_part),
            char(']'),
        ),
        PacketPart::List,
    );

    alt((map_num, map_list))(input)
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum PacketPart {
    Num(u32),
    List(Vec<PacketPart>),
}

impl PartialOrd for PacketPart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketPart {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketPart::Num(s_num), PacketPart::Num(o_num)) => s_num.cmp(o_num),
            (PacketPart::List(s_list), PacketPart::List(o_list)) => s_list
                .iter()
                .zip_longest(o_list)
                .map(|z| match z {
                    EitherOrBoth::Both(s_part, o_part) => s_part.cmp(o_part),
                    EitherOrBoth::Left(_) => Ordering::Greater,
                    EitherOrBoth::Right(_) => Ordering::Less,
                })
                .find(|&o| o != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
            (s_part, PacketPart::List(o_list)) => once(s_part)
                .zip_longest(o_list)
                .map(|z| match z {
                    EitherOrBoth::Both(s_part, o_part) => s_part.cmp(o_part),
                    EitherOrBoth::Left(_) => Ordering::Greater,
                    EitherOrBoth::Right(_) => Ordering::Less,
                })
                .find(|&o| o != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
            (PacketPart::List(s_list), o_part) => s_list
                .iter()
                .zip_longest(once(o_part))
                .map(|z| match z {
                    EitherOrBoth::Both(s_part, o_part) => s_part.cmp(o_part),
                    EitherOrBoth::Left(_) => Ordering::Greater,
                    EitherOrBoth::Right(_) => Ordering::Less,
                })
                .find(|&o| o != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[rstest]
    #[case(TEST_CASE, "13")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned()));
    }

    #[rstest]
    #[case(TEST_CASE, "140")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned()));
    }
}
