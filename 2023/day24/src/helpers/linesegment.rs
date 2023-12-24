pub type LineSegment = ((f64, f64), (f64, f64));

pub trait Intersectable {
    fn intersects(self, other: Self) -> bool;
}

impl Intersectable for LineSegment {
    fn intersects(self, other: Self) -> bool {
        // see https://en.wikipedia.org/wiki/Intersection_(geometry)#Two_line_segments

        let ((x1, y1), (x2, y2)) = self;
        let ((x3, y3), (x4, y4)) = other;

        let a = x2 - x1;
        let b = x4 - x3;
        let c = x3 - x1;
        let e = y2 - y1;
        let f = y4 - y3;
        let g = y3 - y1;

        let d = b * e - f * a;
        if d == 0.0 {
            return false;
        }

        let s = (b * g - f * c) / (b * e - f * a);
        if !(0.0..=1.0).contains(&s) {
            return false;
        }

        let t = if b == 0.0 {
            (e * s - g) / f
        } else {
            (a * s - c) / b
        };

        (0.0..=1.0).contains(&t)
    }
}
