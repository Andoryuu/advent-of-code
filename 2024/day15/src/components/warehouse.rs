use fxhash::FxHashMap;
use itertools::Itertools;

use super::{field::Field, r#move::Move, robot::Robot, tile::Tile};

#[derive(Debug)]
pub struct Warehouse {
    robot: Robot,
    moves: Vec<Move>,
    field: Field,
}

impl Warehouse {
    pub fn new(input: &str, is_wide: bool) -> Self {
        let col_mult = if is_wide { 2 } else { 1 };

        let mut lines_iter = input.lines().skip_while(|line| line.is_empty());
        let lines_iter = lines_iter.by_ref();

        let mut robot = None;
        let mut field = FxHashMap::default();

        for ((row, col), ch) in lines_iter
            .take_while(|line| !line.is_empty())
            .enumerate()
            .flat_map(|(row, line)| {
                line.char_indices()
                    .map(move |(col, ch)| ((row as isize, col as isize), ch))
            })
        {
            match ch {
                '#' => {
                    field.insert((row, col * col_mult), Tile::Wall);
                }
                'O' => {
                    field.insert((row, col * col_mult), Tile::Box);
                }
                '@' => {
                    robot = Some((row, col * col_mult));
                }
                _ => {}
            }
        }

        let moves = lines_iter
            .into_iter()
            .flat_map(|line| line.chars())
            .filter_map(|ch| Move::try_from(ch).ok())
            .collect_vec();

        Warehouse {
            robot: Robot::new(robot.unwrap()),
            moves,
            field: if is_wide {
                Field::Wide(field)
            } else {
                Field::Normal(field)
            },
        }
    }

    pub fn simulate(self) -> isize {
        let Warehouse {
            mut robot,
            moves,
            mut field,
        } = self;

        for mv in moves {
            robot.apply_move(mv, &mut field);
        }

        field.sum_boxes()
    }
}
