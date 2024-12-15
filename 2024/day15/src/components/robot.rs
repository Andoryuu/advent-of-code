use std::collections::VecDeque;

use auto_enums::auto_enum;
use fxhash::{FxHashMap, FxHashSet};

use super::{field::Field, r#move::Move, tile::Tile};

#[derive(Debug)]
pub struct Robot {
    position: (isize, isize),
}

impl Robot {
    pub fn new(position: (isize, isize)) -> Self {
        Robot { position }
    }

    pub fn apply_move(&mut self, mv: Move, field: &mut Field) {
        match field {
            Field::Normal(field) => apply_normal_move(self, mv, field),
            Field::Wide(field) => apply_wide_move(self, mv, field),
        }
    }
}

fn apply_normal_move(robot: &mut Robot, mv: Move, field: &mut FxHashMap<(isize, isize), Tile>) {
    let new_pos = mv.apply_to(robot.position);

    let (end_pos, end_tile) = get_line(new_pos, mv)
        .into_iter()
        .find_map(|pos| try_get_from(pos, field))
        .unwrap();

    if end_tile == Tile::Wall {
        return;
    }

    if end_pos.0 != new_pos.0 || end_pos.1 != new_pos.1 {
        field.remove(&new_pos);
        field.insert(end_pos, Tile::Box);
    }

    robot.position = new_pos;
}

fn apply_wide_move(robot: &mut Robot, mv: Move, field: &mut FxHashMap<(isize, isize), Tile>) {
    let new_pos = mv.apply_to(robot.position);

    if let Some((&t_coord, tile)) = field
        .get_key_value(&new_pos)
        .or_else(|| field.get_key_value(&(new_pos.0, new_pos.1 - 1)))
    {
        if tile == &Tile::Wall {
            return;
        }

        let mut queue = VecDeque::from([t_coord]);
        let mut touched_boxes = FxHashSet::default();
        touched_boxes.insert(t_coord);

        while let Some(pos) = queue.pop_front() {
            for pos in get_candidates(pos, mv) {
                if !try_add_to(pos, field, &mut touched_boxes, &mut queue) {
                    return;
                }
            }
        }

        for box_pos in touched_boxes.iter() {
            field.remove(box_pos);
        }

        for box_pos in touched_boxes {
            field.insert(mv.apply_to(box_pos), Tile::Box);
        }
    }

    robot.position = new_pos;
}

fn try_get_from(
    (row, col): (isize, isize),
    field: &FxHashMap<(isize, isize), Tile>,
) -> Option<((isize, isize), Tile)> {
    if let Some(tile) = field.get(&(row, col)) {
        match tile {
            Tile::None => unreachable!(),
            Tile::Box => None,
            Tile::Wall => Some(((row, col), Tile::Wall)),
        }
    } else {
        Some(((row, col), Tile::None))
    }
}

#[auto_enum(Iterator)]
fn get_line((row, col): (isize, isize), mv: Move) -> impl IntoIterator<Item = (isize, isize)> {
    match mv {
        Move::Up => (0..=row).rev().map(move |r| (r, col)),
        Move::Down => (row..).map(move |r| (r, col)),
        Move::Left => (0..=col).rev().map(move |c| (row, c)),
        Move::Right => (col..).map(move |c| (row, c)),
    }
}

fn try_add_to(
    pos: (isize, isize),
    field: &FxHashMap<(isize, isize), Tile>,
    touched: &mut FxHashSet<(isize, isize)>,
    queue: &mut VecDeque<(isize, isize)>,
) -> bool {
    if let Some(tile) = field.get(&pos) {
        if tile == &Tile::Wall {
            return false;
        } else if !touched.contains(&pos) {
            queue.push_back(pos);
            touched.insert(pos);
        }
    }
    true
}

#[auto_enum(Iterator)]
fn get_candidates(
    (row, col): (isize, isize),
    mv: Move,
) -> impl IntoIterator<Item = (isize, isize)> {
    match mv {
        Move::Up | Move::Down => {
            (-1..=1).map(move |offset| (row + if mv == Move::Up { -1 } else { 1 }, col + offset))
        }
        Move::Left | Move::Right => {
            (0isize..=0isize).map(move |_| (row, col + if mv == Move::Left { -2 } else { 2 }))
        }
    }
}
