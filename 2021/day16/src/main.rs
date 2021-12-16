#![feature(is_sorted)]

mod packet;

use std::fs;

use packet::Packet;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    parse(input)
        .iter()
        .map(|p| p.version_sum())
        .sum::<u32>()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    parse(input)
        .iter()
        .map(|p| p.evaluate())
        .sum::<u64>()
        .to_string()
}

fn parse(input: String) -> Vec<Packet> {
    Packet::parse_from(
        input
            .trim()
            .chars()
            .map(hex_to_binary)
            .collect::<Vec<String>>()
            .join(""),
    )
}

fn hex_to_binary(input: char) -> String {
    let output = match input {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        x => panic!("Unknown hex: {}", x),
    };

    output.to_owned()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    #[rstest]
    #[case("EE00D40C823060", "14")]
    #[case("8A004A801A8002F478", "16")]
    #[case("620080001611562C8802118E34", "12")]
    #[case("C0015000016115A2E0802F182340", "23")]
    #[case("A0016C880162017C3686B18A3D4780", "31")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_string()));
    }

    #[rstest]
    #[case("C200B40A82", "3")]
    #[case("04005AC33890", "54")]
    #[case("880086C3E88112", "7")]
    #[case("CE00C43D881120", "9")]
    #[case("D8005AC2A8F0", "1")]
    #[case("F600BC2D8F", "0")]
    #[case("9C005AC2F8F0", "0")]
    #[case("9C0141080250320F1802104A08", "1")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_string()));
    }
}
