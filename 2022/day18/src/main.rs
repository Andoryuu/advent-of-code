use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    fs,
    ops::Add,
};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let cubes = parse(input);
    let cubes_set = HashSet::<_>::from_iter(cubes.iter());
    let neighs = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    cubes
        .iter()
        .map(|c| {
            neighs
                .iter()
                .map(|n| c + n)
                .filter(|nc| !cubes_set.contains(nc))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    let cubes = parse(input);
    let max_x = cubes.iter().map(|c| c.x).max().unwrap();
    let max_y = cubes.iter().map(|c| c.y).max().unwrap();
    let max_z = cubes.iter().map(|c| c.z).max().unwrap();
    let cubes_set = HashSet::<_>::from_iter(cubes.iter());
    let neighs = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    let mut outside = BTreeSet::<Cube>::new();
    let mut inside = BTreeSet::<Cube>::new();

    cubes
        .iter()
        .flat_map(|c| {
            neighs
                .iter()
                .map(|n| c + n)
                .filter(|nc| !cubes_set.contains(nc))
                .collect_vec()
        })
        .filter(|c| {
            let mut searched = BTreeSet::new();
            let mut queue = VecDeque::new();
            searched.insert(c.clone());
            queue.push_back(c.clone());

            while let Some(next) = queue.pop_front() {
                if inside.contains(&next) {
                    for s in searched {
                        inside.insert(s);
                    }
                    return false;
                }

                if outside.contains(&next)
                    || next.x <= 0
                    || next.x >= max_x
                    || next.y <= 0
                    || next.y >= max_y
                    || next.z <= 0
                    || next.z >= max_z
                {
                    for s in searched {
                        outside.insert(s);
                    }
                    return true;
                }

                for n in neighs
                    .iter()
                    .map(|n| &next + n)
                    .filter(|nc| !cubes_set.contains(&nc))
                {
                    if !searched.contains(&n) {
                        searched.insert(n.clone());
                        queue.push_back(n);
                    }
                }
            }

            for s in searched {
                inside.insert(s);
            }

            false
        })
        .count()
        .to_string()
}

fn parse(input: String) -> Vec<Cube> {
    input.lines().map(Cube::from).collect_vec()
}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Add<&(i32, i32, i32)> for &Cube {
    type Output = Cube;

    fn add(self, rhs: &(i32, i32, i32)) -> Self::Output {
        Cube {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
            z: self.z + rhs.2,
        }
    }
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        let (x, y, z) = value
            .split(',')
            .filter_map(|n| n.parse::<i32>().ok())
            .collect_tuple()
            .unwrap();

        Cube { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[rstest]
    #[case(TEST_CASE, "64")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned()));
    }

    #[rstest]
    #[case(TEST_CASE, "58")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned()));
    }
}
