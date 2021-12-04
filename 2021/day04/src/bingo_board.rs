use std::collections::BTreeMap;

pub enum BingoState {
    Uncompleted,
    Completed(u32),
}

#[derive(Debug)]
pub struct BingoBoard {
    values: [u32; 25],
    hits: [bool; 25],
    values_set: BTreeMap<u32, usize>,
}

impl BingoBoard {
    pub fn new(values: [u32; 25]) -> Self {
        let mut board_values_set = BTreeMap::new();
        for (i, v) in values.iter().enumerate() {
            board_values_set.insert(*v, i);
        }

        BingoBoard {
            values,
            hits: [false; 25],
            values_set: board_values_set
        }
    }

    pub fn mark(&mut self, value: u32) -> BingoState {
        if !self.has_value(&value) {
            return BingoState::Uncompleted;
        }

        let val_index = *self.values_set.get(&value).unwrap();

        self.hits[val_index] = true;

        self.try_get_score(value)
            .map(BingoState::Completed)
            .unwrap_or(BingoState::Uncompleted)
    }

    fn is_completed(&self) -> bool {
        // horizontal direction
        for i in 0..5 {
            let row = i * 5;
            if self.hits[row]
                && self.hits[row + 1]
                && self.hits[row + 2]
                && self.hits[row + 3]
                && self.hits[row + 4]
            {
                return true;
            }
        }

        // vertical direction
        for i in 0..5 {
            if self.hits[i]
                && self.hits[i + 5]
                && self.hits[i + 10]
                && self.hits[i + 15]
                && self.hits[i + 20]
            {
                return true;
            }
        }

        false
    }

    fn try_get_score(&self, last_num: u32) -> Option<u32> {
        if !self.is_completed() {
            return None;
        }

        let mut unhit_sum = 0u32;

        for (index, is_hit) in self.hits.iter().enumerate() {
            if !*is_hit {
                unhit_sum += self.values[index];
            }
        }

        Some(unhit_sum * last_num)
    }

    fn has_value(&self, value: &u32) -> bool {
        self.values_set.contains_key(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_score() {
        let test_case = prepare_completed_board();

        assert_eq!(4512, test_case.try_get_score(24).unwrap());
    }

    #[test]
    fn correct_completed() {
        let test_case = prepare_completed_board();

        assert!(test_case.is_completed());
    }

    fn prepare_completed_board() -> BingoBoard {
        let board_values = [
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ];

        let mut board_values_set = BTreeMap::new();
        for (i, v) in board_values.iter().enumerate() {
            board_values_set.insert(*v, i);
        }

        BingoBoard {
            values: board_values,
            hits: [
                true, true, true, true, true, false, false, false, true, false, false, false, true,
                false, false, false, true, false, false, true, true, true, false, false, true,
            ],
            values_set: board_values_set,
        }
    }
}
