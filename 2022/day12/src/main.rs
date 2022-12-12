#![feature(test)]

use std::fs;

use helpers::star_heap::{Node, StarHeap};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let scan = Scan::new(input);

    let end_row = scan.end / scan.size;
    let end_col = scan.end % scan.size;
    let size = scan.size;
    let heuristic = |ix: usize| {
        let row = ix / size;
        let col = ix % size;

        // use taxicab distance as heuristic
        (end_row.abs_diff(row) + end_col.abs_diff(col)) as u32
    };

    let mut star_heap = StarHeap::new(&heuristic);

    star_heap.push(scan.start, 0);

    while let Some(Node { id, dist, .. }) = star_heap.pop() {
        if id == scan.end {
            return dist.to_string();
        }

        let height_limit = scan.area[id] + 1;

        for (neigh_id, neigh_height) in scan.get_neighbours(id) {
            if height_limit < neigh_height {
                continue;
            }

            star_heap.push(neigh_id, dist + 1);
        }
    }

    "no path found".to_owned()
}

fn process_data_adv(input: String) -> String {
    let scan = Scan::new(input);

    // no heuristic for this one
    let heuristic = |_: usize| 0;

    let mut star_heap = StarHeap::new(&heuristic);

    star_heap.push(scan.end, 0);

    while let Some(Node { id, dist, .. }) = star_heap.pop() {
        let height = scan.area[id];

        if height == 0 {
            return dist.to_string();
        }

        let height_limit = height - 1;

        for (neigh_id, neigh_height) in scan.get_neighbours(id) {
            if height_limit > neigh_height {
                continue;
            }

            star_heap.push(neigh_id, dist + 1);
        }
    }

    "no path found".to_owned()
}

struct Scan {
    size: usize,
    area: Vec<u32>,
    start: usize,
    end: usize,
}

impl Scan {
    fn new(data: String) -> Self {
        const BASE: u32 = 'a' as u32;

        let area = data
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect_vec();

        let (start, _) = area.iter().find_position(|c| **c == 'S').unwrap();
        let (end, _) = area.iter().find_position(|c| **c == 'E').unwrap();

        Scan {
            size: data.lines().next().unwrap().len(),
            area: area
                .iter()
                .map(|&c| match c {
                    'S' => 0,
                    'E' => 25,
                    x => (x as u32) - BASE,
                })
                .collect_vec(),
            start,
            end,
        }
    }

    fn get_neighbours(&self, from: usize) -> Vec<(usize, u32)> {
        let mut neighs = vec![];

        if from >= self.size {
            let ix = from - self.size;
            if let Some(val) = self.area.get(ix) {
                neighs.push((ix, *val));
            }
        }

        if (from + 1) % self.size != 0 {
            let ix = from + 1;
            if let Some(val) = self.area.get(ix) {
                neighs.push((ix, *val));
            }
        }

        if from < (self.area.len() - self.size) {
            let ix = from + self.size;
            if let Some(val) = self.area.get(ix) {
                neighs.push((ix, *val));
            }
        }

        if from % self.size != 0 {
            let ix = from - 1;
            if let Some(val) = self.area.get(ix) {
                neighs.push((ix, *val));
            }
        }

        neighs
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[rstest]
    #[case(TEST_CASE, "31")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned()));
    }

    #[rstest]
    #[case(TEST_CASE, "29")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned()));
    }

    #[bench]
    fn base_bench(b: &mut Bencher) {
        b.iter(|| process_data(TEST_CASE.to_owned()));
    }
}
