use itertools::Itertools;

use super::operation::Operation;

pub struct Workflow {
    pub label: String,
    pub ops: Vec<Operation>,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let (label, rest) = value.split('{').collect_tuple().unwrap();
        let ops = rest[..(rest.len() - 1)].split(',').map_into().collect_vec();

        Workflow {
            label: label.to_owned(),
            ops,
        }
    }
}
