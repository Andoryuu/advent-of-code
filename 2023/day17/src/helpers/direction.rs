#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    pub fn get_next_short(&self, straight: u32) -> Vec<Direction> {
        let mut res = match self {
            Direction::Up => vec![Direction::Left, Direction::Right],
            Direction::Left => vec![Direction::Up, Direction::Down],
            Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Right => vec![Direction::Up, Direction::Down],
        };

        if straight < 3 {
            res.push(*self);
        }

        res
    }

    pub fn get_next_long(&self, straight: u32) -> Vec<Direction> {
        let mut res = if straight >= 4 {
            match self {
                Direction::Up => vec![Direction::Left, Direction::Right],
                Direction::Left => vec![Direction::Up, Direction::Down],
                Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Right => vec![Direction::Up, Direction::Down],
            }
        } else {
            vec![]
        };

        if straight < 10 {
            res.push(*self);
        }

        res
    }

    pub fn get_next_index(&self, index: usize, width: usize, height: usize) -> Option<usize> {
        match self {
            Direction::Up => {
                if index >= width {
                    Some(index - width)
                } else {
                    None
                }
            }
            Direction::Left => {
                if (index % width) > 0 {
                    Some(index - 1)
                } else {
                    None
                }
            }
            Direction::Down => {
                if index < (width * (height - 1)) {
                    Some(index + width)
                } else {
                    None
                }
            }
            Direction::Right => {
                if (index % width) < (width - 1) {
                    Some(index + 1)
                } else {
                    None
                }
            }
        }
    }
}
