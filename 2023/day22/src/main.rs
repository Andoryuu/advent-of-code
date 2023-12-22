#![feature(test)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let input = fs::read_to_string("./day22/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let bricks = parse(input);

    let CollapsedData {
        bottoms,
        tops,
        bricks_map,
        layed_bricks,
    } = collapse_bricks(bricks);

    layed_bricks
        .into_iter()
        .filter(|b_id| {
            let BrickData {
                bottom: _,
                top,
                points,
            } = bricks_map.get(b_id).unwrap();

            points
                .iter()
                .filter_map(|(x, y)| bottoms.get(&(*x, *y, top + 1)))
                .unique()
                .all(|id| {
                    bricks_map
                        .get(id)
                        .unwrap()
                        .points
                        .iter()
                        .filter_map(|(x, y)| tops.get(&(*x, *y, *top)))
                        .any(|x| x != b_id)
                })
        })
        .count()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let bricks = parse(input);

    let CollapsedData {
        bottoms,
        tops,
        bricks_map,
        layed_bricks,
    } = collapse_bricks(bricks);

    let bricks_graph = HashMap::<_, _>::from_iter(layed_bricks.iter().map(|id| {
        let BrickData {
            bottom,
            top,
            points,
        } = bricks_map.get(id).unwrap();

        (
            *id,
            (
                points
                    .iter()
                    .filter_map(|(x, y)| tops.get(&(*x, *y, bottom - 1)))
                    .unique()
                    .copied()
                    .collect_vec(),
                points
                    .iter()
                    .filter_map(|(x, y)| bottoms.get(&(*x, *y, top + 1)))
                    .unique()
                    .copied()
                    .collect_vec(),
            ),
        )
    }));

    layed_bricks
        .into_par_iter()
        .map(|b_id| {
            let (_, b_above) = bricks_graph.get(&b_id).unwrap();
            let mut fallen = HashSet::new();
            let mut queue = VecDeque::new();
            let mut bench = vec![];

            fallen.insert(b_id);
            b_above.iter().for_each(|x| queue.push_back(*x));

            loop {
                let mut new_fallen = false;

                while let Some(id) = queue.pop_front() {
                    let (under, above) = bricks_graph.get(&id).unwrap();

                    if under.iter().all(|u_id| fallen.contains(u_id)) {
                        new_fallen = fallen.insert(id) || new_fallen;
                        above.iter().for_each(|x| queue.push_back(*x));
                    } else {
                        bench.push(id);
                    }
                }

                if !new_fallen {
                    break;
                }

                bench.iter().for_each(|x| queue.push_back(*x));
                bench.clear();
            }

            fallen.len() - 1
        })
        .sum::<usize>()
        .to_string()
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(['~', ','])
                .map(|p| p.parse::<u32>().unwrap())
                .tuples::<(_, _, _)>()
                .next_tuple::<(_, _)>()
                .unwrap()
        })
        .sorted_by_key(|((_, _, z), _)| *z)
        .collect_vec()
}

type Coord = (u32, u32, u32);
type Brick = (Coord, Coord);

struct BrickData {
    bottom: u32,
    top: u32,
    points: Vec<(u32, u32)>,
}

struct CollapsedData {
    bottoms: HashMap<Coord, usize>,
    tops: HashMap<Coord, usize>,
    bricks_map: HashMap<usize, BrickData>,
    layed_bricks: Vec<usize>,
}

fn collapse_bricks(bricks: Vec<Brick>) -> CollapsedData {
    let mut bottoms = HashMap::new();
    let mut tops = HashMap::new();
    let mut bricks_map = HashMap::new();
    let mut layed_bricks = vec![];

    let mut maxes = HashMap::new();

    for (id, ((x1, y1, z1), (x2, y2, z2))) in bricks.into_iter().enumerate() {
        let points = (x1..=x2)
            .flat_map(|x| (y1..=y2).map(move |y| (x, y)))
            .collect_vec();

        let top = points
            .iter()
            .map(|p| *maxes.get(p).unwrap_or(&0))
            .max()
            .unwrap();

        let new_bottom = top + 1;
        let new_top = new_bottom + z2 - z1;

        for p in points.iter() {
            maxes
                .entry(*p)
                .and_modify(|z| *z = new_top)
                .or_insert(new_top);
        }

        layed_bricks.push(id);
        bricks_map.insert(
            id,
            BrickData {
                bottom: new_bottom,
                top: new_top,
                points: points.clone(),
            },
        );

        for (x, y) in points {
            bottoms.insert((x, y, new_bottom), id);
            tops.insert((x, y, new_top), id);
        }
    }

    CollapsedData {
        bottoms,
        tops,
        bricks_map,
        layed_bricks,
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[rstest]
    #[case(TEST_CASE, "5")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "7")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("475")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("79144")]
    fn part_2_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_2(&input));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_1(&input));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        b.iter(|| process_part_2(&input));
    }
}
