use std::fs;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let (p1_start, p2_start) = parse(input);

    let (mut p1_pos, mut p1_score) = (p1_start, 0u32);
    let (mut p2_pos, mut p2_score) = (p2_start, 0u32);
    let mut roll_count = 0;

    for chunk in &(1..).chunks(3) {
        let increment: u32 = chunk.map(|c| (c - 1) % 100 + 1).sum();

        if (roll_count / 3) % 2 == 0 {
            p1_pos = (p1_pos + increment - 1) % 10 + 1;
            p1_score += p1_pos;
            roll_count += 3;

            if p1_score >= 1_000 {
                return (p2_score * roll_count).to_string();
            }
        } else {
            p2_pos = (p2_pos + increment - 1) % 10 + 1;
            p2_score += p2_pos;
            roll_count += 3;

            if p2_score >= 1_000 {
                return (p1_score * roll_count).to_string();
            }
        }
    }

    panic!("Reached the end of infinite iterator.")
}

fn process_data_adv(input: String) -> String {
    let (p1_start, p2_start) = parse(input);

    let (p1_wins, p2_wins) = dirac_step(p1_start, 0, p2_start, 0, true, 1);

    p1_wins.max(p2_wins).to_string()
}

fn parse(input: String) -> (u32, u32) {
    lazy_static! {
        static ref PLAYER_RE: Regex = Regex::new("Player \\d starting position: (\\d+)").unwrap();
    }

    let startings: Vec<u32> = input
        .trim()
        .lines()
        .map(|l| {
            PLAYER_RE
                .captures(l)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str())
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap()
        })
        .collect();

    (*startings.get(0).unwrap(), *startings.get(1).unwrap())
}

const DIRAC_TARGET: u32 = 21;
const INCR_SCORES: [(u32, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn dirac_step(
    p1_pos: u32,
    p1_score: u32,
    p2_pos: u32,
    p2_score: u32,
    p1_turn: bool,
    multiplier: u64,
) -> (u64, u64) {
    if p1_turn {
        INCR_SCORES
            .iter()
            .fold((0u64, 0u64), |(p1_wins, p2_wins), (incr, score)| {
                let (new_pos, new_score) = update_player(p1_pos, p1_score, *incr);
                let (new_p1_wins, new_p2_wins) = if new_score >= DIRAC_TARGET {
                    (*score * multiplier, 0)
                } else {
                    dirac_step(
                        new_pos,
                        new_score,
                        p2_pos,
                        p2_score,
                        !p1_turn,
                        *score * multiplier,
                    )
                };
                (p1_wins + new_p1_wins, p2_wins + new_p2_wins)
            })
    } else {
        INCR_SCORES
            .iter()
            .fold((0u64, 0u64), |(p1_wins, p2_wins), (incr, score)| {
                let (new_pos, new_score) = update_player(p2_pos, p2_score, *incr);
                let (new_p1_wins, new_p2_wins) = if new_score >= DIRAC_TARGET {
                    (0, *score * multiplier)
                } else {
                    dirac_step(
                        p1_pos,
                        p1_score,
                        new_pos,
                        new_score,
                        !p1_turn,
                        *score * multiplier,
                    )
                };
                (p1_wins + new_p1_wins, p2_wins + new_p2_wins)
            })
    }
}

fn update_player(pos: u32, score: u32, roll: u32) -> (u32, u32) {
    let new_pos = (pos + roll - 1) % 10 + 1;
    (new_pos, score + new_pos)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "Player 1 starting position: 4
    Player 2 starting position: 8
    ";

    #[test]
    fn base_check() {
        assert_eq!("739785", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("444356092776315", process_data_adv(TEST_CASE.to_string()));
    }
}
