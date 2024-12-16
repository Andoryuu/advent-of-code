use std::collections::{BinaryHeap, VecDeque};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use nalgebra::Point2;

use super::{direction::Direction, score::Score};

pub struct Maze {
    start: Point2<isize>,
    end: Point2<isize>,
    walls: FxHashSet<Point2<isize>>,
}

impl Maze {
    pub fn new(input: &str) -> Self {
        let mut start = None;
        let mut end = None;
        let mut walls = FxHashSet::default();

        for (coords, ch) in input
            .lines()
            .filter(|line| !line.is_empty())
            .enumerate()
            .flat_map(|(row, line)| {
                line.char_indices()
                    .map(move |(col, ch)| (Point2::new(row as isize, col as isize), ch))
            })
        {
            match ch {
                '#' => {
                    walls.insert(coords);
                }
                'S' => {
                    start = Some(coords);
                }
                'E' => {
                    end = Some(coords);
                }
                _ => {}
            }
        }

        Maze {
            start: start.unwrap(),
            end: end.unwrap(),
            walls,
        }
    }

    pub fn get_lowest_score(self) -> (isize, FxHashMap<(Point2<isize>, Direction), isize>) {
        let mut lowest = FxHashMap::default();
        let mut queue = BinaryHeap::from([Score {
            score: 0,
            position: self.start,
            direction: Direction::Right,
        }]);

        let mut end_score = None;

        while let Some(Score {
            score,
            position,
            direction,
        }) = queue.pop()
        {
            if end_score.is_some_and(|s| s < score) {
                continue;
            }

            if position == self.end {
                end_score = Some(score);
                continue;
            }

            if self.walls.contains(&position) {
                continue;
            }

            let lowest_key = (position, direction);
            if let Some(lowest_score) = lowest.get(&lowest_key) {
                if lowest_score <= &score {
                    continue;
                }
            }
            lowest.insert(lowest_key, score);

            queue.push(Score {
                score: score + 1,
                position: direction.apply_to(position),
                direction,
            });

            let starside_dir = direction.turn_clock();
            queue.push(Score {
                score: score + 1001,
                position: starside_dir.apply_to(position),
                direction: starside_dir,
            });

            let portside_dir = direction.turn_anticlock();
            queue.push(Score {
                score: score + 1001,
                position: portside_dir.apply_to(position),
                direction: portside_dir,
            });
        }

        (end_score.unwrap(), lowest)
    }

    pub fn count_best_spots(self) -> usize {
        let maze_end = self.end;
        let (end_score, traversed) = self.get_lowest_score();

        let traversed = traversed
            .into_iter()
            .map(|((p, d), s)| (p, (d, s)))
            .into_group_map();

        let mut spots = FxHashSet::default();
        let mut queue = VecDeque::from([
            (maze_end, Direction::Up, end_score),
            (maze_end, Direction::Down, end_score),
            (maze_end, Direction::Left, end_score),
            (maze_end, Direction::Right, end_score),
        ]);

        while let Some((pos, dir, score)) = queue.pop_front() {
            spots.insert(pos);

            let op_dir = dir.turn_opposite();
            let pos = op_dir.apply_to(pos);

            if let Some(dirs) = traversed.get(&pos) {
                for &(d, s) in dirs {
                    if d == op_dir {
                        continue;
                    }

                    if (d == dir && score - 1 == s) || (d != dir && score - 1001 == s) {
                        queue.push_back((pos, d, s));
                    }
                }
            }
        }

        spots.len()
    }
}
