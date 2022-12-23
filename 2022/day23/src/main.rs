use std::{
    collections::HashSet,
    fs,
};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let mut elves = parse(input);
    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for _ in 0..10 {
        let mut has_prop = false;
        let propositions = elves
            .iter()
            .map(|elf| (elf.get_proposition(&directions, &elves), elf))
            .sorted_by_key(|(prop, _)| prop.clone())
            .group_by(|(prop, _)| prop.clone())
            .into_iter()
            .flat_map(|(key, group)| {
                let evs = group.map(|(_, e)| e.clone()).collect_vec();
                if let Some(key) = key {
                    if evs.len() > 1 {
                        evs
                    } else {
                        has_prop = true;
                        vec![key]
                    }
                } else {
                    evs
                }
            })
            .collect_vec();

        elves = HashSet::<_>::from_iter(propositions);
        directions.rotate_left(1);

        if !has_prop {
            break;
        }
    }

    let min_row = elves.iter().map(|e| e.row).min().unwrap();
    let max_row = elves.iter().map(|e| e.row).max().unwrap();
    let min_col = elves.iter().map(|e| e.col).min().unwrap();
    let max_col = elves.iter().map(|e| e.col).max().unwrap();
    ((max_row.abs_diff(min_row) + 1) * (max_col.abs_diff(min_col) + 1) - elves.len()).to_string()
}

fn process_data_adv(input: String) -> String {
    let mut elves = parse(input);
    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
    let mut round = 0;

    loop {
        round += 1;

        let mut has_prop = false;
        let propositions = elves
            .iter()
            .map(|elf| (elf.get_proposition(&directions, &elves), elf))
            .sorted_by_key(|(prop, _)| prop.clone())
            .group_by(|(prop, _)| prop.clone())
            .into_iter()
            .flat_map(|(key, group)| {
                let evs = group.map(|(_, e)| e.clone()).collect_vec();
                if let Some(key) = key {
                    if evs.len() > 1 {
                        evs
                    } else {
                        has_prop = true;
                        vec![key]
                    }
                } else {
                    evs
                }
            })
            .collect_vec();

        elves = HashSet::<_>::from_iter(propositions);
        directions.rotate_left(1);

        if !has_prop {
            break;
        }
    }

    round.to_string()
}

fn parse(input: String) -> HashSet<Elf> {
    HashSet::from_iter(input.lines().enumerate().flat_map(|(row, line)| {
        line.char_indices()
            .filter_map(|(col, c)| {
                if c == '#' {
                    Some(Elf {
                        row: row as isize,
                        col: col as isize,
                    })
                } else {
                    None
                }
            })
            .collect_vec()
    }))
}

fn print_elves(elves: &HashSet<Elf>) {
    println!("elves count: {}", elves.len());

    let min_row = elves.iter().map(|e| e.row).min().unwrap();
    let max_row = elves.iter().map(|e| e.row).max().unwrap();
    let min_col = elves.iter().map(|e| e.col).min().unwrap();
    let max_col = elves.iter().map(|e| e.col).max().unwrap();

    for row in min_row..=max_row {
        println!();
        for col in min_col..=max_col {
            if elves.contains(&Elf { row, col }) {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
    println!();
    println!("{:?}", elves.iter().sorted());
    println!();
}

enum Direction {
    North,
    South,
    West,
    East,
}

// we could use simple tuple, but I hate unnamed x-y manipulation
#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Debug)]
struct Elf {
    row: isize,
    col: isize,
}

impl Elf {
    fn has_any_neighbor(&self, others: &HashSet<Elf>) -> bool {
        for row_os in -1..=1 {
            for col_os in -1..=1 {
                if row_os == 0 && col_os == 0 {
                    continue;
                }

                if others.contains(&Elf {
                    row: self.row + row_os,
                    col: self.col + col_os,
                }) {
                    return true;
                }
            }
        }

        false
    }

    fn has_neighbor(&self, dir: &Direction, others: &HashSet<Elf>) -> bool {
        match dir {
            Direction::North => (-1..=1).any(|os| {
                others.contains(&Elf {
                    row: self.row - 1,
                    col: self.col + os,
                })
            }),
            Direction::South => (-1..=1).any(|os| {
                others.contains(&Elf {
                    row: self.row + 1,
                    col: self.col + os,
                })
            }),
            Direction::West => (-1..=1).any(|os| {
                others.contains(&Elf {
                    row: self.row + os,
                    col: self.col - 1,
                })
            }),
            Direction::East => (-1..=1).any(|os| {
                others.contains(&Elf {
                    row: self.row + os,
                    col: self.col + 1,
                })
            }),
        }
    }

    fn get_step(&self, dir: &Direction) -> Elf {
        match dir {
            Direction::North => Elf {
                row: self.row - 1,
                col: self.col,
            },
            Direction::South => Elf {
                row: self.row + 1,
                col: self.col,
            },
            Direction::West => Elf {
                row: self.row,
                col: self.col - 1,
            },
            Direction::East => Elf {
                row: self.row,
                col: self.col + 1,
            },
        }
    }

    fn get_proposition(&self, dirs: &Vec<Direction>, others: &HashSet<Elf>) -> Option<Elf> {
        if self.has_any_neighbor(others) {
            for dir in dirs {
                if !self.has_neighbor(dir, others) {
                    return Some(self.get_step(dir));
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    #[rstest]
    #[case(TEST_CASE, "110")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned()));
    }

    #[rstest]
    #[case(TEST_CASE, "20")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned()));
    }
}
