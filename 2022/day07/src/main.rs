#![feature(if_let_guard)]

use std::{cell::RefCell, fs, rc::Rc};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let cmds = parse(input);
    let structure = generate_structure(cmds);
    let stats = get_dir_statistics(&structure);

    stats
        .1
        .iter()
        .map(|dir| dir.1)
        .filter(|&size| size <= 100_000)
        .sum::<u32>()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    let cmds = parse(input);
    let structure = generate_structure(cmds);
    let stats = get_dir_statistics(&structure);

    let to_delete = stats.0 - 40_000_000;
    stats
        .1
        .iter()
        .map(|dir| dir.1)
        .sorted()
        .find(|&size| size >= to_delete)
        .unwrap()
        .to_string()
}

fn get_dir_statistics(root: &Rc<RefCell<TreeNode>>) -> (u32, Vec<(String, u32)>) {
    let mut items = vec![];
    let mut size = 0u32;

    for child in root.borrow().children.iter() {
        let child_ref = child.borrow();

        if child_ref.is_dir {
            let (inner_size, mut inner) = get_dir_statistics(child);
            items.push((child_ref.name.clone(), inner_size));
            items.append(&mut inner);
            size += inner_size;
        } else {
            size += child_ref.size;
        }
    }

    (size, items)
}

fn generate_structure(cmds: Vec<CmdLine>) -> Rc<RefCell<TreeNode>> {
    let root = Rc::new(RefCell::new(TreeNode::dir("/".to_owned())));
    let mut stack = vec![root.clone()];

    for cmd in cmds {
        match cmd {
            CmdLine::Command(c) => match c {
                Command::CdRoot => {
                    let root = stack.first().unwrap().clone();
                    stack.clear();
                    stack.push(root);
                }
                Command::CdFolder(name) => {
                    let node = Rc::new(RefCell::new(TreeNode::dir(name)));
                    stack
                        .last()
                        .unwrap()
                        .borrow_mut()
                        .children
                        .push(node.clone());
                    stack.push(node.clone());
                }
                Command::CdUp => {
                    if stack.len() > 1 {
                        stack.pop();
                    }
                }
                Command::List => {}
            },
            CmdLine::Output(o) => match o {
                Output::Dir(_) => {}
                Output::File(size, name) => {
                    let file = Rc::new(RefCell::new(TreeNode::file(name, size)));
                    stack
                        .last()
                        .unwrap()
                        .borrow_mut()
                        .children
                        .push(file.clone());
                }
            },
        }
    }

    root
}

fn parse(input: String) -> Vec<CmdLine> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
        .map(|split| match split.as_slice() {
            ["$", cmd @ ..] => match cmd {
                ["cd", path @ ..] => match path {
                    ["/"] => CmdLine::Command(Command::CdRoot),
                    [".."] => CmdLine::Command(Command::CdUp),
                    [x] => CmdLine::Command(Command::CdFolder(x.to_string())),
                    _ => panic!("unexpected cd parameters: {:?}", path),
                },
                ["ls"] => CmdLine::Command(Command::List),
                _ => panic!("unknown command: {:?}", cmd),
            },
            ["dir", name] => CmdLine::Output(Output::Dir(name.to_string())),
            [size_raw, name] if let Ok(size) = size_raw.parse::<u32>() => {
                CmdLine::Output(Output::File(size, name.to_string()))
            }
            _ => panic!("unexpected line: {:?}", split),
        })
        .collect_vec()
}

enum CmdLine {
    Command(Command),
    Output(Output),
}

enum Command {
    CdRoot,
    CdFolder(String),
    CdUp,
    List,
}

enum Output {
    Dir(String),
    File(u32, String),
}

struct TreeNode {
    name: String,
    is_dir: bool,
    size: u32,
    children: Vec<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn dir(name: String) -> Self {
        TreeNode {
            name,
            is_dir: true,
            size: 0,
            children: vec![],
        }
    }

    fn file(name: String, size: u32) -> Self {
        TreeNode {
            name,
            is_dir: false,
            size,
            children: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[rstest]
    fn base_check() {
        assert_eq!("95437", process_data(TEST_CASE.to_string()));
    }

    #[rstest]
    fn adv_check() {
        assert_eq!("24933642", process_data_adv(TEST_CASE.to_string()));
    }
}
