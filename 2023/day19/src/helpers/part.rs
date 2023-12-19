use itertools::Itertools;

use super::{category::Category, operation::Operation};

pub struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

impl Part {
    pub fn get_rating(self) -> u32 {
        self.x + self.m + self.a + self.s
    }

    pub fn apply_op(&self, op: &Operation) -> Option<String> {
        match op {
            Operation::LesserThan(c, v, l) => {
                let l = l.to_owned();
                match c {
                    Category::X => {
                        if self.x < *v {
                            Some(l)
                        } else {
                            None
                        }
                    }
                    Category::M => {
                        if self.m < *v {
                            Some(l)
                        } else {
                            None
                        }
                    }
                    Category::A => {
                        if self.a < *v {
                            Some(l)
                        } else {
                            None
                        }
                    }
                    Category::S => {
                        if self.s < *v {
                            Some(l)
                        } else {
                            None
                        }
                    }
                }
            }
            Operation::GreaterThan(c, v, l) => {
                let l = l.to_owned();
                match c {
                    Category::X => {
                        if self.x > *v {
                            Some(l)
                        } else {
                            None
                        }
                    }
                    Category::M => {
                        if self.m > *v {
                            Some(l)
                        } else {
                            None
                        }
                    }
                    Category::A => {
                        if self.a > *v {
                            Some(l)
                        } else {
                            None
                        }
                    }
                    Category::S => {
                        if self.s > *v {
                            Some(l)
                        } else {
                            None
                        }
                    }
                }
            }
            Operation::Always(l) => Some(l.to_owned()),
        }
    }
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let (x, m, a, s) = value[1..(value.len() - 1)]
            .split(',')
            .collect_tuple()
            .unwrap();

        Part {
            x: x[2..].parse::<u32>().unwrap(),
            m: m[2..].parse::<u32>().unwrap(),
            a: a[2..].parse::<u32>().unwrap(),
            s: s[2..].parse::<u32>().unwrap(),
        }
    }
}
