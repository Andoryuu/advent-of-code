#![feature(test)]

use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;
use num::Integer;

fn main() {
    let input = fs::read_to_string("./day12/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    calculate_price(input, collect_field).to_string()
}

fn process_part_2(input: &str) -> String {
    calculate_price(input, collect_field_bulk).to_string()
}

fn calculate_price<F>(input: &str, measure_fn: F) -> usize
where
    F: Fn(
        (isize, isize),
        char,
        &HashMap<(isize, isize), char>,
        &mut HashSet<(isize, isize)>,
    ) -> (usize, usize),
{
    let field = parse(input);
    let mut visited = HashSet::new();

    field
        .iter()
        .filter_map(|(&coord, &plant)| {
            if visited.contains(&coord) {
                None
            } else {
                Some(measure_fn(coord, plant, &field, &mut visited))
            }
        })
        .map(|(area, perimeter)| area * perimeter)
        .sum()
}

fn parse(input: &str) -> HashMap<(isize, isize), char> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .map(move |(col, ch)| ((row as isize, col as isize), ch))
        })
        .collect()
}

fn collect_field(
    coord: (isize, isize),
    plant: char,
    field: &HashMap<(isize, isize), char>,
    visited: &mut HashSet<(isize, isize)>,
) -> (usize, usize) {
    let mut queue = vec![coord];
    visited.insert(coord);

    let mut area = 1;
    let mut perimeter = 0;

    while let Some((row, col)) = queue.pop() {
        for neigh in [
            (row + 1, col),
            (row - 1, col),
            (row, col + 1),
            (row, col - 1),
        ] {
            if field.get(&neigh).is_some_and(|p| &plant == p) {
                if visited.contains(&neigh) {
                    continue;
                }

                visited.insert(neigh);
                area.inc();
                queue.push(neigh);
            } else {
                perimeter.inc();
            }
        }
    }

    (area, perimeter)
}

#[derive(Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn collect_field_bulk(
    coord: (isize, isize),
    plant: char,
    field: &HashMap<(isize, isize), char>,
    visited: &mut HashSet<(isize, isize)>,
) -> (usize, usize) {
    let mut queue = vec![coord];
    visited.insert(coord);

    let mut area = 1;
    let mut sides: HashMap<_, Vec<_>> = HashMap::new();

    while let Some((row, col)) = queue.pop() {
        for (key, val, neigh) in [
            ((Direction::Down, row + 1), col, (row + 1, col)),
            ((Direction::Up, row - 1), col, (row - 1, col)),
            ((Direction::Right, col + 1), row, (row, col + 1)),
            ((Direction::Left, col - 1), row, (row, col - 1)),
        ] {
            if field.get(&neigh).is_some_and(|p| &plant == p) {
                if visited.contains(&neigh) {
                    continue;
                }

                visited.insert(neigh);
                area.inc();
                queue.push(neigh);
            } else {
                sides
                    .entry(key)
                    .and_modify(|v| v.push(val))
                    .or_insert_with(|| vec![val]);
            }
        }
    }

    (
        area,
        sides
            .into_values()
            .map(|v| {
                v.into_iter()
                    .sorted()
                    .tuple_windows()
                    .filter(|(a, b)| (b - a) > 1)
                    .count()
                    + 1
            })
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
AAAA
BBCD
BBCC
EEEC";

    const TEST_CASE_2: &str = "
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const TEST_CASE_3: &str = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const TEST_CASE_4: &str = "
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const TEST_CASE_5: &str = "
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "140")]
    #[case(TEST_CASE_2, "772")]
    #[case(TEST_CASE_3, "1930")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "80")]
    #[case(TEST_CASE_2, "436")]
    #[case(TEST_CASE_3, "1206")]
    #[case(TEST_CASE_4, "236")]
    #[case(TEST_CASE_5, "368")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("1415378")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("862714")]
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
