#[derive(Debug)]
pub enum PacketType {
    LiteralValue(u64),
    SumOperator,
    ProductOperator,
    MinimumOperator,
    MaximumOperator,
    GreaterThanOperator,
    LessThanOperator,
    EqualToOperator,
}

#[derive(Debug)]
pub struct Packet {
    version: u32,
    packet_type: PacketType,
    children: Vec<Packet>,
}

impl Packet {
    pub fn parse_from(input: String) -> Vec<Self> {
        Self::parse(&input[..])
    }

    pub fn version_sum(&self) -> u32 {
        self.version + self.children.iter().map(|c| c.version_sum()).sum::<u32>()
    }

    pub fn evaluate(&self) -> u64 {
        match self.packet_type {
            PacketType::LiteralValue(x) => x,
            PacketType::SumOperator => self.children.iter().map(|c| c.evaluate()).sum(),
            PacketType::ProductOperator => {
                if self.children.is_empty() {
                    0
                } else {
                    self.children.iter().map(|c| c.evaluate()).product()
                }
            }
            PacketType::MinimumOperator => {
                self.children.iter().map(|c| c.evaluate()).min().unwrap()
            }
            PacketType::MaximumOperator => {
                self.children.iter().map(|c| c.evaluate()).max().unwrap()
            }
            PacketType::GreaterThanOperator => {
                if self.children.iter().map(|c| c.evaluate()).is_sorted() {
                    0
                } else {
                    1
                }
            }
            PacketType::LessThanOperator => {
                if self.children.iter().map(|c| c.evaluate()).rev().is_sorted() {
                    0
                } else {
                    1
                }
            }
            PacketType::EqualToOperator => {
                if self
                    .children
                    .iter()
                    .map(|c| c.evaluate())
                    .fold((true, None), |(b, prev), curr| {
                        (b && prev.map(|p| p == curr).unwrap_or(true), Some(curr))
                    })
                    .0
                {
                    1
                } else {
                    0
                }
            }
        }
    }

    fn parse(input: &str) -> Vec<Self> {
        let mut output = Vec::new();
        let mut rest = input;

        while rest.len() > 16 || !rest.chars().all(|c| c == '0') {
            let (packet, new_rest) = Self::parse_single(rest);
            rest = new_rest;
            output.push(packet);
        }

        output
    }

    fn parse_single(input: &str) -> (Self, &str) {
        let version = u32::from_str_radix(&input[0..3], 2).unwrap();
        let packet_num_type = u32::from_str_radix(&input[3..6], 2).unwrap();

        if packet_num_type == 4 {
            let (value, rest) = Self::parse_literal_value(&input[6..]);

            (
                Packet {
                    version,
                    packet_type: PacketType::LiteralValue(value),
                    children: vec![],
                },
                rest,
            )
        } else {
            let (children, rest) = Self::parse_operator(&input[6..]);

            (
                Packet {
                    version,
                    packet_type: match packet_num_type {
                        0 => PacketType::SumOperator,
                        1 => PacketType::ProductOperator,
                        2 => PacketType::MinimumOperator,
                        3 => PacketType::MaximumOperator,
                        5 => PacketType::GreaterThanOperator,
                        6 => PacketType::LessThanOperator,
                        7 => PacketType::EqualToOperator,
                        x => panic!("Unknown packet type: {}", x),
                    },
                    children,
                },
                rest,
            )
        }
    }

    fn parse_literal_value(input: &str) -> (u64, &str) {
        let mut value_parts = Vec::new();
        let mut index = 0;

        loop {
            value_parts.push(&input[(index + 1)..(index + 5)]);

            if input[index..].starts_with('0') {
                break;
            }

            index += 5;
        }

        (
            u64::from_str_radix(&value_parts.join(""), 2).unwrap(),
            &input[(index + 5)..],
        )
    }

    fn parse_operator(input: &str) -> (Vec<Packet>, &str) {
        if input.starts_with('0') {
            let size = usize::from_str_radix(&input[1..16], 2).unwrap();
            let packets = Self::parse(&input[16..(16 + size)]);

            (packets, &input[(16 + size)..])
        } else {
            let count = usize::from_str_radix(&input[1..12], 2).unwrap();

            Self::parse_next_n_packets(&input[12..], count)
        }
    }

    fn parse_next_n_packets(input: &str, count: usize) -> (Vec<Packet>, &str) {
        let mut packets = Vec::with_capacity(count);
        let mut rest = input;

        for _ in 0..count {
            let (packet, new_rest) = Self::parse_single(rest);
            rest = new_rest;
            packets.push(packet);
        }

        (packets, rest)
    }
}
