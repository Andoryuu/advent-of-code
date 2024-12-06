#![feature(test)]

use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("./day06/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let Data {
        mut guard_pos,
        obstacles,
        max_size,
    } = parse(input);

    let mut direction = Direction::Up;
    let mut visited = HashSet::from([guard_pos]);

    while let Some(next_pos) = try_get_next_pos(guard_pos, &direction, max_size) {
        if obstacles.contains(&next_pos) {
            direction = turn_right(&direction)
        } else {
            visited.insert(next_pos);
            guard_pos = next_pos;
        }
    }

    visited.len().to_string()
}

fn process_part_2(input: &str) -> String {
    let Data {
        mut guard_pos,
        obstacles,
        max_size,
    } = parse(input);

    let mut direction = Direction::Up;
    let mut visited = HashMap::from([(guard_pos, HashSet::from([direction]))]);
    let mut obstructions = HashSet::new();

    while let Some(next_pos) = try_get_next_pos(guard_pos, &direction, max_size) {
        if obstacles.contains(&next_pos) {
            direction = turn_right(&direction);
            visited
                .entry(guard_pos)
                .and_modify(|v| {
                    v.insert(direction);
                })
                .or_insert_with(|| HashSet::from([direction]));
        } else {
            if !visited.contains_key(&next_pos)
                && is_valid_obstruction(
                    next_pos, direction, guard_pos, max_size, &visited, &obstacles,
                )
            {
                obstructions.insert(next_pos);
            }

            visited
                .entry(next_pos)
                .and_modify(|v| {
                    v.insert(direction);
                })
                .or_insert_with(|| HashSet::from([direction]));
            guard_pos = next_pos;
        }
    }

    obstructions.len().to_string()
}

fn is_valid_obstruction(
    next_pos: (usize, usize),
    direction: Direction,
    guard_pos: (usize, usize),
    max_size: (usize, usize),
    visited: &HashMap<(usize, usize), HashSet<Direction>>,
    obstacles: &HashSet<(usize, usize)>,
) -> bool {
    let mut alt_dir = turn_right(&direction);
    let mut alt_pos = guard_pos;
    let mut alt_visited = HashSet::from([(alt_pos, alt_dir)]);

    while let Some(next_alt_pos) = try_get_next_pos(alt_pos, &alt_dir, max_size) {
        if obstacles.contains(&next_alt_pos) || next_pos == next_alt_pos {
            alt_dir = turn_right(&alt_dir);
            alt_visited.insert((alt_pos, alt_dir));
        } else {
            let next_alt_dir_pos = (next_alt_pos, alt_dir);

            if alt_visited.contains(&next_alt_dir_pos)
                || visited
                    .get(&next_alt_pos)
                    .is_some_and(|v| v.contains(&alt_dir))
            {
                return true;
            }

            alt_visited.insert(next_alt_dir_pos);
            alt_pos = next_alt_pos;
        }
    }

    false
}

fn try_get_next_pos(
    pos: (usize, usize),
    direction: &Direction,
    max_size: (usize, usize),
) -> Option<(usize, usize)> {
    let (pos_row, pos_col) = pos;
    let (max_row, max_col) = max_size;
    match direction {
        Direction::Up => {
            if pos_row == 0 {
                None
            } else {
                Some((pos_row - 1, pos_col))
            }
        }
        Direction::Down => {
            if pos_row == max_row {
                None
            } else {
                Some((pos_row + 1, pos_col))
            }
        }
        Direction::Left => {
            if pos_col == 0 {
                None
            } else {
                Some((pos_row, pos_col - 1))
            }
        }
        Direction::Right => {
            if pos_col == max_col {
                None
            } else {
                Some((pos_row, pos_col + 1))
            }
        }
    }
}

fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Data {
    guard_pos: (usize, usize),
    obstacles: HashSet<(usize, usize)>,
    max_size: (usize, usize),
}

fn parse(input: &str) -> Data {
    let mut guard_pos = None;
    let mut obstacles = HashSet::new();
    let mut max_row = 0;
    let mut max_col = 0;

    for (row, line) in input.lines().filter(|line| !line.is_empty()).enumerate() {
        if max_col == 0 {
            max_col = line.len() - 1;
        }
        if max_row < row {
            max_row = row;
        }

        for (col, ch) in line.char_indices() {
            if ch == '^' {
                guard_pos = Some((row, col));
            } else if ch == '#' {
                obstacles.insert((row, col));
            }
        }
    }

    Data {
        guard_pos: guard_pos.unwrap(),
        obstacles,
        max_size: (max_row, max_col),
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "41")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "6")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("4656")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("1575")]
    fn part_2_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(&input));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_1(&input));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_2(&input));
    }
}
