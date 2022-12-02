use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    input
        .lines()
        .filter_map(|line| line.split(' ').map(Rps::from).next_tuple())
        .map(get_score)
        .sum::<u32>()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    input
        .lines()
        .filter_map(|line| {
            let mut vals = line.split(' ');
            vals.next()
                .map(Rps::from)
                .zip(vals.next().map(GameResult::from))
        })
        .map(get_score_by_win)
        .sum::<u32>()
        .to_string()
}

fn get_score_by_win((elf, res): (Rps, GameResult)) -> u32 {
    match res {
        GameResult::Loss => get_value(match elf {
            Rps::Rock => Rps::Scissors,
            Rps::Paper => Rps::Rock,
            Rps::Scissors => Rps::Paper,
        }),
        GameResult::Draw => 3 + get_value(elf),
        GameResult::Win => {
            6 + get_value(match elf {
                Rps::Rock => Rps::Paper,
                Rps::Paper => Rps::Scissors,
                Rps::Scissors => Rps::Rock,
            })
        }
    }
}

fn get_score((elf, me): (Rps, Rps)) -> u32 {
    let result_score = match (elf, me) {
        (Rps::Rock, Rps::Paper)
        | (Rps::Paper, Rps::Scissors)
        | (Rps::Scissors, Rps::Rock) => 6,

        (Rps::Rock, Rps::Scissors)
        | (Rps::Paper, Rps::Rock)
        | (Rps::Scissors, Rps::Paper) => 0,

        _ => 3,
    };

    result_score + get_value(me)
}

fn get_value(rps: Rps) -> u32 {
    match rps {
        Rps::Rock => 1,
        Rps::Paper => 2,
        Rps::Scissors => 3,
    }
}

#[derive(Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Rps {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Rps::Rock,
            "B" | "Y" => Rps::Paper,
            "C" | "Z" => Rps::Scissors,
            _ => panic!("{}", value),
        }
    }
}

enum GameResult {
    Loss,
    Draw,
    Win,
}

impl From<&str> for GameResult {
    fn from(value: &str) -> Self {
        match value {
            "X" => GameResult::Loss,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => panic!("{}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
A Y
B X
C Z
";

    #[rstest]
    fn base_check() {
        assert_eq!("15", process_data(TEST_CASE.to_string()));
    }

    #[rstest]
    fn adv_check() {
        assert_eq!("12", process_data_adv(TEST_CASE.to_string()));
    }
}
