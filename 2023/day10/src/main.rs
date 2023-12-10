#![feature(test)]

use std::{collections::HashMap, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day10/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    (get_main_pipe(input).len() / 2).to_string()
}

fn process_part_2(input: &str) -> String {
    let rows = input.lines().filter(|l| !l.is_empty()).count() as isize;
    let cols = input.lines().find(|l| !l.is_empty()).unwrap().len() as isize;
    let pipe = get_main_pipe(input);

    let mut tiles = HashMap::<Pos, i32>::new();

    // it would be probably better to count from non-pipe tiles out, but whatever
    for (pos, dir) in pipe {
        tiles
            .entry(pos)
            .and_modify(|c| *c += 100_000)
            .or_insert(100_000);

        for d in dir {
            match d {
                Dir::East => {
                    for r in 0..pos.row {
                        tiles
                            .entry(Pos {
                                row: r,
                                col: pos.col,
                            })
                            .and_modify(|c| *c -= 1)
                            .or_insert(-1);
                    }
                    for r in (pos.row + 1)..rows {
                        tiles
                            .entry(Pos {
                                row: r,
                                col: pos.col,
                            })
                            .and_modify(|c| *c += 1)
                            .or_insert(1);
                    }
                }
                Dir::South => {
                    for c in 0..pos.col {
                        tiles
                            .entry(Pos {
                                row: pos.row,
                                col: c,
                            })
                            .and_modify(|c| *c += 1)
                            .or_insert(1);
                    }
                    for c in (pos.col + 1)..cols {
                        tiles
                            .entry(Pos {
                                row: pos.row,
                                col: c,
                            })
                            .and_modify(|c| *c -= 1)
                            .or_insert(-1);
                    }
                }
                Dir::West => {
                    for r in 0..pos.row {
                        tiles
                            .entry(Pos {
                                row: r,
                                col: pos.col,
                            })
                            .and_modify(|c| *c += 1)
                            .or_insert(1);
                    }
                    for r in (pos.row + 1)..rows {
                        tiles
                            .entry(Pos {
                                row: r,
                                col: pos.col,
                            })
                            .and_modify(|c| *c -= 1)
                            .or_insert(-1);
                    }
                }
                Dir::North => {
                    for c in 0..pos.col {
                        tiles
                            .entry(Pos {
                                row: pos.row,
                                col: c,
                            })
                            .and_modify(|c| *c -= 1)
                            .or_insert(-1);
                    }
                    for c in (pos.col + 1)..cols {
                        tiles
                            .entry(Pos {
                                row: pos.row,
                                col: c,
                            })
                            .and_modify(|c| *c += 1)
                            .or_insert(1);
                    }
                }
            }
        }
    }

    tiles
        .values()
        .filter(|v| **v == 8 || **v == -8)
        .count()
        .to_string()
}

fn get_main_pipe(input: &str) -> Vec<(Pos, Vec<Dir>)> {
    let tiles: HashMap<Pos, Tile> = input
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(col, c)| {
                    (
                        Pos {
                            row: row as isize,
                            col: col as isize,
                        },
                        match c {
                            '|' => Tile::Pipe([Dir::North, Dir::South]),
                            '-' => Tile::Pipe([Dir::West, Dir::East]),
                            'L' => Tile::Pipe([Dir::North, Dir::East]),
                            'J' => Tile::Pipe([Dir::North, Dir::West]),
                            '7' => Tile::Pipe([Dir::West, Dir::South]),
                            'F' => Tile::Pipe([Dir::East, Dir::South]),
                            'S' => Tile::Start,
                            x => panic!("{:?}", x),
                        },
                    )
                })
                .collect_vec()
        })
        .collect();

    let start = tiles
        .iter()
        .find(|(_, tile)| Tile::Start == **tile)
        .unwrap();

    let (mut dir, mut pos) = get_start_conn(&tiles, start.0);
    let start_dir = dir;

    let mut main_pipe = Vec::<(Pos, Vec<Dir>)>::new();

    loop {
        if let Tile::Pipe(v) = pos.1 {
            let (r, c, new_dir) = match v.iter().find(|d| dir != **d).unwrap() {
                Dir::East => (0, 1, Dir::West),
                Dir::South => (1, 0, Dir::North),
                Dir::West => (0, -1, Dir::East),
                Dir::North => (-1, 0, Dir::South),
            };

            main_pipe.push((pos.0, vec![dir.opposite(), new_dir.opposite()]));

            let next_pos = Pos {
                row: pos.0.row + r,
                col: pos.0.col + c,
            };

            let next_tile = tiles.get(&next_pos).unwrap();

            pos = (next_pos, next_tile);
            dir = new_dir;
        } else {
            main_pipe.push((*start.0, vec![start_dir.opposite(), dir.opposite()]));
            return main_pipe;
        }
    }
}

fn get_start_conn<'a>(tiles: &'a HashMap<Pos, Tile>, start: &Pos) -> (Dir, (Pos, &'a Tile)) {
    let east = Pos {
        row: start.row,
        col: start.col + 1,
    };
    if let Some(tile) = tiles.get(&east) {
        if let Tile::Pipe(v) = tile {
            if v.iter().any(|d| Dir::West == *d) {
                return (Dir::West, (east, tile));
            }
        }
    }

    let south = Pos {
        row: start.row + 1,
        col: start.col,
    };
    if let Some(tile) = tiles.get(&south) {
        if let Tile::Pipe(v) = tile {
            if v.iter().any(|d| Dir::North == *d) {
                return (Dir::North, (south, tile));
            }
        }
    }

    let west = Pos {
        row: start.row,
        col: start.col - 1,
    };
    if let Some(tile) = tiles.get(&west) {
        if let Tile::Pipe(v) = tile {
            if v.iter().any(|d| Dir::East == *d) {
                return (Dir::East, (west, tile));
            }
        }
    }

    let north = Pos {
        row: start.row - 1,
        col: start.col,
    };
    if let Some(tile) = tiles.get(&north) {
        if let Tile::Pipe(v) = tile {
            if v.iter().any(|d| Dir::South == *d) {
                return (Dir::South, (north, tile));
            }
        }
    }

    panic!("start is not connected")
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
struct Pos {
    row: isize,
    col: isize,
}

#[derive(PartialEq, Debug)]
enum Tile {
    Start,
    Pipe([Dir; 2]),
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Dir {
    East,
    South,
    West,
    North,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
            Dir::North => Dir::South,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE_1: &str = "
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const TEST_CASE_2: &str = "
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const TEST_CASE_3: &str = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const TEST_CASE_4: &str = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const TEST_CASE_5: &str = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[rstest]
    #[case(TEST_CASE_1, "4")]
    #[case(TEST_CASE_2, "8")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE_3, "4")]
    #[case(TEST_CASE_4, "8")]
    #[case(TEST_CASE_5, "10")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("6942")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("297")]
    fn part_2_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_2(&input));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE_2));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_1(&input));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE_5));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_2(&input));
    }
}
