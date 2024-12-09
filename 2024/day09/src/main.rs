#![feature(test)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs, iter,
};

use auto_enums::auto_enum;
use itertools::{Either, Itertools};
use num::Integer;

fn main() {
    let input = fs::read_to_string("./day09/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let first_line = input.lines().find(|line| !line.is_empty()).unwrap();

    let mut disk = Vec::with_capacity(first_line.len() * 8);
    for (i, n) in first_line
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .enumerate()
        .map(|(i, n)| (if i.is_even() { Some(i / 2) } else { None }, n as usize))
    {
        // faster than flat_map
        for _ in 0..n {
            disk.push(i);
        }
    }

    let mut defraged_disk = vec![];
    let mut iter_disk = disk.into_iter();

    while let Some(block) = iter_disk.next() {
        // rfind continues from where it stopped last time
        if let Some(x) = block.or_else(|| iter_disk.rfind(|y| y.is_some()).flatten()) {
            defraged_disk.push(x);
        } else {
            break;
        }
    }

    defraged_disk
        .into_iter()
        .enumerate()
        .map(|(i, b)| b * i)
        .sum::<usize>()
        .to_string()
}

fn process_part_2(input: &str) -> String {
    let first_line = input.lines().find(|line| !line.is_empty()).unwrap();

    let mut disk = first_line
        .chars()
        .filter_map(|ch| ch.to_digit(10).map(|n| n as usize))
        .collect_vec();

    let files = disk
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(index, &size)| {
            if index.is_even() {
                Some((index, size))
            } else {
                None
            }
        })
        .collect_vec();

    let mut spaces: HashMap<_, BinaryHeap<_>> = HashMap::from_iter(
        disk.iter()
            .enumerate()
            .filter_map(|(index, &size)| {
                if index.is_odd() && size != 0 {
                    Some((size, Reverse(index)))
                } else {
                    None
                }
            })
            .into_group_map()
            .into_iter()
            .map(|(k, v)| (k, v.into())),
    );

    let mut moved: HashMap<usize, Vec<(_, _)>> = HashMap::new();
    let mut removed = HashSet::new();

    for (file_index, file_size) in files {
        if let Some((space_size, space_index)) = (file_size..=9)
            .filter_map(|size| {
                spaces
                    .get(&size)
                    .and_then(|v| v.peek())
                    .map(move |ix| (size, ix.0))
            })
            .k_smallest_by_key(1, |(_, ix)| *ix)
            .next()
        {
            if space_index > file_index {
                continue;
            }

            removed.insert(file_index);
            moved
                .entry(space_index)
                .and_modify(|e| e.push((file_index, file_size)))
                .or_insert_with(|| [(file_index, file_size)].into());

            let new_space_size = space_size - file_size;
            disk[space_index] = new_space_size;

            spaces.get_mut(&space_size).map(|v| v.pop());

            if new_space_size > 0 {
                spaces
                    .entry(new_space_size)
                    .and_modify(|new| {
                        new.push(Reverse(space_index));
                    })
                    .or_insert_with(|| [Reverse(space_index)].into());
            }
        }
    }

    disk.into_iter()
        .enumerate()
        .flat_map(|(index, size)| {
            if index.is_even() {
                if removed.contains(&index) {
                    inflate(Either::Left(None), size)
                } else {
                    inflate(Either::Left(Some(index / 2)), size)
                }
            } else if let Some(m) = moved.get(&index) {
                inflate(Either::Right(m), size)
            } else {
                inflate(Either::Left(None), size)
            }
        })
        .enumerate()
        .filter_map(|(i, b)| b.map(|b| b * i))
        .sum::<usize>()
        .to_string()
}

#[auto_enum(Iterator)]
fn inflate(
    value: Either<Option<usize>, &[(usize, usize)]>,
    count: usize,
) -> impl IntoIterator<Item = Option<usize>> + use<'_> {
    match value {
        Either::Left(value) => iter::repeat_n(value, count),
        Either::Right(moved) => iter::repeat((0usize, 0usize))
            .zip(moved)
            .flat_map(|((_, space_size), &(b_index, b_size))| {
                iter::repeat_n(None, space_size).chain(iter::repeat_n(Some(b_index / 2), b_size))
            })
            .chain(iter::repeat_n(None, count)),
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "2333133121414131402";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "1928")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "2858")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("6201130364722")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("6221662795602")]
    fn part_2_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(&input));
    }

    #[rstest]
    #[case("97898222299196")]
    fn part_m_ante(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input_mid.txt").expect("oh noes");
        assert_eq!(expected, process_part_2(&input));
    }

    #[rstest]
    #[case("5799706413896802")]
    fn part_h_ante(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input_hard.txt").expect("oh noes");
        assert_eq!(expected, process_part_2(&input));
    }

    #[bench]
    fn part_1_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_1(TEST_CASE));
    }

    #[bench]
    fn part_1_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_1(&input));
    }

    #[bench]
    fn part_2_check_bench(b: &mut Bencher) {
        b.iter(|| process_part_2(TEST_CASE));
    }

    #[bench]
    fn part_2_control_bench(b: &mut Bencher) {
        let input = input();
        b.iter(|| process_part_2(&input));
    }

    #[bench]
    fn part_m1_ante_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input_mid.txt").expect("oh noes");
        b.iter(|| process_part_1(&input));
    }

    #[bench]
    fn part_m2_ante_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input_mid.txt").expect("oh noes");
        b.iter(|| process_part_2(&input));
    }

    #[bench]
    fn part_h1_ante_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input_hard.txt").expect("oh noes");
        b.iter(|| process_part_1(&input));
    }

    #[bench]
    fn part_h2_ante_bench(b: &mut Bencher) {
        let input = fs::read_to_string("./_data/input_hard.txt").expect("oh noes");
        b.iter(|| process_part_2(&input));
    }
}
