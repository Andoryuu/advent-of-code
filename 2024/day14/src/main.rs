#![feature(test)]

use std::fs;

use fxhash::FxHashSet;
use itertools::Itertools;
use num::Integer;

fn main() {
    let input = fs::read_to_string("./day14/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input, 101, 103));
    println!("Part 2 result is: {}", process_part_2(&input, 101, 103));
}

fn process_part_1(input: &str, width: isize, height: isize) -> String {
    let mid_x = width / 2;
    let mid_y = height / 2;

    let q = parse(input)
        .into_iter()
        .fold((0, 0, 0, 0), |mut acc, mut r| {
            r.move_ntimes(100, width, height);
            let (x, y) = r.position;

            if x != mid_x && y != mid_y {
                if y < mid_y {
                    if x < mid_x {
                        acc.0.inc();
                    } else {
                        acc.1.inc();
                    }
                } else if x < mid_x {
                    acc.2.inc();
                } else {
                    acc.3.inc();
                }
            }

            acc
        });

    (q.0 * q.1 * q.2 * q.3).to_string()
}

fn process_part_2(input: &str, width: isize, height: isize) -> String {
    let mut robots = parse(input);
    let robots_len = robots.len();
    let mut points = FxHashSet::default();

    for i in 1.. {
        points.clear();

        for robot in robots.iter_mut() {
            robot.move_ntimes(1, width, height);
            points.insert(robot.position);
        }

        // image appears when no robots share position
        if points.len() == robots_len {
            return i.to_string();
        }
    }

    unreachable!()
}

#[derive(Debug)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn move_ntimes(&mut self, seconds: isize, width: isize, height: isize) {
        let Robot {
            position: (p_x, p_y),
            velocity: (v_x, v_y),
        } = *self;

        self.position = (
            ((v_x + width) * seconds + p_x) % width,
            ((v_y + height) * seconds + p_y) % height,
        );
    }
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            line.split(['=', ' ', ','])
                .filter_map(|p| p.parse().ok())
                .collect_tuple()
        })
        .map(|(p_x, p_y, v_x, v_y)| Robot {
            position: (p_x, p_y),
            velocity: (v_x, v_y),
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "12")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input, 11, 7));
    }

    #[rstest]
    #[case("224438715")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input, 101, 103));
    }

    #[rstest]
    #[case("7603")]
    fn part_2_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(&input, 101, 103));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE, 11, 7));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_1(&input, 101, 103));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_2(&input, 101, 103));
    }
}
