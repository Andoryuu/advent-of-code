use fxhash::FxHashMap;

use super::tile::Tile;

#[derive(Debug)]
pub enum Field {
    Normal(FxHashMap<(isize, isize), Tile>),
    Wide(FxHashMap<(isize, isize), Tile>),
}

impl Field {
    pub fn sum_boxes(self) -> isize {
        match self {
            Field::Normal(field) => field,
            Field::Wide(field) => field,
        }
        .into_iter()
        .filter_map(|((row, col), tile)| {
            if tile == Tile::Box {
                Some(row * 100 + col)
            } else {
                None
            }
        })
        .sum()
    }
}
