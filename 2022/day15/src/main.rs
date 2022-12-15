use std::{fs, iter::once};

use geo::{coord, polygon, BooleanOps, Centroid, MultiPolygon, Rect};
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone(), 2_000_000);
    let adv_output = process_data_adv(input, 4_000_000);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String, line: isize) -> String {
    let mut line_beacons = Vec::<isize>::new();
    let mut line_ranges = Vec::<(isize, isize)>::new();

    for (sensor, beacon) in input.lines().map(parse_line) {
        let range = get_range(&sensor, &beacon);
        let line_dist = sensor.y.abs_diff(line) as isize;

        if line_dist <= range {
            let sideways = range - line_dist;
            line_ranges.push((sensor.x - sideways, sensor.x + sideways));
        }

        if beacon.y == line {
            line_beacons.push(beacon.x);
        }
    }

    line_beacons.dedup();

    line_ranges
        .iter()
        .sorted()
        .fold(Vec::<(isize, isize)>::new(), |mut acc, &r| {
            if let Some((last, elements)) = acc.split_last() {
                if last.1 < r.0 {
                    acc.push(r);
                    acc
                } else {
                    elements
                        .iter()
                        .cloned()
                        .chain(once((last.0, last.1.max(r.1))))
                        .collect_vec()
                }
            } else {
                vec![r]
            }
        })
        .iter()
        .map(|(x_s, x_e)| {
            x_e - x_s + 1
                - line_beacons
                    .iter()
                    .filter(|b_x| (x_s..=x_e).contains(b_x))
                    .count() as isize
        })
        .sum::<isize>()
        .to_string()
}

fn process_data_adv(input: String, max: usize) -> String {
    let max_f = max as f64;

    let bounding_poly =
        Rect::new(coord! { x: 0.0, y: 0.0 }, coord! { x: max_f, y: max_f }).to_polygon();

    let mut geometry = MultiPolygon::from(vec![bounding_poly]);

    for sensor_poly in input
        .lines()
        .map(parse_line)
        .map(|(sensor, beacon)| (get_range(&sensor, &beacon) as f64 + 0.5, sensor))
        .map(|(range, sensor)| {
            let (x, y) = (sensor.x as f64, sensor.y as f64);
            polygon![
                (x: x, y: y - range),
                (x: x + range, y: y),
                (x: x, y: y + range),
                (x: x - range, y: y),
                (x: x, y: y - range),
            ]
        })
    {
        geometry = geometry.difference(&MultiPolygon::from(vec![sensor_poly]));
    }

    let c = geometry.centroid().unwrap();

    (c.x().round() as u64 * 4_000_000 + c.y().round() as u64).to_string()
}

fn parse_line(line: &str) -> (Sensor, Beacon) {
    let coor_rgx = Regex::new("x=(?P<x>-?\\d+), y=(?P<y>-?\\d+)").unwrap();
    let res = coor_rgx
        .captures_iter(line)
        .map(|cap| {
            (
                cap["x"].parse::<isize>().unwrap(),
                cap["y"].parse::<isize>().unwrap(),
            )
        })
        .next_tuple()
        .map(|(sen, beac)| {
            (
                Sensor { x: sen.0, y: sen.1 },
                Beacon {
                    x: beac.0,
                    y: beac.1,
                },
            )
        })
        .unwrap();

    res
}

fn get_range(sensor: &Sensor, beacon: &Beacon) -> isize {
    (sensor.x.abs_diff(beacon.x) + sensor.y.abs_diff(beacon.y)) as isize
}

struct Sensor {
    x: isize,
    y: isize,
}

struct Beacon {
    x: isize,
    y: isize,
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[rstest]
    #[case(TEST_CASE, 10, "26")]
    fn base_check(#[case] input: &str, #[case] line: isize, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned(), line));
    }

    #[rstest]
    #[case(TEST_CASE, 20, "56000011")]
    fn adv_check(#[case] input: &str, #[case] max: usize, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned(), max));
    }
}
