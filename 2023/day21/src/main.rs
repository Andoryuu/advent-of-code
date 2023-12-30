#![feature(test)]

use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use itertools::Itertools;
use num::Integer;

fn main() {
    let input = fs::read_to_string("./day21/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input, 64));
    println!("Part 2 result is: {}", process_part_2(&input, 26501365));
}

fn process_part_1(input: &str, steps: usize) -> String {
    let (field, start) = parse(input);

    calculate_steps(&field, start, steps).to_string()
}

fn process_part_2(input: &str, steps: usize) -> String {
    let (field, (s_row, s_col)) = parse(input);
    let size = field.len();

    let even_end_step_tile = calculate_steps(&field, (s_row, s_col), 1000);
    let odd_end_step_tile = calculate_steps(&field, (s_row, s_col), 1001);

    let (count, rem) = (steps - (size / 2)).div_rem(&size);

    let (count, rem) = if rem == 0 {
        (count - 1, size)
    } else {
        panic!("let's not bother")
    };

    let end = size - 1;

    let up = calculate_steps(&field, (end, s_col), rem - 1);
    let down = calculate_steps(&field, (0, s_col), rem - 1);
    let left = calculate_steps(&field, (s_row, 0), rem - 1);
    let right = calculate_steps(&field, (s_row, end), rem - 1);

    let steps_out_corn = rem - 2 - (size / 2);
    let steps_in_corn = rem + size - 2 - (size / 2);

    let up_left_out = calculate_steps(&field, (end, end), steps_out_corn);
    let up_left = calculate_steps(&field, (end, end), steps_in_corn);
    let up_right_out = calculate_steps(&field, (end, 0), steps_out_corn);
    let up_right = calculate_steps(&field, (end, 0), steps_in_corn);
    let down_right_out = calculate_steps(&field, (0, 0), steps_out_corn);
    let down_right = calculate_steps(&field, (0, 0), steps_in_corn);
    let down_left_out = calculate_steps(&field, (0, end), steps_out_corn);
    let down_left = calculate_steps(&field, (0, end), steps_in_corn);

    let odd_tiles = 4
        * (1 + (if count.is_odd() || count == 0 {
            count
        } else {
            count - 1
        }))
        * (count - (count / 2))
        / 2;

    let even_tiles = 4
        * (2 + (if count.is_even() || count == 0 {
            count
        } else {
            count - 1
        }))
        * (count / 2)
        / 2;

    ((up + down + left + right)
        + count * up_left
        + count * up_right
        + count * down_right
        + count * down_left
        + (count + 1) * up_left_out
        + (count + 1) * up_right_out
        + (count + 1) * down_right_out
        + (count + 1) * down_left_out
        + (if steps.is_odd() { odd_end_step_tile } else { even_end_step_tile }) // middle
        + odd_tiles * (if (steps - (size / 2)).is_odd() { odd_end_step_tile } else { even_end_step_tile })
        + even_tiles * (if (steps - (size / 2) - size).is_odd() { odd_end_step_tile } else { even_end_step_tile }))
        .to_string()
}

fn parse(input: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let mut start = None;

    (
        input
            .trim()
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.char_indices()
                    .map(|(col, c)| {
                        if c == 'S' {
                            start = Some((row, col));
                            return true;
                        }

                        c == '.'
                    })
                    .collect_vec()
            })
            .collect_vec(),
        start.unwrap(),
    )
}

fn calculate_steps(field: &[Vec<bool>], start: (usize, usize), steps: usize) -> usize {
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut queue = VecDeque::new();
    queue.push_back((start, steps));

    let mut total = 0usize;

    while let Some(((row, col), rem)) = queue.pop_front() {
        if rem.is_even() {
            total += 1;
        }

        if rem == 0 {
            continue;
        }

        if row > 0 {
            let up = (row - 1, col);
            if field[row - 1][col] && !visited.contains(&up) {
                queue.push_back((up, rem - 1));
                visited.insert(up);
            }
        }

        let down = (row + 1, col);
        if let Some(r) = field.get(row + 1) {
            if r[col] && !visited.contains(&down) {
                queue.push_back((down, rem - 1));
                visited.insert(down);
            }
        }

        if col > 0 {
            let left = (row, col - 1);
            if field[row][col - 1] && !visited.contains(&left) {
                queue.push_back((left, rem - 1));
                visited.insert(left);
            }
        }

        let right = (row, col + 1);
        if let Some(c) = field[row].get(col + 1) {
            if *c && !visited.contains(&right) {
                queue.push_back((right, rem - 1));
                visited.insert(right);
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[rstest]
    #[case(TEST_CASE, 6, "16")]
    fn part_1_check(#[case] input: &str, #[case] steps: usize, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input, steps));
    }

    #[rstest]
    #[case(TEST_CASE, 6, "16")]
    #[case(TEST_CASE, 10, "50")]
    #[case(TEST_CASE, 50, "1594")]
    #[case(TEST_CASE, 100, "6536")]
    #[case(TEST_CASE, 500, "167004")]
    #[case(TEST_CASE, 1000, "668697")]
    fn part_2_brute_check(#[case] input: &str, #[case] steps: usize, #[case] expected: &str) {
        assert_eq!(expected, process_part_2_brute(input, steps));
    }

    #[rstest]
    #[case(196)]
    #[case(327)]
    #[case(458)]
    #[case(589)]
    fn part_2_check(#[case] steps: usize) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(
            process_part_2_brute(&input, steps),
            process_part_2(&input, steps),
        )
    }

    #[rstest]
    #[case("3615")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input, 64));
    }

    #[rstest]
    #[case("602259568764234")]
    fn part_2_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_2(&input, 26501365));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE, 6));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_1(&input, 64));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2_brute(TEST_CASE, 500));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_2(&input, 26501365));
    }

    fn process_part_2_brute(input: &str, steps: usize) -> String {
        let (field, (s_row, s_col)) = parse(input);

        let width = field.first().unwrap().len() as isize;
        let height = field.len() as isize;

        let mut total = 0usize;

        let mut visited = HashSet::new();
        visited.insert((s_row as isize, s_col as isize));

        let mut queue = VecDeque::new();
        queue.push_back(((s_row as isize, s_col as isize), steps));

        let neighbs = vec![(-1isize, 0isize), (1, 0), (0, -1), (0, 1)];

        while let Some(((row, col), rem)) = queue.pop_front() {
            if rem.is_even() {
                total += 1;
            }

            if rem == 0 {
                continue;
            }

            for (row_ofs, col_ofs) in &neighbs {
                let row = row + row_ofs;
                let col = col + col_ofs;

                let next = (row, col);

                if visited.contains(&next) {
                    continue;
                }

                if let Some(r) = field.get(row.rem_euclid(height) as usize) {
                    if let Some(c) = r.get(col.rem_euclid(width) as usize) {
                        if *c {
                            queue.push_back(((row, col), rem - 1));
                            visited.insert((row, col));
                        }
                    }
                }
            }
        }

        total.to_string()
    }
}
