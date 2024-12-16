use nalgebra::{Point2, Vector2};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn apply_to(&self, point: Point2<isize>) -> Point2<isize> {
        point
            + match self {
                Direction::Up => Vector2::new(-1, 0),
                Direction::Down => Vector2::new(1, 0),
                Direction::Left => Vector2::new(0, -1),
                Direction::Right => Vector2::new(0, 1),
            }
    }

    pub fn turn_clock(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn turn_anticlock(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
