use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let graph = parse(input);
    let mut visited_small = BTreeSet::new();

    traverse(START, &graph, &mut visited_small).to_string()
}

fn process_data_adv(input: String) -> String {
    let graph = parse(input);
    let mut visited_small = BTreeSet::new();

    traverse_with_repeat(START, &graph, &mut visited_small, false).to_string()
}

const START: &str = "start";
const END: &str = "end";

fn parse(input: String) -> BTreeMap<String, Vec<Node>> {
    let mut res: BTreeMap<String, Vec<Node>> = BTreeMap::new();

    for line in input.trim().lines() {
        let nodes: Vec<&str> = line.trim().split('-').collect();
        let first_name = *nodes.get(0).unwrap();
        let second_name = *nodes.get(1).unwrap();

        let first_node = Node {
            name: first_name.to_owned(),
            is_small: is_all_lower(first_name),
        };

        let second_node = Node {
            name: second_name.to_owned(),
            is_small: is_all_lower(second_name),
        };

        if let Some(first_nodes) = res.get_mut(first_name) {
            first_nodes.push(second_node);
        } else {
            res.insert(first_name.to_owned(), vec![second_node]);
        }

        if let Some(second_nodes) = res.get_mut(second_name) {
            second_nodes.push(first_node);
        } else {
            res.insert(second_name.to_owned(), vec![first_node]);
        }
    }

    res
}

fn traverse(
    from: &str,
    graph: &BTreeMap<String, Vec<Node>>,
    visited_small: &mut BTreeSet<String>,
) -> u32 {
    if let Some(paths) = graph.get(from) {
        paths
            .iter()
            .filter(|p| p.name != START)
            .map(|path| {
                if path.name == END {
                    return 1;
                }

                if path.is_small {
                    if visited_small.contains(&path.name) {
                        return 0;
                    } else {
                        visited_small.insert(path.name.to_owned());
                    }
                }

                let end_count = traverse(&path.name, graph, visited_small);

                if path.is_small {
                    visited_small.remove(&path.name);
                }

                end_count
            })
            .sum::<u32>()
    } else {
        0
    }
}

fn traverse_with_repeat(
    from: &str,
    graph: &BTreeMap<String, Vec<Node>>,
    visited_small: &mut BTreeSet<String>,
    repeated_once: bool,
) -> u32 {
    if let Some(paths) = graph.get(from) {
        paths
            .iter()
            .filter(|p| p.name != START)
            .map(|path| {
                if path.name == END {
                    return 1;
                }

                let mut added_repeat = false;

                if path.is_small {
                    if !visited_small.contains(&path.name) {
                        visited_small.insert(path.name.to_owned());
                    } else if repeated_once {
                        return 0;
                    } else {
                        added_repeat = true;
                    }
                }

                let end_count = traverse_with_repeat(
                    &path.name,
                    graph,
                    visited_small,
                    repeated_once || added_repeat,
                );

                if path.is_small && !added_repeat {
                    visited_small.remove(&path.name);
                }

                end_count
            })
            .sum::<u32>()
    } else {
        0
    }
}

fn is_all_lower(input: &str) -> bool {
    input.chars().all(|c| c.is_ascii_lowercase())
}

#[derive(Debug)]
struct Node {
    name: String,
    is_small: bool,
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE_1: &str = "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end
    ";

    const TEST_CASE_2: &str = "dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc
    ";

    const TEST_CASE_3: &str = "fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW
    ";

    #[test]
    fn base_check_1() {
        assert_eq!("10", process_data(TEST_CASE_1.to_string()));
    }

    #[test]
    fn base_check_2() {
        assert_eq!("19", process_data(TEST_CASE_2.to_string()));
    }

    #[test]
    fn base_check_3() {
        assert_eq!("226", process_data(TEST_CASE_3.to_string()));
    }

    #[test]
    fn adv_check_1() {
        assert_eq!("36", process_data_adv(TEST_CASE_1.to_string()));
    }

    #[test]
    fn adv_check_2() {
        assert_eq!("103", process_data_adv(TEST_CASE_2.to_string()));
    }

    #[test]
    fn adv_check_3() {
        assert_eq!("3509", process_data_adv(TEST_CASE_3.to_string()));
    }
}
