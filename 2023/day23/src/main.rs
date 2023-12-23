#![feature(test)]

use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let input = fs::read_to_string("./day23/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let field = input
        .trim()
        .lines()
        .map(|line| line.chars().map_into().collect_vec())
        .collect_vec();

    let mut graph = HashMap::new();
    fill_slippery_graph(PRESTART_POINT, START_POINT, &mut graph, &field);

    get_longest(graph).to_string()
}

fn process_part_2(input: &str) -> String {
    let field = input
        .trim()
        .lines()
        .map(|line| line.chars().map_into().collect_vec())
        .collect_vec();

    let mut graph = HashMap::new();
    fill_dry_graph(PRESTART_POINT, START_POINT, &mut graph, &field);

    get_longest(graph).to_string()
}

const START_POINT: (isize, isize) = (0, 1);
const PRESTART_POINT: (isize, isize) = (-1, 1);
const START_ID: isize = get_id(PRESTART_POINT);
const END_ID: isize = 999_999_999_999;

fn fill_slippery_graph(
    mut prev_pos: (isize, isize),
    (mut row, mut col): (isize, isize),
    graph: &mut HashMap<isize, HashSet<(usize, isize)>>,
    field: &[Vec<Tile>],
) {
    let start = get_id(prev_pos);

    graph.entry(start).or_default();

    let mut len = 0;

    loop {
        len += 1;

        let down = (row + 1, col);

        if get_from_field(down, field).is_none() {
            graph.entry(start).and_modify(|v| {
                v.insert((len, END_ID));
            });
            return;
        }

        let up = ((row - 1, col), Tile::SlopeUp);
        let right = ((row, col + 1), Tile::SlopeRight);
        let down = (down, Tile::SlopeDown);
        let left = ((row, col - 1), Tile::SlopeLeft);

        let valid_conts = [up, right, down, left]
            .into_iter()
            .filter_map(|(pos, slope)| {
                if pos == prev_pos {
                    None
                } else {
                    let tile = get_from_field(pos, field).unwrap();

                    if tile == Tile::Path || tile == slope {
                        Some(pos)
                    } else {
                        None
                    }
                }
            })
            .collect_vec();

        prev_pos = (row, col);

        if valid_conts.len() == 1 {
            (row, col) = *valid_conts.first().unwrap();
            continue;
        }

        let end = get_id(prev_pos);

        graph.entry(start).and_modify(|v| {
            v.insert((len, end));
        });

        if graph.contains_key(&end) {
            return;
        }

        for cont in valid_conts {
            fill_slippery_graph(prev_pos, cont, graph, field);
        }

        return;
    }
}

fn fill_dry_graph(
    mut prev_pos: (isize, isize),
    (mut row, mut col): (isize, isize),
    graph: &mut HashMap<isize, HashSet<(usize, isize)>>,
    field: &[Vec<Tile>],
) {
    let start = get_id(prev_pos);

    graph.entry(start).or_default();

    let mut len = 0;

    loop {
        len += 1;

        let down = (row + 1, col);

        if get_from_field(down, field).is_none() {
            graph.entry(start).and_modify(|v| {
                v.insert((len, END_ID));
            });
            return;
        }

        let up = (row - 1, col);
        let right = (row, col + 1);
        let left = (row, col - 1);

        let valid_conts = [up, right, down, left]
            .into_iter()
            .filter(|pos| *pos != prev_pos && get_from_field(*pos, field).unwrap() != Tile::Forest)
            .collect_vec();

        prev_pos = (row, col);

        if valid_conts.len() == 1 {
            (row, col) = *valid_conts.first().unwrap();
            continue;
        }

        let end = get_id(prev_pos);
        let end_processed = graph.contains_key(&end);

        graph.entry(start).and_modify(|v| {
            v.insert((len, end));
        });

        let end_set = graph.entry(end).or_default();
        end_set.insert((len, start));

        if end_processed {
            return;
        }

        for cont in valid_conts {
            fill_dry_graph(prev_pos, cont, graph, field);
        }

        return;
    }
}

const fn get_id((row, col): (isize, isize)) -> isize {
    row * 100_000 + col
}

fn get_from_field((row, col): (isize, isize), field: &[Vec<Tile>]) -> Option<Tile> {
    if row < 0 || col < 0 {
        None
    } else {
        field
            .get(row as usize)
            .and_then(|r| r.get(col as usize))
            .copied()
    }
}

fn get_longest(graph: HashMap<isize, HashSet<(usize, isize)>>) -> usize {
    get_path_from_graph(START_ID, HashSet::new(), &graph).unwrap() - 1
}

fn get_path_from_graph(
    current: isize,
    mut visited: HashSet<isize>,
    graph: &HashMap<isize, HashSet<(usize, isize)>>,
) -> Option<usize> {
    visited.insert(current);

    let mut targets = Vec::new();
    let mut res = Vec::new();

    for (len, target) in graph.get(&current).unwrap() {
        if *target == END_ID {
            res.push(*len);
        } else if !visited.contains(target) {
            targets.push((*len, *target));
        }
    }

    if !targets.is_empty() {
        if targets.len() > 1 {
            if let Some(max) = targets
                .into_par_iter()
                .filter_map(|(len, target)| {
                    get_path_from_graph(target, visited.clone(), graph).map(|l| l + len)
                })
                .max()
            {
                res.push(max);
            }
        } else {
            let (len, target) = targets.first().unwrap();
            if let Some(max) = get_path_from_graph(*target, visited, graph) {
                res.push(max + len);
            }
        }
    }

    if res.is_empty() {
        None
    } else {
        res.into_iter().max()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Path,
    Forest,
    SlopeUp,
    SlopeRight,
    SlopeDown,
    SlopeLeft,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '^' => Tile::SlopeUp,
            '>' => Tile::SlopeRight,
            'v' => Tile::SlopeDown,
            '<' => Tile::SlopeLeft,
            x => panic!("unknown tile: {x}"),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[rstest]
    #[case(TEST_CASE, "94")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "154")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("2190")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("6258")]
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
