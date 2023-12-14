#![feature(test)]

use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day14/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let lines = input.lines().filter(|l| !l.is_empty()).collect_vec();

    let width = lines.first().unwrap().len();
    let height = lines.len();

    lines
        .into_iter()
        .enumerate()
        .fold(
            (0, vec![height + 1; width]),
            |(mut sum, mut walls), (row, line)| {
                for (i, c) in line.char_indices() {
                    match c {
                        '#' => walls[i] = height - row,
                        'O' => {
                            walls[i] -= 1;
                            sum += walls[i];
                        }
                        _ => {}
                    }
                }

                (sum, walls)
            },
        )
        .0
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let lines = input.lines().filter(|l| !l.is_empty()).collect_vec();

    let width = lines.first().unwrap().len();
    let height = lines.len();

    let mut buff = vec![Vec::<char>::new(); 1000];
    let mut buff_i = 0usize;
    buff[buff_i] = lines.concat().chars().collect_vec();

    let (last_i, rep_buff_i) = (0..1_000_000_000)
        .find_map(|i| {
            let new_state = apply_cycle(buff[buff_i].clone(), width, height);

            if let Some((bi, _)) = buff.iter().find_position(|s| **s == new_state) {
                return Some((i, bi));
            }

            buff_i = (buff_i + 1) % buff.len();
            buff[buff_i] = new_state;

            None
        })
        .unwrap();

    let cycle_len = (buff_i + 1 + buff.len() - rep_buff_i) % buff.len();
    let buff_ofs = (1_000_000_000 - last_i - 1) % cycle_len;

    buff[rep_buff_i + buff_ofs]
        .chunks(width)
        .enumerate()
        .map(|(row, line)| line.iter().filter(|c| **c == 'O').count() * (height - row))
        .sum::<usize>()
        .to_string()
}

fn apply_cycle(mut prev_state: Vec<char>, width: usize, height: usize) -> Vec<char> {
    for col in 0..width {
        let mut wall = 0;
        for row in 0..height {
            let i = col + (row * width);
            match prev_state[i] {
                '#' => wall = row + 1,
                'O' => {
                    prev_state[i] = '.';
                    prev_state[col + (wall * width)] = 'O';
                    wall += 1;
                }
                _ => {}
            }
        }
    }

    for row in 0..height {
        let mut wall = 0;
        for col in 0..width {
            let i = (row * width) + col;
            match prev_state[i] {
                '#' => wall = col + 1,
                'O' => {
                    prev_state[i] = '.';
                    prev_state[(row * width) + wall] = 'O';
                    wall += 1;
                }
                _ => {}
            }
        }
    }

    for col in 0..width {
        let mut wall = height;
        for row in (0..height).rev() {
            let i = col + (row * width);
            match prev_state[i] {
                '#' => wall = row,
                'O' => {
                    wall -= 1;
                    prev_state[i] = '.';
                    prev_state[col + (wall * width)] = 'O';
                }
                _ => {}
            }
        }
    }

    for row in 0..height {
        let mut wall = width;
        for col in (0..width).rev() {
            let i = (row * width) + col;
            match prev_state[i] {
                '#' => wall = col,
                'O' => {
                    wall -= 1;
                    prev_state[i] = '.';
                    prev_state[(row * width) + wall] = 'O';
                }
                _ => {}
            }
        }
    }

    prev_state
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[rstest]
    #[case(TEST_CASE, "136")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "64")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("107430")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("96317")]
    fn part_2_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_2(&input));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_1(&input));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_2(&input));
    }
}
