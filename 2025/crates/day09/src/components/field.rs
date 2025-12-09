use itertools::Itertools;

use crate::components::{orientation::Orientation, oriented_point::OrientedPoint, vector::Vector};

pub struct Field {
    points: Vec<OrientedPoint>,
    vecs: Vec<Vector>,
    orientation: Orientation,
}

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        let vecs = value
            .lines()
            .filter_map(|line| line.try_into().ok())
            .collect_vec()
            .iter()
            .circular_tuple_windows()
            .map(Vector::from_points)
            .collect_vec();

        let points = vecs
            .iter()
            .circular_tuple_windows()
            .map(OrientedPoint::from)
            .collect_vec();

        let orientation = if points
            .iter()
            .map(|p| p.orientation_signum())
            .sum::<isize>()
            > 0
        {
            Orientation::Clockwise
        } else {
            Orientation::Counter
        };

        Field {
            points,
            vecs,
            orientation,
        }
    }
}

impl Field {
    pub fn get_largest_inner_square_area(self) -> usize {
        self.points
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| a.is_facing(b, self.orientation))
            .sorted_by_key(|(a, b)| a.get_square_area_with(b))
            .rev()
            .filter(|o| {
                let base_vec = Vector::from_oriented(*o);
                self.vecs.iter().all(|v| v.square_avoids(&base_vec))
            })
            .map(|(a, b)| a.get_square_area_with(b))
            .next()
            .unwrap()
    }
}
