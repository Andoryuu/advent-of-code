use nalgebra::Point2;

use super::direction::Direction;

#[derive(Eq, PartialEq)]
pub struct Score {
    pub score: isize,
    pub position: Point2<isize>,
    pub direction: Direction,
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}
