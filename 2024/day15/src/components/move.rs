#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn apply_to(&self, (row, col): (isize, isize)) -> (isize, isize) {
        match self {
            Move::Up => (row - 1, col),
            Move::Down => (row + 1, col),
            Move::Left => (row, col - 1),
            Move::Right => (row, col + 1),
        }
    }
}

impl TryFrom<char> for Move {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Move::Up),
            'v' => Ok(Move::Down),
            '<' => Ok(Move::Left),
            '>' => Ok(Move::Right),
            _ => Err("invalid character".to_owned()),
        }
    }
}
