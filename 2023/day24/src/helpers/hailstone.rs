use itertools::Itertools;

use super::linesegment::LineSegment;

pub struct Hailstone {
    pub x: isize,
    pub y: isize,
    pub z: isize,
    pub vx: isize,
    pub vy: isize,
    pub vz: isize,
}

impl Hailstone {
    pub fn clamped_xy(&self, min: isize, max: isize) -> Option<LineSegment> {
        // see https://en.wikipedia.org/wiki/Liang%E2%80%93Barsky_algorithm

        let min = min as f64;
        let max = max as f64;

        let x1 = self.x as f64;
        let y1 = self.y as f64;
        let x2 = self.vx as f64 * max + x1;
        let y2 = self.vy as f64 * max + y1;

        let p1 = x1 - x2;
        let p2 = -p1;
        let p3 = y1 - y2;
        let p4 = -p3;

        let q1 = x1 - min;
        let q2 = max - x1;
        let q3 = y1 - min;
        let q4 = max - y1;

        if (p1 == 0.0 && q1 < 0.0)
            || (p2 == 0.0 && q2 < 0.0)
            || (p3 == 0.0 && q3 < 0.0)
            || (p4 == 0.0 && q4 < 0.0)
        {
            return None;
        }

        let mut posarr = vec![1.0];
        let mut negarr = vec![0.0];

        if p1 != 0.0 {
            let r1 = q1 / p1;
            let r2 = q2 / p2;

            if p1 < 0.0 {
                negarr.push(r1);
                posarr.push(r2);
            } else {
                negarr.push(r2);
                posarr.push(r1);
            }
        }
        if p3 != 0.0 {
            let r3 = q3 / p3;
            let r4 = q4 / p4;

            if p3 < 0.0 {
                negarr.push(r3);
                posarr.push(r4);
            } else {
                negarr.push(r4);
                posarr.push(r3);
            }
        }

        let rn1 = negarr.into_iter().reduce(|m, n| m.max(n)).unwrap();
        let rn2 = posarr.into_iter().reduce(|m, n| m.min(n)).unwrap();

        if rn1 > rn2 {
            return None;
        }

        Some((
            (x1 + p2 * rn1, y1 + p4 * rn1),
            (x1 + p2 * rn2, y1 + p4 * rn2),
        ))
    }
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let (x, y, z, vx, vy, vz) = value
            .split([',', '@'])
            .map(|p| p.trim().parse().unwrap())
            .next_tuple()
            .unwrap();

        Hailstone {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        }
    }
}
