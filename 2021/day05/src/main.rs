use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::{collections::BTreeMap, fs};

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let mut vents: BTreeMap<(u32, u32), u32> = BTreeMap::new();

    for line in parse(input)
        .iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
    {
        if line.start.x == line.end.x {
            let x = line.start.x;

            for y in get_range(line.start.y, line.end.y) {
                let vent = vents.entry((x, y)).or_insert(0);
                *vent += 1;
            }
        } else {
            let y = line.start.y;

            for x in get_range(line.start.x, line.end.x) {
                let vent = vents.entry((x, y)).or_insert(0);
                *vent += 1;
            }
        }
    }

    vents.into_iter().filter(|v| v.1 > 1).count().to_string()
}

fn process_data_adv(input: String) -> String {
    let mut vents: BTreeMap<(u32, u32), u32> = BTreeMap::new();

    for line in parse(input).iter() {
        if line.start.x == line.end.x {
            let x = line.start.x;

            for y in get_range(line.start.y, line.end.y) {
                let vent = vents.entry((x, y)).or_insert(0);
                *vent += 1;
            }
        } else if line.start.y == line.end.y {
            let y = line.start.y;

            for x in get_range(line.start.x, line.end.x) {
                let vent = vents.entry((x, y)).or_insert(0);
                *vent += 1;
            }
        } else {
            let range_x = get_range(line.start.x, line.end.x);
            let range_y = get_range(line.start.y, line.end.y);

            for (x, y) in range_x.iter().zip(range_y) {
                let vent = vents.entry((*x, y)).or_insert(0);
                *vent += 1;
            }
        }
    }

    vents.into_iter().filter(|v| v.1 > 1).count().to_string()
}

fn parse(input: String) -> Vec<Line> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new("(\\d+),(\\d+) -> (\\d+),(\\d+)").unwrap();
    }

    input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let caps = LINE_RE.captures(s).unwrap();

            Line {
                start: Point {
                    x: parse_capture(caps.get(1)).unwrap(),
                    y: parse_capture(caps.get(2)).unwrap(),
                },
                end: Point {
                    x: parse_capture(caps.get(3)).unwrap(),
                    y: parse_capture(caps.get(4)).unwrap(),
                },
            }
        })
        .collect()
}

fn parse_capture(mat: Option<Match>) -> Option<u32> {
    mat.map(|m| m.as_str()).and_then(|s| s.parse::<u32>().ok())
}

fn get_range(start: u32, end: u32) -> Vec<u32> {
    if start < end {
        (start..=end).collect()
    } else {
        (end..=start).rev().collect()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

    #[test]
    fn parse_check() {
        let lines = parse(TEST_CASE.to_string());

        assert_eq!(10, lines.len());

        let test_line = Line {
            start: Point { x: 0, y: 9 },
            end: Point { x: 5, y: 9 },
        };

        assert_eq!(test_line, *lines.get(0).unwrap());
    }

    #[test]
    fn base_check() {
        assert_eq!("5", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("12", process_data_adv(TEST_CASE.to_string()));
    }
}
