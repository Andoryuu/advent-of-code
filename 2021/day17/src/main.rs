use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::fs;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let (_, _, y, _) = parse_values(input);

    (0..y.abs()).sum::<i32>().to_string()
}

fn process_data_adv(input: String) -> String {
    let (x1, x2, y1, y2) = parse_values(input);
    let x_min = ((x1 as f64).sqrt() / 2f64).round() as u32;
    let x_max = x2;
    let y_min = y1;
    let y_max = y1.abs() - 1;

    let hits = (y_min..=y_max)
        .flat_map(|y| (x_min..=x_max).map(move |x| Trajectory::new(x, y)))
        .filter_map(|t| {
            t.map(|(x, y)| {
                if x > x2 || y < y1 {
                    ShotResult::Miss
                } else if x < x1 || y > y2 {
                    ShotResult::None
                } else {
                    ShotResult::Hit
                }
            })
            .find(|r| *r != ShotResult::None)
        })
        .filter(|r| *r == ShotResult::Hit);

    hits.count().to_string()
}

fn parse_values(input: String) -> (u32, u32, i32, i32) {
    lazy_static! {
        static ref LINE_RE: Regex =
            Regex::new("target area: x=(\\d+)..(\\d+), y=-(\\d+)..-(\\d+)").unwrap();
    }

    let caps = LINE_RE.captures(input.trim()).unwrap();
    let x1 = parse_capture(caps.get(1));
    let x2 = parse_capture(caps.get(2));
    let y1 = -parse_capture::<i32>(caps.get(3));
    let y2 = -parse_capture::<i32>(caps.get(4));

    (x1, x2, y1, y2)
}

fn parse_capture<T: std::str::FromStr>(cap: Option<Match>) -> T {
    cap.and_then(|m| m.as_str().parse::<T>().ok()).unwrap()
}

#[derive(Debug, std::cmp::PartialEq)]
enum ShotResult {
    None,
    Hit,
    Miss,
}

struct Trajectory {
    v_x: u32,
    v_y: i32,
    x: u32,
    y: i32,
}

impl Trajectory {
    fn new(init_vx: u32, init_vy: i32) -> Self {
        Trajectory {
            v_x: init_vx,
            v_y: init_vy,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for Trajectory {
    type Item = (u32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.x += self.v_x;
        self.y += self.v_y;
        self.v_x = if self.v_x > 0 { self.v_x - 1 } else { 0 };
        self.v_y -= 1;

        Some((self.x, self.y))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "target area: x=20..30, y=-10..-5
    ";

    #[test]
    fn base_check() {
        assert_eq!("45", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("112", process_data_adv(TEST_CASE.to_string()));
    }
}
