use std::collections::{VecDeque, BTreeSet};

pub struct DedupQueue<T> {
    queue: VecDeque<T>,
    checker: BTreeSet<T>,
}

impl<T: std::cmp::Ord + Copy> DedupQueue<T> {
    pub fn new() -> Self {
        DedupQueue {
            queue: VecDeque::new(),
            checker: BTreeSet::new(),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(val) = self.queue.pop_front() {
            self.checker.remove(&val);

            Some(val)
        } else {
            None
        }
    }

    pub fn push_front(&mut self, val: T) {
        if self.checker.contains(&val) {
            return;
        }

        self.queue.push_front(val);
        self.checker.insert(val);
    }

    pub fn push_back(&mut self, val: T) {
        if self.checker.contains(&val) {
            return;
        }

        self.queue.push_back(val);
        self.checker.insert(val);
    }
}
