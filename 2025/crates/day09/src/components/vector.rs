use crate::components::{oriented_point::OrientedPoint, point::Point};

pub struct Vector {
    pub start: Point,
    pub end: Point,
}

impl Vector {
    pub fn from_points((a, b): (&Point, &Point)) -> Vector {
        Vector { start: *a, end: *b }
    }

    pub fn from_oriented((a, b): (&OrientedPoint, &OrientedPoint)) -> Vector {
        Vector {
            start: (*a).into(),
            end: (*b).into(),
        }
    }

    pub fn dimensions(&self) -> Point {
        self.end - self.start
    }

    pub fn square_intersects(&self, other: &Vector) -> bool {
        let min_x = other.start.x.min(other.end.x);
        let min_y = other.start.y.min(other.end.y);
        let max_x = other.start.x.max(other.end.x);
        let max_y = other.start.y.max(other.end.y);

        (self.start.x <= min_x && self.end.x <= min_x)
            || (self.start.y <= min_y && self.end.y <= min_y)
            || (self.start.x >= max_x && self.end.x >= max_x)
            || (self.start.y >= max_y && self.end.y >= max_y)
    }
}
