#![feature(test)]

use std::fs;

use cached::proc_macro::cached;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./day23/_data/input.txt").expect("oh noes");

    println!("Part 1 result is: {}", process_part_1(&input));
    println!("Part 2 result is: {}", process_part_2(&input));
}

fn process_part_1(input: &str) -> String {
    let computers = parse(input);

    let mut groups = FxHashSet::default();
    for (cpu, conns) in computers.iter() {
        if !cpu.starts_with('t') {
            continue;
        }

        for cpu2 in conns {
            for cpu3 in computers.get(cpu2).unwrap().intersection(conns) {
                let mut group = [cpu, cpu2, cpu3];
                group.sort();
                groups.insert(group);
            }
        }
    }

    groups.len().to_string()
}

fn process_part_2(input: &str) -> String {
    let computers = parse(input);

    bron_kerbosch(
        FxHashSet::default(),
        computers.keys().cloned().collect(),
        FxHashSet::default(),
        &computers,
    )
    .join(",")
}

fn parse(input: &str) -> FxHashMap<&str, FxHashSet<&str>> {
    let mut computers: FxHashMap<_, FxHashSet<_>> = FxHashMap::default();
    for (cpu1, cpu2) in input.lines().filter_map(|line| line.split_once('-')) {
        computers
            .entry(cpu1)
            .and_modify(|s| {
                s.insert(cpu2);
            })
            .or_insert_with(|| FxHashSet::from_iter([cpu2]));

        computers
            .entry(cpu2)
            .and_modify(|s| {
                s.insert(cpu1);
            })
            .or_insert_with(|| FxHashSet::from_iter([cpu1]));
    }
    computers
}

#[cached(
    key = "(String, String, String, String)",
    convert = r#"{(format!("{r:?}"), format!("{p:?}"), format!("{x:?}"), format!("{c:p}"))}"#
)]
fn bron_kerbosch<'a>(
    r: FxHashSet<&'a str>,
    mut p: FxHashSet<&'a str>,
    mut x: FxHashSet<&'a str>,
    c: &FxHashMap<&'a str, FxHashSet<&'a str>>,
) -> Vec<String> {
    if p.is_empty() && x.is_empty() {
        return r.into_iter().sorted().map(|s| s.to_owned()).collect_vec();
    }

    let mut max = vec![];
    for v in p.clone() {
        let n = c.get(v).unwrap();
        let mut r = r.clone();
        r.insert(v);

        let res = bron_kerbosch(
            r,
            p.intersection(n).cloned().collect(),
            x.intersection(n).cloned().collect(),
            c,
        );

        if max.len() < res.len() {
            max = res;
        }

        p.remove(v);
        x.insert(v);
    }

    max
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::*;
    use rstest::{fixture, rstest};
    use test::Bencher;

    const TEST_CASE: &str = "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[fixture]
    fn input() -> String {
        fs::read_to_string("./_data/input.txt").expect("oh noes")
    }

    #[rstest]
    #[case(TEST_CASE, "7")]
    fn part_1_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(input));
    }

    #[rstest]
    #[case(TEST_CASE, "co,de,ka,ta")]
    fn part_2_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_part_2(input));
    }

    #[rstest]
    #[case("1151")]
    fn part_1_control(input: String, #[case] expected: &str) {
        assert_eq!(expected, process_part_1(&input));
    }

    #[rstest]
    #[case("ar,cd,hl,iw,jm,ku,qo,rz,vo,xe,xm,xv,ys")]
    fn part_2_control(input: String, #[case] expected: &str) {
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
}
