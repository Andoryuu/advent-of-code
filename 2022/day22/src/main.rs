use std::{collections::HashMap, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone(), 50);
    let adv_output = process_data_adv(input, false);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String, tile_size: usize) -> String {
    let (board, movement) = parse(input);
    let mut pos = board.start;
    let mut dir = Direction::Right;

    for m in movement {
        match m {
            Movement::Turn(turn) => dir = dir.get_turned(turn),
            Movement::Move(n) => {
                'm: for _ in 0..n {
                    let temp_pos = pos.step(&dir);

                    if let Some(t) = board.map.get(&temp_pos) {
                        if *t {
                            break 'm;
                        } else {
                            pos = temp_pos;
                            continue;
                        }
                    }

                    let mut temp_pos = match dir {
                        Direction::Up => Coordinates {
                            col: temp_pos.col,
                            row: board.map_size.rows,
                        },
                        Direction::Right => Coordinates {
                            col: 1,
                            row: temp_pos.row,
                        },
                        Direction::Down => Coordinates {
                            col: temp_pos.col,
                            row: 1,
                        },
                        Direction::Left => Coordinates {
                            col: board.map_size.cols,
                            row: temp_pos.row,
                        },
                    };

                    loop {
                        if let Some(t) = board.map.get(&temp_pos) {
                            if *t {
                                break 'm;
                            } else {
                                pos = temp_pos;
                                break;
                            }
                        } else {
                            temp_pos = temp_pos.step_by(&dir, tile_size);
                        }
                    }
                }
            }
        }
    }

    (1_000 * pos.row + 4 * pos.col + dir.get_value()).to_string()
}

fn process_data_adv(input: String, is_test: bool) -> String {
    let (board, movement) = parse(input);
    let mut pos = board.start;
    let mut dir = Direction::Right;

    for m in movement {
        match m {
            Movement::Turn(turn) => dir = dir.get_turned(turn),
            Movement::Move(n) => {
                'm: for _ in 0..n {
                    let temp_pos = pos.step(&dir);

                    if let Some(t) = board.map.get(&temp_pos) {
                        if *t {
                            break 'm;
                        } else {
                            pos = temp_pos;
                            continue;
                        }
                    }

                    let (temp_pos, temp_dir) = if is_test {
                        transform_coords_test(temp_pos, &dir)
                    } else {
                        transform_coords(temp_pos, &dir)
                    };

                    if let Some(t) = board.map.get(&temp_pos) {
                        if *t {
                            break 'm;
                        } else {
                            pos = temp_pos;
                            dir = temp_dir;
                        }
                    } else {
                        panic!("invalid transformation");
                    }
                }
            }
        }
    }

    (1_000 * pos.row + 4 * pos.col + dir.get_value()).to_string()
}

fn parse(input: String) -> (Board, Vec<Movement>) {
    let (board_lines, move_line) = input
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, g)| g.collect_vec())
        .next_tuple()
        .unwrap();

    (
        parse_board(board_lines),
        parse_move(move_line.first().unwrap()),
    )
}

fn parse_board(lines: Vec<&str>) -> Board {
    let mut map = HashMap::new();
    let mut start = None;

    for (row, line) in lines.iter().enumerate() {
        let row = row + 1;
        for (col, c) in line.chars().enumerate() {
            let col = col + 1;
            match c {
                '.' => {
                    if start.is_none() {
                        start = Some(Coordinates { row, col });
                    }
                    map.insert(Coordinates { row, col }, false);
                }
                '#' => {
                    if start.is_none() {
                        start = Some(Coordinates { row, col });
                    }
                    map.insert(Coordinates { row, col }, true);
                }
                ' ' => {}
                _ => panic!("unexpected char: {c}"),
            }
        }
    }

    Board {
        start: start.unwrap(),
        map_size: Size {
            cols: map.iter().map(|t| t.0.col).max().unwrap(),
            rows: map.iter().map(|t| t.0.row).max().unwrap(),
        },
        map,
    }
}

fn parse_move(line: &str) -> Vec<Movement> {
    line.chars()
        .group_by(|c| c.is_ascii_digit())
        .into_iter()
        .map(|(is_digit, chars)| {
            if is_digit {
                Movement::Move(chars.collect::<String>().parse::<usize>().unwrap())
            } else {
                Movement::Turn(if chars.into_iter().next().unwrap() == 'R' {
                    Direction::Right
                } else {
                    Direction::Left
                })
            }
        })
        .collect_vec()
}

