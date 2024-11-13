#![feature(test)]

use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day25/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
}

fn process_part_1(input: &str) -> String {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut set = HashSet::new();

    for line in input.trim().lines() {
        let (fst, rest) = line.split(':').next_tuple().unwrap();

        for r in rest.trim().split_ascii_whitespace() {
            map.entry(fst.to_owned())
                .and_modify(|v| {
                    v.insert(r.to_owned());
                })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(r.to_owned());
                    set
                });

            map.entry(r.to_owned())
                .and_modify(|v| {
                    v.insert(fst.to_owned());
                })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(fst.to_owned());
                    set
                });

            set.insert(if fst < r {
                (fst.to_owned(), r.to_owned())
            } else {
                (r.to_owned(), fst.to_owned())
            });
        }
    }

    todo!()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::rstest;
    use test::Bencher;

    const TEST_CASE: &str = "
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[rstest]
    #[case(TEST_CASE, "54")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case("")]
    fn part_1_control(#[case] expected: &str) {
        let input = fs::read_to_string("./_data/input.txt").expect("oh noes");
        assert_eq!(expected, process_part_1(&input));
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
}
