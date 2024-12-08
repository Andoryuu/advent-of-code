use num::Integer;

// We could use the iterator logic directly as a stateless function
// that creates and returns vec with values.
// But where is the fun in that. (Also, this runs faster.)

pub struct AntinodesIterator {
    node_a: (isize, isize),
    node_b: (isize, isize),
    size: (isize, isize),
    use_harmonics: bool,
    state: u32,
    ab_vec: (isize, isize),
    ba_vec: (isize, isize),
}

impl AntinodesIterator {
    pub fn new(
        node_a: (isize, isize),
        node_b: (isize, isize),
        size: (isize, isize),
        use_harmonics: bool,
    ) -> Self {
        AntinodesIterator {
            node_a,
            node_b,
            size,
            use_harmonics,
            state: 0,
            ab_vec: (node_a.0 - node_b.0, node_a.1 - node_b.1),
            ba_vec: (node_b.0 - node_a.0, node_b.1 - node_a.1),
        }
    }
}

impl Iterator for AntinodesIterator {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.state == 0 {
            self.state.inc();

            let node = checked_add_vecs(self.node_a, self.ab_vec, self.size);
            if node.is_some() {
                return node;
            }
        }

        if self.state == 1 {
            self.state.inc();

            let node = checked_add_vecs(self.node_b, self.ba_vec, self.size);
            if node.is_some() {
                return node;
            }
        }

        if self.state == 2 {
            if !self.use_harmonics {
                return None;
            }

            self.state.inc();
            return Some(self.node_a);
        }

        if self.state == 3 {
            self.state.inc();
            return Some(self.node_b);
        }

        if self.state == 4 {
            if let Some(next_node_a) = checked_add_vecs(self.node_a, self.ab_vec, self.size) {
                self.node_a = next_node_a;
                return Some(next_node_a);
            }
            self.state.inc();
        }

        if self.state == 5 {
            if let Some(next_node_b) = checked_add_vecs(self.node_b, self.ba_vec, self.size) {
                self.node_b = next_node_b;
                return Some(next_node_b);
            }
            self.state.inc();
        }

        None
    }
}

fn checked_add_vecs(
    (a_row, a_col): (isize, isize),
    (b_row, b_col): (isize, isize),
    (size_row, size_col): (isize, isize),
) -> Option<(isize, isize)> {
    let new_row = a_row + b_row;
    if !(0..size_row).contains(&new_row) {
        return None;
    }

    let new_col = a_col + b_col;
    if !(0..size_col).contains(&new_col) {
        return None;
    }

    Some((new_row, new_col))
}
