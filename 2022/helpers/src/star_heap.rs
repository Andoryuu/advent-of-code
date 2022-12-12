use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
pub struct Node {
    pub id: usize,
    pub dist: u32,
    score: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct StarHeap<'a, F>
where
    F: Fn(usize) -> u32,
{
    heap: BinaryHeap<Node>,
    distances: Vec<u32>,
    scores: Vec<u32>,
    heuristic: &'a F,
}

impl<'a, F> StarHeap<'a, F>
where
    F: Fn(usize) -> u32,
{
    pub fn new(heuristic: &'a F) -> Self {
        StarHeap {
            heap: BinaryHeap::new(),
            distances: Vec::new(),
            scores: Vec::new(),
            heuristic,
        }
    }

    pub fn push(&mut self, id: usize, dist: u32) {
        if self.distances.len() <= id {
            self.distances.resize(id + 1, u32::MAX);
            self.scores.resize(id + 1, u32::MAX);
        }

        if dist < self.distances[id] {
            let score = dist + (self.heuristic)(id);

            self.distances[id] = dist;
            self.scores[id] = score;
            self.heap.push(Node { id, dist, score });
        }
    }

    pub fn pop(&mut self) -> Option<Node> {
        loop {
            if let Some(node) = self.heap.pop() {
                if node.score <= self.scores[node.id] {
                    return Some(node);
                }
            } else {
                return None;
            }
        }
    }
}