// hardcode all cube transitions
fn transform_coords(coords: Coordinates, dir: &Direction) -> (Coordinates, Direction) {
    match dir {
        Direction::Up => {
            if coords.row == 0 {
                if coords.col < 101 {
                    (
                        Coordinates {
                            row: coords.col + 100,
                            col: 1,
                        },
                        Direction::Right,
                    )
                } else {
                    (
                        Coordinates {
                            row: 200,
                            col: coords.col - 100,
                        },
                        Direction::Up,
                    )
                }
            } else {
                (
                    Coordinates {
                        row: coords.col + 50,
                        col: 51,
                    },
                    Direction::Right,
                )
            }
        }
        Direction::Right => {
            if coords.col == 101 {
                if coords.row < 101 {
                    (
                        Coordinates {
                            row: 50,
                            col: coords.row + 50,
                        },
                        Direction::Up,
                    )
                } else {
                    (
                        Coordinates {
                            row: 151 - coords.row,
                            col: 150,
                        },
                        Direction::Left,
                    )
                }
            } else if coords.col == 151 {
                (
                    Coordinates {
                        row: 151 - coords.row,
                        col: 100,
                    },
                    Direction::Left,
                )
            } else {
                (
                    Coordinates {
                        row: 150,
                        col: coords.row - 100,
                    },
                    Direction::Up,
                )
            }
        }
        Direction::Down => {
            if coords.row == 51 {
                (
                    Coordinates {
                        row: coords.col - 50,
                        col: 100,
                    },
                    Direction::Left,
                )
            } else if coords.row == 151 {
                (
                    Coordinates {
                        row: coords.col + 100,
                        col: 50,
                    },
                    Direction::Left,
                )
            } else {
                (
                    Coordinates {
                        row: 1,
                        col: coords.col + 100,
                    },
                    Direction::Down,
                )
            }
        }
        Direction::Left => {
            if coords.col == 50 {
                if coords.row < 51 {
                    (
                        Coordinates {
                            row: 151 - coords.row,
                            col: 1,
                        },
                        Direction::Right,
                    )
                } else {
                    (
                        Coordinates {
                            row: 101,
                            col: coords.row - 50,
                        },
                        Direction::Down,
                    )
                }
            } else if coords.row < 151 {
                (
                    Coordinates {
                        row: 151 - coords.row,
                        col: 51,
                    },
                    Direction::Right,
                )
            } else {
                (
                    Coordinates {
                        row: 1,
                        col: coords.row - 100,
                    },
                    Direction::Down,
                )
            }
        }
    }
}

fn transform_coords_test(coords: Coordinates, dir: &Direction) -> (Coordinates, Direction) {
    match dir {
        Direction::Up => {
            if coords.row == 4 {
                if coords.col < 5 {
                    (
                        Coordinates {
                            row: 1,
                            col: 13 - coords.col,
                        },
                        Direction::Down,
                    )
                } else {
                    (
                        Coordinates {
                            row: coords.col - 4,
                            col: 9,
                        },
                        Direction::Right,
                    )
                }
            } else if coords.row == 0 {
                (
                    Coordinates {
                        row: 5,
                        col: 13 - coords.col,
                    },
                    Direction::Down,
                )
            } else {
                (
                    Coordinates {
                        row: 21 - coords.col,
                        col: 12,
                    },
                    Direction::Left,
                )
            }
        }
        Direction::Right => {
            if coords.col == 13 {
                if coords.row < 5 {
                    (
                        Coordinates {
                            row: 13 - coords.row,
                            col: 16,
                        },
                        Direction::Left,
                    )
                } else {
                    (
                        Coordinates {
                            row: 9,
                            col: 21 - coords.row,
                        },
                        Direction::Down,
                    )
                }
            } else {
                (
                    Coordinates {
                        row: 13 - coords.row,
                        col: 12,
                    },
                    Direction::Left,
                )
            }
        }
        Direction::Down => {
            if coords.row == 9 {
                if coords.col < 5 {
                    (
                        Coordinates {
                            row: 12,
                            col: 13 - coords.col,
                        },
                        Direction::Up,
                    )
                } else {
                    (
                        Coordinates {
                            row: 17 - coords.col,
                            col: 9,
                        },
                        Direction::Right,
                    )
                }
            } else if coords.col < 13 {
                (
                    Coordinates {
                        row: 8,
                        col: 13 - coords.col,
                    },
                    Direction::Up,
                )
            } else {
                (
                    Coordinates {
                        row: 21 - coords.row,
                        col: 1,
                    },
                    Direction::Right,
                )
            }
        }
        Direction::Left => {
            if coords.col == 8 {
                if coords.row < 5 {
                    (
                        Coordinates {
                            row: 5,
                            col: coords.row + 4,
                        },
                        Direction::Down,
                    )
                } else {
                    (
                        Coordinates {
                            row: 8,
                            col: 17 - coords.row,
                        },
                        Direction::Up,
                    )
                }
            } else {
                (
                    Coordinates {
                        row: 12,
                        col: 21 - coords.row,
                    },
                    Direction::Up,
                )
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordinates {
    row: usize,
    col: usize,
}

impl Coordinates {
    fn step(&self, dir: &Direction) -> Coordinates {
        match dir {
            Direction::Up => Coordinates {
                col: self.col,
                row: self.row - 1,
            },
            Direction::Right => Coordinates {
                col: self.col + 1,
                row: self.row,
            },
            Direction::Down => Coordinates {
                col: self.col,
                row: self.row + 1,
            },
            Direction::Left => Coordinates {
                col: self.col - 1,
                row: self.row,
            },
        }
    }

    fn step_by(&self, dir: &Direction, amount: usize) -> Coordinates {
        match dir {
            Direction::Up => Coordinates {
                col: self.col,
                row: self.row - amount,
            },
            Direction::Right => Coordinates {
                col: self.col + amount,
                row: self.row,
            },
            Direction::Down => Coordinates {
                col: self.col,
                row: self.row + amount,
            },
            Direction::Left => Coordinates {
                col: self.col - amount,
                row: self.row,
            },
        }
    }
}

#[derive(Debug)]
struct Size {
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
struct Board {
    start: Coordinates,
    map_size: Size,
    map: HashMap<Coordinates, bool>,
}

enum Movement {
    Move(usize),
    Turn(Direction),
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn get_value(&self) -> usize {
        match self {
            Direction::Up => 3,
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
        }
    }

    fn get_turned(&self, dir: Direction) -> Direction {
        match dir {
            Direction::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            Direction::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            },
            _ => panic!("invalid turn direction: {dir:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    #[rstest]
    #[case(TEST_CASE, "6032")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned(), 4));
    }

    #[rstest]
    #[case(TEST_CASE, "5031")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned(), true));
    }
}
