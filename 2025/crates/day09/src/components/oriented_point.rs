use crate::components::{
    direction::Direction, orientation::Orientation, point::Point, vector::Vector,
};

#[derive(Clone, Copy)]
pub struct OrientedPoint {
    point: Point,
    orientation: Orientation,
    direction: Direction,
}

impl From<OrientedPoint> for Point {
    fn from(val: OrientedPoint) -> Self {
        val.point
    }
}

impl OrientedPoint {
    pub fn from((v1, v2): (&Vector, &Vector)) -> OrientedPoint {
        let (a1, a2) = v1.dimensions().coordinates_signums();
        let (b1, b2) = v2.dimensions().coordinates_signums();

        let (o, d) = if a1 == 0 {
            match (a2, b1) {
                (1, 1) => (Orientation::Counter, Direction::UpRight),
                (-1, 1) => (Orientation::Clockwise, Direction::DownRight),
                (1, -1) => (Orientation::Clockwise, Direction::UpLeft),
                (-1, -1) => (Orientation::Counter, Direction::DownLeft),
                _ => panic!(),
            }
        } else {
            match (a1, b2) {
                (1, 1) => (Orientation::Clockwise, Direction::DownLeft),
                (-1, 1) => (Orientation::Counter, Direction::DownRight),
                (1, -1) => (Orientation::Counter, Direction::UpLeft),
                (-1, -1) => (Orientation::Clockwise, Direction::UpRight),
                _ => panic!(),
            }
        };

        OrientedPoint {
            point: v1.end,
            orientation: o,
            direction: d,
        }
    }

    pub fn is_facing(&self, other: &OrientedPoint, orientation: Orientation) -> bool {
        if let Some(rel_dir) = Direction::try_from(&self.point, &other.point) {
            let op_dir = rel_dir.invert();

            return (self.orientation == orientation) == (self.direction == rel_dir)
                || (other.orientation == orientation) == (other.direction == op_dir);
        }

        false
    }

    pub fn get_square_area_with(&self, other: &OrientedPoint) -> usize {
        self.point.get_square_area_with(&other.point)
    }

    pub fn orientation_signum(&self) -> isize {
        match self.orientation {
            Orientation::Clockwise => 1,
            Orientation::Counter => -1,
        }
    }
}
