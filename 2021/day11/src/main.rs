use std::{collections::BTreeSet, fs};

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let mut grid = parse(input);

    (0..SIZE)
        .map(|_| step(&mut grid))
        .sum::<usize>()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    let mut grid = parse(input);
    let mut step_count = 0u32;

    loop {
        step_count += 1;

        if step(&mut grid) == SIZE {
            return step_count.to_string();
        }
    }
}

const DIMENSION: usize = 10;
const SIZE: usize = DIMENSION * DIMENSION;

fn parse(input: String) -> Vec<u32> {
    input
        .trim()
        .lines()
        .map(|l| l.trim())
        .flat_map(|l| l.chars())
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn step(grid: &mut Vec<u32>) -> usize {
    for v in grid.iter_mut() {
        if *v > 9 {
            *v = 0;
        }

        *v += 1
    }

    let mut flashed = BTreeSet::new();
    let mut to_do: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if *v > 9 {
                flashed.insert(i);
                Some(i)
            } else {
                None
            }
        })
        .collect();

    while let Some(next) = to_do.pop() {
        let neighs = get_neighbor_positions(next);

        for &n in neighs.iter() {
            grid[n] += 1;

            if grid[n] > 9 && !flashed.contains(&n) {
                to_do.push(n);
                flashed.insert(n);
            }
        }
    }

    flashed.len()
}

fn get_neighbor_positions(position: usize) -> Vec<usize> {
    let mut neighs = Vec::with_capacity(8);
    let is_not_left = position % DIMENSION != 0;
    let is_not_right = position % DIMENSION != (DIMENSION - 1);
    let is_not_top = position >= DIMENSION;
    let is_not_bottom = position < SIZE - DIMENSION;

    if is_not_top {
        let above = position - DIMENSION;

        neighs.push(above);

        if is_not_left {
            neighs.push(above - 1);
        }

        if is_not_right {
            neighs.push(above + 1);
        }
    }

    if is_not_left {
        neighs.push(position - 1);
    }

    if is_not_right {
        neighs.push(position + 1);
    }

    if is_not_bottom {
        let below = position + DIMENSION;

        neighs.push(below);

        if is_not_left {
            neighs.push(below - 1);
        }

        if is_not_right {
            neighs.push(below + 1);
        }
    }

    neighs
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526
    ";

    #[test]
    fn base_check() {
        assert_eq!("1656", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("195", process_data_adv(TEST_CASE.to_string()));
    }
}
