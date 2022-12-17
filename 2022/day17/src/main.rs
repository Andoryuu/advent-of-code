use std::fs;

use itertools::Itertools;
use num::Integer;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    const TARGET: usize = 2022;

    let vents_base = parse(input);
    let shapes_base = get_shapes();

    let mut vents = vents_base.iter().cycle();
    let mut shapes = shapes_base.iter().cycle().take(TARGET);
    let mut top = 0usize;
    let mut chamber = Chamber::new();
    let mut rock = shapes.next().map(|r| generate_rock(r, &top)).unwrap();

    loop {
        if step(&mut chamber, &mut top, &mut rock, vents.next().unwrap()) {
            if let Some(next_shape) = shapes.next() {
                rock = generate_rock(next_shape, &top);
                chamber.fit_rock(rock.iter().map(|(_, y)| y).max().unwrap());
            } else {
                break;
            }
        }
    }

    top.to_string()
}

fn process_data_adv(input: String) -> String {
    const TARGET: usize = 1_000_000_000_000;

    let vents_base = parse(input);
    let shapes_base = get_shapes();

    let common_multiple = vents_base.len() * shapes_base.len();
    let mut vents_counter = 0usize;
    let mut rocks_counter = 0usize;
    let mut fst_top = 0usize;
    let mut fst_rock = 0usize;
    let mut skipped_top = 0usize;
    let mut remaining_rocks = 0usize;

    let mut vents = vents_base.iter().cycle();
    let mut shapes = shapes_base.iter().cycle();
    let mut top = 0usize;
    let mut chamber = Chamber::new();
    let mut rock = shapes.next().map(|r| generate_rock(r, &top)).unwrap();

    loop {
        vents_counter += 1;

        if step(&mut chamber, &mut top, &mut rock, vents.next().unwrap()) {
            if let Some(next_shape) = shapes.next() {
                rock = generate_rock(next_shape, &top);
                chamber.fit_rock(rock.iter().map(|(_, y)| y).max().unwrap());
                rocks_counter += 1;
            } else {
                break;
            }
        }

        if vents_counter.is_multiple_of(&common_multiple) {
            if fst_top == 0 {
                fst_top = top;
                fst_rock = rocks_counter;
            } else {
                skipped_top =
                    ((TARGET - fst_rock) / (rocks_counter - fst_rock) - 1) * (top - fst_top);
                remaining_rocks = (TARGET - fst_rock) % (rocks_counter - fst_rock);

                break;
            }
        }
    }

    let mut shapes = shapes.take(remaining_rocks);

    loop {
        if step(&mut chamber, &mut top, &mut rock, vents.next().unwrap()) {
            if let Some(next_shape) = shapes.next() {
                rock = generate_rock(next_shape, &top);
                chamber.fit_rock(rock.iter().map(|(_, y)| y).max().unwrap());
            } else {
                break;
            }
        }
    }

    (top + skipped_top - 1).to_string()
}

fn step(
    chamber: &mut Chamber,
    top: &mut usize,
    rock: &mut Vec<(usize, usize)>,
    vent: &Push,
) -> bool {
    let mut side_rock = match vent {
        Push::Left => {
            if rock.iter().any(|(x, _)| *x == 0) {
                rock.clone()
            } else {
                rock.iter().map(|(x, y)| (x - 1, *y)).collect_vec()
            }
        }
        Push::Right => {
            if rock.iter().any(|(x, _)| *x == 6) {
                rock.clone()
            } else {
                rock.iter().map(|(x, y)| (x + 1, *y)).collect_vec()
            }
        }
    };

    if side_rock.iter().any(|(x, y)| chamber.is_rock(x, y)) {
        side_rock = rock.clone();
    }

    let down_rock = side_rock.iter().map(|(x, y)| (*x, y - 1)).collect_vec();

    if down_rock
        .iter()
        .any(|(x, y)| *y == 0 || chamber.is_rock(x, y))
    {
        chamber.set_rock(&side_rock);
        *top = (*top).max(*side_rock.iter().map(|(_, y)| y).max().unwrap());

        return true;
    } else {
        *rock = down_rock;
    }

    false
}

fn generate_rock(shape: &Shape, top: &usize) -> Vec<(usize, usize)> {
    shape.parts
        .iter()
        .map(|(x, y)| (x + 2, y + top + 4))
        .collect_vec()
}

fn get_shapes() -> Vec<Shape> {
    vec![
        Shape {
            parts: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        },
        Shape {
            parts: vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        },
        Shape {
            parts: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        },
        Shape {
            parts: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        },
        Shape {
            parts: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        },
    ]
}

fn parse(input: String) -> Vec<Push> {
    input.trim().chars().map(Push::from).collect_vec()
}

enum Push {
    Left,
    Right,
}

impl From<char> for Push {
    fn from(value: char) -> Self {
        match value {
            '<' => Push::Left,
            '>' => Push::Right,
            _ => panic!("unexpected char: {}", value),
        }
    }
}

struct Shape {
    parts: Vec<(usize, usize)>,
}

struct Chamber {
    internal: Vec<Vec<bool>>,
}

impl Chamber {
    fn new() -> Self {
        Chamber {
            internal: vec![vec![false; 7]; 2_000],
        }
    }

    fn is_rock(&self, x: &usize, y: &usize) -> bool {
        *self.internal.get(*y).and_then(|r| r.get(*x)).unwrap()
    }

    fn fit_rock(&mut self, y: &usize) {
        if self.internal.len() <= *y {
            self.internal.resize_with(y + 1, || vec![false; 7]);
        }
    }

    fn set_rock(&mut self, rock: &Vec<(usize, usize)>) {
        for (x, y) in rock {
            *self
                .internal
                .get_mut(*y)
                .and_then(|r| r.get_mut(*x))
                .unwrap() = true;
        }
    }

    fn print_to(&self, top: &usize) {
        println!("-------");
        for line in self.internal.iter().take(top + 1).rev() {
            println!(
                "{}",
                line.iter()
                    .map(|b| if *b { '#' } else { '.' })
                    .collect::<String>()
            );
        }
        println!("-------");
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";

    #[rstest]
    #[case(TEST_CASE, "3068")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned()));
    }

    #[rstest]
    #[case(TEST_CASE, "1514285714288")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned()));
    }
}
