#![feature(test)]

mod helpers;

use std::fs;

use itertools::Itertools;

use crate::helpers::{dist_heap::DistHeap, node::Node};

fn main() {
    let input = fs::read_to_string("./day17/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let (width, height, city) = parse(input);
    let dest = city.len() - 1;
    let mut heap = DistHeap::new(city.len());

    while let Some(Node {
        index,
        dist,
        dir,
        straight,
    }) = heap.pop()
    {
        if index == dest {
            return dist.to_string();
        }

        for next_dir in dir.get_next_short(straight) {
            if let Some(next_index) = next_dir.get_next_index(index, width, height) {
                heap.push(Node {
                    index: next_index,
                    dist: dist + city[next_index],
                    dir: next_dir,
                    straight: if dir == next_dir { straight + 1 } else { 1 },
                });
            }
        }
    }

    panic!("path not found")
}

fn process_part_2(input: &str) -> String {
    let (width, height, city) = parse(input);
    let dest = city.len() - 1;
    let mut heap = DistHeap::new(city.len());

    while let Some(Node {
        index,
        dist,
        dir,
        straight,
    }) = heap.pop()
    {
        if index == dest && straight >= 4 {
            return dist.to_string();
        }

        for next_dir in dir.get_next_long(straight) {
            if let Some(next_index) = next_dir.get_next_index(index, width, height) {
                heap.push(Node {
                    index: next_index,
                    dist: dist + city[next_index],
                    dir: next_dir,
                    straight: if dir == next_dir { straight + 1 } else { 1 },
                });
            }
        }
    }

    panic!("path not found")
}

fn parse(input: &str) -> (usize, usize, Vec<u32>) {
    let input = input.trim();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let city = input.chars().filter_map(|c| c.to_digit(10)).collect_vec();

    (width, height, city)
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const TEST_CASE_2: &str = "
111111111111
999999999991
999999999991
999999999991
999999999991";

    #[rstest]
    #[case(TEST_CASE, "102")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "94")]
    #[case(TEST_CASE_2, "71")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("959")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("1135")]
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
