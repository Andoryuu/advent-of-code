use itertools::Itertools;

use super::category::Category;

pub enum Operation {
    LesserThan(Category, u32, String),
    GreaterThan(Category, u32, String),
    Always(String),
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        if let Some((op, label)) = value.split(':').collect_tuple() {
            match &op[1..2] {
                ">" => Operation::GreaterThan(
                    op[..1].into(),
                    op[2..].parse().unwrap(),
                    label.to_owned(),
                ),
                "<" => Operation::LesserThan(
                    op[..1].into(),
                    op[2..].parse().unwrap(),
                    label.to_owned(),
                ),
                x => panic!("unknown operation '{}' in '{}'", x, op),
            }
        } else {
            Operation::Always(value.to_owned())
        }
    }
}
