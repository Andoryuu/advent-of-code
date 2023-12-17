use super::direction::Direction;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub index: usize,
    pub dist: u32,
    pub dir: Direction,
    pub straight: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.index.cmp(&other.index))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
