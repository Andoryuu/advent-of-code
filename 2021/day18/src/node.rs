use nom::{
    branch::alt,
    character::complete::{char, digit0},
    combinator::{map, map_opt},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
enum NodeValue {
    Literal(u32),
    Child(Box<Node>),
}

#[derive(Clone, Copy, std::cmp::PartialEq)]
enum NodeCommand {
    AddLeft(u32),
    AddRight(u32),
    Done,
    None,
}

#[derive(Debug)]
pub struct Node {
    left: NodeValue,
    right: NodeValue,
}

impl Node {
    pub fn from(s: &str) -> Self {
        let (rem, node) = parser_tuple(s).unwrap();

        if !rem.is_empty() {
            panic!("Unparsed data: {}", rem);
        }

        node
    }

    pub fn add(self, value: Node) -> Node {
        let mut res = Node {
            left: NodeValue::Child(self.into()),
            right: NodeValue::Child(value.into()),
        };

        res.normalize();

        res
    }

    pub fn magnitude(&self) -> u32 {
        let left = match &self.left {
            NodeValue::Literal(v) => *v,
            NodeValue::Child(n) => n.magnitude(),
        };
        let right = match &self.right {
            NodeValue::Literal(v) => *v,
            NodeValue::Child(n) => n.magnitude(),
        };

        (left * 3) + (right * 2)
    }

    pub fn to_string(&self) -> String {
        let left = match &self.left {
            NodeValue::Literal(v) => v.to_string(),
            NodeValue::Child(bn) => bn.to_string(),
        };
        let right = match &self.right {
            NodeValue::Literal(v) => v.to_string(),
            NodeValue::Child(bn) => bn.to_string(),
        };

        format!("[{},{}]", left, right)
    }

    fn normalize(&mut self) {
        loop {
            if self.explode() {
                continue;
            }

            if self.split() {
                continue;
            }

            break;
        }
    }

    fn explode(&mut self) -> bool {
        explode(self, 0) != NodeCommand::None
    }

    fn split(&mut self) -> bool {
        if match &mut self.left {
            NodeValue::Literal(v) => {
                if *v > 9 {
                    self.left = NodeValue::Child(Node::from_split(*v).into());
                    true
                } else {
                    false
                }
            }
            NodeValue::Child(n) => n.split(),
        } {
            return true;
        }

        match &mut self.right {
            NodeValue::Literal(v) => {
                if *v > 9 {
                    self.right = NodeValue::Child(Node::from_split(*v).into());
                    true
                } else {
                    false
                }
            }
            NodeValue::Child(n) => n.split(),
        }
    }

    fn from_split(v: u32) -> Self {
        Node {
            left: NodeValue::Literal(v / 2),
            right: NodeValue::Literal(v / 2 + (v % 2)),
        }
    }
}

fn explode(node: &mut Node, layer: u32) -> NodeCommand {
    if let NodeValue::Child(n) = &mut node.left {
        if layer == 3 {
            let left = match n.left {
                NodeValue::Literal(v) => v,
                NodeValue::Child(_) => panic!("Wrong structure, too deep."),
            };

            let right = match n.right {
                NodeValue::Literal(v) => v,
                NodeValue::Child(_) => panic!("Wrong structure, too deep."),
            };

            match &mut node.right {
                NodeValue::Literal(v) => node.right = NodeValue::Literal(right + *v),
                NodeValue::Child(n) => incr_first_left(n, right),
            }

            node.left = NodeValue::Literal(0);

            return NodeCommand::AddLeft(left);
        } else {
            let cmd = explode(n, layer + 1);

            if let NodeCommand::AddRight(v) = cmd {
                match &mut node.right {
                    NodeValue::Literal(e) => node.right = NodeValue::Literal(v + *e),
                    NodeValue::Child(n) => incr_first_left(n, v),
                }

                return NodeCommand::Done;
            } else if cmd != NodeCommand::None {
                return cmd;
            }
        }
    };

    if let NodeValue::Child(n) = &mut node.right {
        if layer == 3 {
            let left = match n.left {
                NodeValue::Literal(v) => v,
                NodeValue::Child(_) => panic!("Wrong structure, too deep."),
            };

            match &mut node.left {
                NodeValue::Literal(v) => node.left = NodeValue::Literal(left + *v),
                NodeValue::Child(n) => incr_first_right(n, left),
            }

            let right = match n.right {
                NodeValue::Literal(v) => v,
                NodeValue::Child(_) => panic!("Wrong structure, too deep."),
            };

            node.right = NodeValue::Literal(0);

            return NodeCommand::AddRight(right);
        } else {
            let cmd = explode(n, layer + 1);

            if let NodeCommand::AddLeft(v) = cmd {
                match &mut node.left {
                    NodeValue::Literal(e) => node.left = NodeValue::Literal(v + *e),
                    NodeValue::Child(n) => incr_first_right(n, v),
                }

                return NodeCommand::Done;
            } else if cmd != NodeCommand::None {
                return cmd;
            }
        }
    };

    NodeCommand::None
}

fn incr_first_left(node: &mut Node, value: u32) {
    match &mut node.left {
        NodeValue::Literal(v) => node.left = NodeValue::Literal(value + *v),
        NodeValue::Child(n) => incr_first_left(n, value),
    }
}

fn incr_first_right(node: &mut Node, value: u32) {
    match &mut node.right {
        NodeValue::Literal(v) => node.right = NodeValue::Literal(value + *v),
        NodeValue::Child(n) => incr_first_right(n, value),
    }
}

fn parser_tuple(i: &str) -> IResult<&str, Node, ()> {
    let mapped_node = map(
        separated_pair(parser_value_or_tuple, char(','), parser_value_or_tuple),
        |(l, r)| Node { left: l, right: r },
    );

    delimited(char('['), mapped_node, char(']'))(i)
}

fn parser_value_or_tuple(i: &str) -> IResult<&str, NodeValue, ()> {
    let mapped_digit0 = map_opt(digit0, |s: &str| {
        s.parse::<u32>().map(NodeValue::Literal).ok()
    });

    let mapped_tuple = map(parser_tuple, |n| NodeValue::Child(Box::new(n)));

    alt((mapped_digit0, mapped_tuple))(i)
}

#[cfg(test)]
mod tests {
    use crate::node::*;
    use rstest::rstest;

    #[rstest]
    #[case("[[1,2],[[3,4],5]]")]
    #[case("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")]
    #[case("[[[[1,1],[2,2]],[3,3]],[4,4]]")]
    #[case("[[[[3,0],[5,3]],[4,4]],[5,5]]")]
    #[case("[[[[5,0],[7,4]],[5,5]],[6,6]]")]
    #[case("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")]
    fn parse_roundtrip(#[case] raw: &str) {
        assert_eq!(raw, Node::from(raw).to_string());
    }

    #[rstest]
    #[case("[[1,2],[[3,4],5]]", 143)]
    #[case("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384)]
    #[case("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445)]
    #[case("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791)]
    #[case("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137)]
    #[case("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488)]
    #[case("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]", 4140)]
    fn magnitude_check(#[case] raw: &str, #[case] mag: u32) {
        assert_eq!(mag, Node::from(raw).magnitude());
    }

    #[rstest]
    #[case(
        "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
    )]
    #[case(
        "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
    )]
    fn normalize_check(#[case] raw: &str, #[case] exp: &str) {
        let mut node = Node::from(raw);
        node.normalize();
        assert_eq!(exp, node.to_string());
    }

    #[rstest]
    #[case(
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
        [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
        [7,[5,[[3,8],[1,4]]]]
        [[2,[2,2]],[8,[8,1]]]
        [2,9]
        [1,[[[9,3],9],[[9,0],[0,7]]]]
        [[[5,[7,4]],7],1]
        [[[[4,2],2],6],[8,7]]",
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
    )]
    #[case(
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
    )]
    fn sum_check(#[case] inputs: &str, #[case] exp: &str) {
        let mut node: Option<Node> = None;

        for line in inputs.trim().lines().map(|l| l.trim()) {
            if let Some(n) = node {
                node = Some(n.add(Node::from(line)));
            } else {
                node = Some(Node::from(line));
            }
        }

        assert_eq!(exp, node.unwrap().to_string());
    }
}
