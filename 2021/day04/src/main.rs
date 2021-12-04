mod bingo_board;

use std::{collections::BTreeSet, fs};

use bingo_board::{BingoBoard, BingoState};

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let (numbers, mut boards) = parse_data(input);

    for n in numbers.iter() {
        for b in boards.iter_mut() {
            if let BingoState::Completed(s) = b.mark(*n) {
                return s.to_string();
            }
        }
    }

    String::from("No bingo was completed.")
}

fn process_data_adv(input: String) -> String {
    let (numbers, mut boards) = parse_data(input);
    let mut boards_count = boards.len();
    let mut removed_indexes = BTreeSet::new();

    for n in numbers.iter() {
        for (i, b) in boards.iter_mut().enumerate() {
            if removed_indexes.contains(&i) {
                continue;
            }

            if let BingoState::Completed(s) = b.mark(*n) {
                if boards_count == 1 {
                    return s.to_string();
                } else {
                    removed_indexes.insert(i);
                    boards_count -= 1;
                }
            }
        }
    }

    String::from("No bingo was completed.")
}

fn parse_data(input: String) -> (Vec<u32>, Vec<BingoBoard>) {
    let lines: Vec<&str> = input.lines().collect();

    let numbers: Vec<u32> = lines
        .get(0)
        .unwrap()
        .split(',')
        .map(|v| v.trim().parse::<u32>().expect(v))
        .collect();

    let mut boards = Vec::new();
    let mut last_board = [0u32; 25];

    for (index, line) in lines.iter().skip(2).enumerate() {
        let norm_line = line.trim();

        if norm_line.is_empty() {
            boards.push(BingoBoard::new(last_board));
            last_board = [0u32; 25];
            continue;
        }

        let row_index = (index % 6) * 5;

        norm_line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .enumerate()
            .for_each(|(i, n)| last_board[row_index + i] = n);
    }

    (numbers, boards)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7
     ";

    #[test]
    fn parsing_manual_check() {
        let (numbers, boards) = parse_data(TEST_CASE.to_string());

        println!("{:?}", numbers);
        boards.iter().for_each(|b| println!("{:?}", *b));
    }

    #[test]
    fn base_check() {
        assert_eq!("4512", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("1924", process_data_adv(TEST_CASE.to_string()));
    }
}
