use crate::components::point::Point;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    UpLeft,
    UpRight,
    DownRight,
    DownLeft,
}

impl Direction {
    pub fn try_from(a: &Point, b: &Point) -> Option<Direction> {
        match (*b - *a).coordinates_signums() {
            (1, 1) => Some(Direction::DownRight),
            (-1, 1) => Some(Direction::DownLeft),
            (1, -1) => Some(Direction::UpRight),
            (-1, -1) => Some(Direction::UpLeft),
            _ => None, // bet on same-line rectangles not being the answer
        }
    }

    pub fn invert(&self) -> Direction {
        match self {
            Direction::UpLeft => Direction::DownRight,
            Direction::UpRight => Direction::DownLeft,
            Direction::DownRight => Direction::UpLeft,
            Direction::DownLeft => Direction::UpRight,
        }
    }
}
