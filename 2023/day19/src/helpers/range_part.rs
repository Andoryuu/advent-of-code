use super::{category::Category, operation::Operation};

#[derive(Clone, Copy)]
pub struct RangePart {
    pub x: (u32, u32),
    pub m: (u32, u32),
    pub a: (u32, u32),
    pub s: (u32, u32),
}

impl RangePart {
    pub fn get_combinations(self) -> usize {
        (1 + self.x.1 - self.x.0) as usize
            * (1 + self.m.1 - self.m.0) as usize
            * (1 + self.a.1 - self.a.0) as usize
            * (1 + self.s.1 - self.s.0) as usize
    }

    pub fn apply_op(self, op: &Operation) -> (Option<(RangePart, String)>, Option<RangePart>) {
        match op {
            Operation::LesserThan(cat, val, label) => match cat {
                Category::X => {
                    if self.x.0 >= *val {
                        (None, Some(self))
                    } else if self.x.1 < *val {
                        (Some((self, label.to_owned())), None)
                    } else {
                        (
                            Some((
                                RangePart {
                                    x: (self.x.0, val - 1),
                                    ..self
                                },
                                label.to_owned(),
                            )),
                            Some(RangePart {
                                x: (*val, self.x.1),
                                ..self
                            }),
                        )
                    }
                }
                Category::M => {
                    if self.m.0 >= *val {
                        (None, Some(self))
                    } else if self.m.1 < *val {
                        (Some((self, label.to_owned())), None)
                    } else {
                        (
                            Some((
                                RangePart {
                                    m: (self.m.0, val - 1),
                                    ..self
                                },
                                label.to_owned(),
                            )),
                            Some(RangePart {
                                m: (*val, self.m.1),
                                ..self
                            }),
                        )
                    }
                }
                Category::A => {
                    if self.a.0 >= *val {
                        (None, Some(self))
                    } else if self.a.1 < *val {
                        (Some((self, label.to_owned())), None)
                    } else {
                        (
                            Some((
                                RangePart {
                                    a: (self.a.0, val - 1),
                                    ..self
                                },
                                label.to_owned(),
                            )),
                            Some(RangePart {
                                a: (*val, self.a.1),
                                ..self
                            }),
                        )
                    }
                }
                Category::S => {
                    if self.s.0 >= *val {
                        (None, Some(self))
                    } else if self.s.1 < *val {
                        (Some((self, label.to_owned())), None)
                    } else {
                        (
                            Some((
                                RangePart {
                                    s: (self.s.0, val - 1),
                                    ..self
                                },
                                label.to_owned(),
                            )),
                            Some(RangePart {
                                s: (*val, self.s.1),
                                ..self
                            }),
                        )
                    }
                }
            },
            Operation::GreaterThan(cat, val, label) => match cat {
                Category::X => {
                    if self.x.1 <= *val {
                        (None, Some(self))
                    } else if self.x.0 > *val {
                        (Some((self, label.to_owned())), None)
                    } else {
                        (
                            Some((
                                RangePart {
                                    x: (val + 1, self.x.1),
                                    ..self
                                },
                                label.to_owned(),
                            )),
                            Some(RangePart {
                                x: (self.x.0, *val),
                                ..self
                            }),
                        )
                    }
                }
                Category::M => {
                    if self.m.1 <= *val {
                        (None, Some(self))
                    } else if self.m.0 > *val {
                        (Some((self, label.to_owned())), None)
                    } else {
                        (
                            Some((
                                RangePart {
                                    m: (val + 1, self.m.1),
                                    ..self
                                },
                                label.to_owned(),
                            )),
                            Some(RangePart {
                                m: (self.m.0, *val),
                                ..self
                            }),
                        )
                    }
                }
                Category::A => {
                    if self.a.1 <= *val {
                        (None, Some(self))
                    } else if self.a.0 > *val {
                        (Some((self, label.to_owned())), None)
                    } else {
                        (
                            Some((
                                RangePart {
                                    a: (val + 1, self.a.1),
                                    ..self
                                },
                                label.to_owned(),
                            )),
                            Some(RangePart {
                                a: (self.a.0, *val),
                                ..self
                            }),
                        )
                    }
                }
                Category::S => {
                    if self.s.1 <= *val {
                        (None, Some(self))
                    } else if self.s.0 > *val {
                        (Some((self, label.to_owned())), None)
                    } else {
                        (
                            Some((
                                RangePart {
                                    s: (val + 1, self.s.1),
                                    ..self
                                },
                                label.to_owned(),
                            )),
                            Some(RangePart {
                                s: (self.s.0, *val),
                                ..self
                            }),
                        )
                    }
                }
            },
            Operation::Always(label) => (Some((self, label.to_owned())), None),
        }
    }
}
