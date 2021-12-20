mod node;

use std::fs;

use itertools::Itertools;
use node::Node;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let mut node: Option<Node> = None;

    for line in input.trim().lines().map(|l| l.trim()) {
        if let Some(n) = node {
            node = Some(n.add(Node::from(line)));
        } else {
            node = Some(Node::from(line));
        }
    }

    node.unwrap().magnitude().to_string()
}

fn process_data_adv(input: String) -> String {
    input
        .trim()
        .lines()
        .map(|l| l.trim())
        .permutations(2)
        .filter_map(|a| {
            a.get(0)
                .and_then(|s1| a.get(1).map(|s2| (s1.to_owned(), s2.to_owned())))
        })
        .map(|(s1, s2)| Node::from(s1).add(Node::from(s2)))
        .map(|n| n.magnitude())
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    ";

    #[test]
    fn base_check() {
        assert_eq!("4140", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("3993", process_data_adv(TEST_CASE.to_string()));
    }
}
