use std::{collections::BTreeMap, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let mut map = parse(input);
    let mut sand_count = 0u32;

    while let PlacementResult::Placed = try_place_sand(&mut map, 500usize, 0usize) {
        sand_count += 1;
    }

    sand_count.to_string()
}

fn process_data_adv(input: String) -> String {
    let mut map = parse(input);
    let mut sand_count = 0u32;
    let max = map.values().map(|col| col.len()).max().unwrap();

    while let PlacementResult::Placed =
        try_place_sand_with_floor(&mut map, 500usize, 0usize, max + 2)
    {
        sand_count += 1;
    }

    sand_count.to_string()
}

fn try_place_sand(map: &mut BTreeMap<usize, Vec<Tile>>, col: usize, row: usize) -> PlacementResult {
    if !map.contains_key(&col) {
        return PlacementResult::Void;
    }

    let block_ix = {
        map.get(&col)
            .unwrap()
            .iter()
            .enumerate()
            .skip(row)
            .find_position(|(_, t)| **t != Tile::Air)
            .map(|(_, (ix, _))| ix)
    };

    if let Some(block_ix) = block_ix {
        if block_ix == row {
            return PlacementResult::Blocked;
        }

        match try_place_sand(map, col - 1, block_ix) {
            PlacementResult::Blocked => match try_place_sand(map, col + 1, block_ix) {
                PlacementResult::Blocked => {
                    let this_col = map.get_mut(&col).unwrap();
                    this_col[block_ix - 1] = Tile::Sand;
                    PlacementResult::Placed
                }
                r => r,
            },
            r => r,
        }
    } else {
        PlacementResult::Void
    }
}

fn try_place_sand_with_floor(
    map: &mut BTreeMap<usize, Vec<Tile>>,
    col: usize,
    row: usize,
    max: usize,
) -> PlacementResult {
    {
        let this_col = map.entry(col).or_insert_with(|| vec![Tile::Air; max]);

        if this_col.len() < max {
            this_col.resize(max, Tile::Air)
        }
    }

    let block_ix = {
        map.get(&col)
            .unwrap()
            .iter()
            .enumerate()
            .skip(row)
            .find(|(_, t)| **t != Tile::Air)
            .map(|(ix, _)| ix)
    }
    .unwrap_or(max - 1);

    if block_ix == row {
        return PlacementResult::Blocked;
    }

    match try_place_sand_with_floor(map, col - 1, block_ix, max) {
        PlacementResult::Blocked => match try_place_sand_with_floor(map, col + 1, block_ix, max) {
            PlacementResult::Blocked => {
                let this_col = map.get_mut(&col).unwrap();
                this_col[block_ix - 1] = Tile::Sand;
                PlacementResult::Placed
            }
            r => r,
        },
        r => r,
    }
}

fn parse(input: String) -> BTreeMap<usize, Vec<Tile>> {
    let mut map = BTreeMap::new();

    for (rock_x, rock_y) in input.lines().flat_map(parse_line) {
        let col = map.entry(rock_x).or_insert_with(Vec::new);

        if col.len() <= rock_y {
            col.resize(rock_y + 1, Tile::Air)
        }

        col[rock_y] = Tile::Rock;
    }

    map
}

fn parse_line(line: &str) -> Vec<(usize, usize)> {
    line.split(" -> ")
        .map(|coord| {
            coord
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .next_tuple()
                .unwrap()
        })
        .tuple_windows()
        .flat_map(|((a_x, a_y), (b_x, b_y))| {
            if a_x == b_x {
                if a_y < b_y { a_y..=b_y } else { b_y..=a_y }
                    .map(|y| (a_x, y))
                    .collect_vec()
            } else {
                if a_x < b_x { a_x..=b_x } else { b_x..=a_x }
                    .map(|x| (x, a_y))
                    .collect_vec()
            }
        })
        .collect_vec()
}

enum PlacementResult {
    Placed,
    Blocked,
    Void,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Tile {
    Air,
    Rock,
    Sand,
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[rstest]
    #[case(TEST_CASE, "24")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned()));
    }

    #[rstest]
    #[case(TEST_CASE, "93")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned()));
    }
}
