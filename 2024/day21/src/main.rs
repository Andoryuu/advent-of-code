#![feature(test)]

use std::{fs, hash::Hash, sync::LazyLock};

use cached::proc_macro::cached;
use fxhash::FxHashMap;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day21/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    type_passwords(input, 2).to_string()
}

fn process_part_2(input: &str) -> String {
    type_passwords(input, 25).to_string()
}

static NUM_POSITIONS: LazyLock<FxHashMap<isize, (isize, isize)>> = LazyLock::new(|| {
    FxHashMap::from_iter([
        (0, (3, 1)),
        (1, (2, 0)),
        (2, (2, 1)),
        (3, (2, 2)),
        (4, (1, 0)),
        (5, (1, 1)),
        (6, (1, 2)),
        (7, (0, 0)),
        (8, (0, 1)),
        (9, (0, 2)),
        (10, (3, 2)),
    ])
});

static DIR_POSITIONS: LazyLock<FxHashMap<DirectionalSymbol, (isize, isize)>> =
    LazyLock::new(|| {
        FxHashMap::from_iter([
            (DirectionalSymbol::Up, (0, 1)),
            (DirectionalSymbol::Down, (1, 1)),
            (DirectionalSymbol::Left, (1, 0)),
            (DirectionalSymbol::Right, (1, 2)),
            (DirectionalSymbol::A, (0, 2)),
        ])
    });

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum DirectionalSymbol {
    Up,
    Down,
    Left,
    Right,
    A,
}

fn parse(input: &str) -> Vec<(Vec<isize>, usize)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            (
                line.chars()
                    .map(|c| c.to_digit(10).unwrap_or(10) as isize)
                    .collect_vec(),
                line[..3].parse().unwrap(),
            )
        })
        .collect_vec()
}

fn type_passwords(input: &str, indirections: usize) -> usize {
    parse(input)
        .into_iter()
        .map(|(v, n)| {
            n * numbers_into_directions(v)
                .into_iter()
                .map(|d| reduce_directions(d, indirections))
                .sum::<usize>()
        })
        .sum()
}

#[cached(
    key = "(String, usize)",
    convert = r#"{(format!("{directions:?}"), nest)}"#
)]
fn reduce_directions(directions: Vec<Vec<DirectionalSymbol>>, nest: usize) -> usize {
    if nest == 0 {
        directions.into_iter().map(|d| d.len()).min().unwrap()
    } else {
        directions
            .into_iter()
            .map(|d| {
                directions_into_directions(d)
                    .into_iter()
                    .map(|nd| reduce_directions(nd, nest - 1))
                    .sum()
            })
            .min()
            .unwrap()
    }
}

fn next_numeric(current: isize, target: isize) -> Vec<Vec<DirectionalSymbol>> {
    next(&NUM_POSITIONS, current, target, (3, 0))
}

fn next_direction(
    current: DirectionalSymbol,
    target: DirectionalSymbol,
) -> Vec<Vec<DirectionalSymbol>> {
    next(&DIR_POSITIONS, current, target, (0, 0))
}

fn next<K: Hash + Eq>(
    map: &FxHashMap<K, (isize, isize)>,
    current: K,
    target: K,
    missing: (isize, isize),
) -> Vec<Vec<DirectionalSymbol>> {
    let curr_pos = map.get(&current).unwrap();
    let tar_pos = map.get(&target).unwrap();

    let horz_count = tar_pos.1.abs_diff(curr_pos.1);
    let horz_dir = (tar_pos.1 - curr_pos.1).signum();
    let horz = vec![
        if horz_dir > 0 {
            DirectionalSymbol::Right
        } else {
            DirectionalSymbol::Left
        };
        horz_count
    ];

    let vert_count = tar_pos.0.abs_diff(curr_pos.0);
    let vert_dir = (tar_pos.0 - curr_pos.0).signum();
    let vert = vec![
        if vert_dir > 0 {
            DirectionalSymbol::Down
        } else {
            DirectionalSymbol::Up
        };
        vert_count
    ];

    if horz.is_empty() {
        let mut vert = vert;
        vert.push(DirectionalSymbol::A);
        return vec![vert];
    }

    if vert.is_empty() {
        let mut horz = horz;
        horz.push(DirectionalSymbol::A);
        return vec![horz];
    }

    let mut res = vec![];
    if curr_pos.1 != missing.1 || tar_pos.0 != missing.0 {
        let mut path = [vert.clone(), horz.clone()].concat();
        path.push(DirectionalSymbol::A);
        res.push(path);
    }
    if curr_pos.0 != missing.0 || tar_pos.1 != missing.1 {
        let mut path = [horz.clone(), vert.clone()].concat();
        path.push(DirectionalSymbol::A);
        res.push(path);
    }

    res
}

fn numbers_into_directions(values: Vec<isize>) -> Vec<Vec<Vec<DirectionalSymbol>>> {
    let mut current = 10;
    let mut res = vec![];
    for value in values {
        res.push(next_numeric(current, value));
        current = value;
    }
    res
}

fn directions_into_directions(values: Vec<DirectionalSymbol>) -> Vec<Vec<Vec<DirectionalSymbol>>> {
    let mut current = DirectionalSymbol::A;
    let mut res = vec![];
    for value in values {
        res.push(next_direction(current, value));
        current = value;
    }
    res
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
029A
980A
179A
456A
379A";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "126384")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "154115708116294")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("155252")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("195664513288128")]
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
