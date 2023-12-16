#![feature(test)]

use std::{collections::HashSet, fs};

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let input = fs::read_to_string("./day16/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let (tiles, width, height) = parse(input);

    let beams = tiles.first().unwrap().interact(Dir::Right).into_vec(0, 0);

    get_energy_value(beams, tiles, width, height).to_string()
}

fn process_part_2(input: &str) -> String {
    let (tiles, width, height) = parse(input);

    let last_col = width - 1;
    let last_row = height - 1;

    let up_beams = tiles
        .iter()
        .skip(last_row * width)
        .enumerate()
        .map(|(col, t)| t.interact(Dir::Up).into_vec(last_row, col))
        .collect_vec();

    let right_beams = tiles
        .iter()
        .step_by(width)
        .enumerate()
        .map(|(row, t)| t.interact(Dir::Right).into_vec(row, 0))
        .collect_vec();

    let down_beams = tiles
        .iter()
        .take(width)
        .enumerate()
        .map(|(col, t)| t.interact(Dir::Down).into_vec(0, col))
        .collect_vec();

    let left_beams = tiles
        .iter()
        .skip(last_col)
        .step_by(width)
        .enumerate()
        .map(|(row, t)| t.interact(Dir::Left).into_vec(row, last_col))
        .collect_vec();

    let beam_sets = [up_beams, right_beams, down_beams, left_beams].concat();

    beam_sets
        .into_par_iter()
        .map(|beams| get_energy_value(beams, tiles.clone(), width, height))
        .max()
        .unwrap()
        .to_string()
}

fn parse(input: &str) -> (Vec<Tile>, usize, usize) {
    let input = input.trim();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let tiles = input
        .lines()
        .flat_map(|line| line.chars())
        .map_into()
        .collect_vec();

    (tiles, width, height)
}

fn get_energy_value(
    mut beams: Vec<(usize, usize, Dir)>,
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
) -> usize {
    let mut energized = HashSet::new();

    while let Some((row, col, dir)) = beams.pop() {
        let next_tile = match dir {
            Dir::Up => tiles
                .iter()
                .enumerate()
                .skip(col)
                .step_by(width)
                .take(row)
                .rev()
                .find(|(_, t)| **t != Tile::Empty),
            Dir::Right => tiles
                .iter()
                .enumerate()
                .skip(row * width + col)
                .take(width - col)
                .skip(1)
                .find(|(_, t)| **t != Tile::Empty),
            Dir::Down => tiles
                .iter()
                .enumerate()
                .skip(row * width + col)
                .step_by(width)
                .take(height - row)
                .skip(1)
                .find(|(_, t)| **t != Tile::Empty),
            Dir::Left => tiles
                .iter()
                .enumerate()
                .skip(row * width)
                .take(col)
                .rev()
                .find(|(_, t)| **t != Tile::Empty),
        };

        if let Some((ix, tile)) = next_tile {
            let next_row = ix / width;
            let next_col = ix % width;

            if !energized.contains(&(row, col, dir)) {
                match dir {
                    Dir::Up => (next_row..=row).skip(1).for_each(|i| {
                        energized.insert((i, col, dir));
                    }),
                    Dir::Right => (col..next_col).for_each(|i| {
                        energized.insert((row, i, dir));
                    }),
                    Dir::Down => (row..next_row).for_each(|i| {
                        energized.insert((i, col, dir));
                    }),
                    Dir::Left => (next_col..=col).skip(1).for_each(|i| {
                        energized.insert((row, i, dir));
                    }),
                };

                match tile.interact(dir) {
                    MaybePair::Single(d) => beams.push((next_row, next_col, d)),
                    MaybePair::Pair((a, b)) => {
                        beams.push((next_row, next_col, a));
                        beams.push((next_row, next_col, b));
                    }
                }
            }
        } else {
            match dir {
                Dir::Up => (0..=row).for_each(|i| {
                    energized.insert((i, col, dir));
                }),
                Dir::Right => (col..width).for_each(|i| {
                    energized.insert((row, i, dir));
                }),
                Dir::Down => (row..height).for_each(|i| {
                    energized.insert((i, col, dir));
                }),
                Dir::Left => (0..=col).for_each(|i| {
                    energized.insert((row, i, dir));
                }),
            };
        }
    }

    energized
        .into_iter()
        .unique_by(|(row, col, _)| (*row, *col))
        .count()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

enum MaybePair<T> {
    Single(T),
    Pair((T, T)),
}

impl<Dir> MaybePair<Dir> {
    fn into_vec(self, row: usize, col: usize) -> Vec<(usize, usize, Dir)> {
        match self {
            MaybePair::Single(d) => vec![(row, col, d)],
            MaybePair::Pair((a, b)) => vec![(row, col, a), (row, col, b)],
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Tile {
    Empty,
    MirrorUp,
    MirrorDown,
    SplitHorz,
    SplitVert,
}

impl Tile {
    fn interact(&self, dir: Dir) -> MaybePair<Dir> {
        match self {
            Tile::Empty => MaybePair::Single(dir),
            Tile::MirrorUp => match dir {
                Dir::Up => MaybePair::Single(Dir::Right),
                Dir::Right => MaybePair::Single(Dir::Up),
                Dir::Down => MaybePair::Single(Dir::Left),
                Dir::Left => MaybePair::Single(Dir::Down),
            },
            Tile::MirrorDown => match dir {
                Dir::Up => MaybePair::Single(Dir::Left),
                Dir::Right => MaybePair::Single(Dir::Down),
                Dir::Down => MaybePair::Single(Dir::Right),
                Dir::Left => MaybePair::Single(Dir::Up),
            },
            Tile::SplitHorz => match dir {
                Dir::Up | Dir::Down => MaybePair::Pair((Dir::Left, Dir::Right)),
                Dir::Left | Dir::Right => MaybePair::Single(dir),
            },
            Tile::SplitVert => match dir {
                Dir::Up | Dir::Down => MaybePair::Single(dir),
                Dir::Left | Dir::Right => MaybePair::Pair((Dir::Up, Dir::Down)),
            },
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '/' => Tile::MirrorUp,
            '\\' => Tile::MirrorDown,
            '-' => Tile::SplitHorz,
            '|' => Tile::SplitVert,
            x => panic!("unknown tile: {}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[rstest]
    #[case(TEST_CASE, "46")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "51")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("6361")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("6701")]
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
