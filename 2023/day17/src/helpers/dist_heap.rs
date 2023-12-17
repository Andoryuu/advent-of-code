use std::collections::BinaryHeap;

use super::{direction::Direction, node::Node};

pub struct DistHeap {
    heap: BinaryHeap<Node>,
    dists: Vec<Vec<Vec<u32>>>,
}

impl DistHeap {
    pub fn new(size: usize) -> DistHeap {
        let mut heap = DistHeap {
            heap: BinaryHeap::new(),
            dists: vec![vec![vec![u32::MAX; 11]; 4]; size],
        };

        heap.push(Node {
            index: 0,
            dist: 0,
            dir: Direction::Right,
            straight: 0,
        });

        heap.push(Node {
            index: 0,
            dist: 0,
            dir: Direction::Down,
            straight: 0,
        });

        heap
    }

    pub fn push(&mut self, node: Node) {
        let dist = self.dists[node.index][node.dir as usize]
            .get_mut(node.straight as usize)
            .unwrap();

        if *dist > node.dist {
            *dist = node.dist;
            self.heap.push(node);
        }
    }

    pub fn pop(&mut self) -> Option<Node> {
        self.heap.pop()
    }
}
