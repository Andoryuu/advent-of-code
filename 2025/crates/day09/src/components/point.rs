use std::ops::Sub;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .split_once(',')
            .and_then(|(a, b)| a.parse::<isize>().ok().zip(b.parse::<isize>().ok()))
            .map(|(x, y)| Point { x, y })
            .ok_or(value.to_owned())
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point {
    pub fn get_square_area_with(&self, other: &Point) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }

    pub fn coordinates_signums(&self) -> (isize, isize) {
        (self.x.signum(), self.y.signum())
    }
}
