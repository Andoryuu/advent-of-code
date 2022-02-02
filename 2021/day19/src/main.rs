use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

use itertools::Itertools;
use nom::{
    character::complete::{char, digit0},
    combinator::{map_opt, opt},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let lines = input
        .trim()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(parser_line)
        .flatten();

    let mut areas = Vec::new();
    let mut last_col: Option<Vec<Vec<i32>>> = None;

    for (s, v) in lines {
        if s.is_empty() {
            if let Some(col) = &mut last_col {
                col.push(v);
            }
        } else {
            if let Some(col) = last_col {
                areas.push(col);
            }

            last_col = Some(Vec::new());
        }
    }

    if let Some(col) = last_col {
        areas.push(col);
    }

    let mut probes_per_area: Vec<BTreeMap<usize, (Vec<i32>, BTreeSet<u64>)>> = Vec::new();
    let mut distance_sets_per_area: Vec<BTreeSet<u64>> = Vec::new();

    for area in areas.iter() {
        let mut probes_map = BTreeMap::new();
        let mut distance_set = BTreeSet::new();

        for probes in area.iter().enumerate().combinations(2) {
            let (i1, v1) = probes.first().unwrap();
            let (i2, v2) = probes.last().unwrap();
            let dist = get_distance(get_diff(v1, v2));

            let (_, v1_entry) = probes_map
                .entry(*i1)
                .or_insert(((*v1).clone(), BTreeSet::new()));

            v1_entry.insert(dist);

            let (_, v2_entry) = probes_map
                .entry(*i2)
                .or_insert(((*v2).clone(), BTreeSet::new()));

            v2_entry.insert(dist);

            distance_set.insert(dist);
        }

        probes_per_area.push(probes_map);
        distance_sets_per_area.push(distance_set);
    }

    let mut to_process = BTreeSet::from_iter(1..areas.len());
    let mut all_probes = probes_per_area.first().unwrap().to_owned();
    let mut all_distances = distance_sets_per_area.first().unwrap().to_owned();

    while !to_process.is_empty() {
        let mut distance_data: Vec<(usize, usize, Vec<u64>)> = distance_sets_per_area
            .iter()
            .enumerate()
            .filter(|(i, _)| to_process.contains(i))
            .map(|(i, a)| {
                (
                    i,
                    all_distances.intersection(a).cloned().collect::<Vec<u64>>(),
                )
            })
            .map(|(i, inter)| (inter.len(), i, inter))
            .collect();

        distance_data.sort_by(|a, b| a.0.cmp(&b.0));

        let (_, used_i, common_dists) = distance_data.last().unwrap();

        to_process.remove(used_i);

        let common_set = BTreeSet::from_iter(common_dists.iter().cloned());
        let new_probes = probes_per_area.get(*used_i).unwrap();

        for (_, (new_coords, new_probe)) in new_probes.iter() {
            let trimmed_new_probe =
                BTreeSet::from_iter(new_probe.intersection(&common_set).into_iter().cloned());

            if trimmed_new_probe.len() < 10 {
                continue;
            }

            let matched = all_probes
                .iter()
                .filter(|(_, (_, probe))| {
                    probe.intersection(&trimmed_new_probe).count() == trimmed_new_probe.len()
                })
                .map(|(i, _)| *i)
                .next()
                .unwrap();

            let (existing_coords, existing_dists) = all_probes.get_mut(&matched).unwrap();

            for dist in new_probe.iter() {
                existing_dists.insert(*dist);
            }



            // if let Some(matched) = has_match {
            // } else {
            //     let new_i = all_probes.len();

            //     all_probes.insert(new_i, new_probe.clone());
            // }

            // for dist in new_probe.iter() {
            //     all_distances.insert(*dist);
            // }
        }
    }

    all_probes.len().to_string()
}

fn process_data_adv(input: String) -> String {
    "".to_string()
}

fn get_distance(vector: Vec<i32>) -> u64 {
    vector.iter().map(|v| (v * v) as u64).sum()
}

fn get_diff(vec1: &[i32], vec2: &[i32]) -> Vec<i32> {
    vec1.iter().zip(vec2).map(|(v1, v2)| v1 - v2).collect()
}

fn parser_line(i: &str) -> IResult<&str, Vec<i32>, ()> {
    let negative_digit0 = map_opt(
        tuple((opt(char('-')), digit0)),
        |(opt, dig): (Option<char>, &str)| {
            dig.parse::<i32>()
                .map(|d| if opt.is_some() { -d } else { d })
                .ok()
        },
    );

    separated_list0(char(','), negative_digit0)(i)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "--- scanner 0 ---
    404,-588,-901
    528,-643,409
    -838,591,734
    390,-675,-793
    -537,-823,-458
    -485,-357,347
    -345,-311,381
    -661,-816,-575
    -876,649,763
    -618,-824,-621
    553,345,-567
    474,580,667
    -447,-329,318
    -584,868,-557
    544,-627,-890
    564,392,-477
    455,729,728
    -892,524,684
    -689,845,-530
    423,-701,434
    7,-33,-71
    630,319,-379
    443,580,662
    -789,900,-551
    459,-707,401

    --- scanner 1 ---
    686,422,578
    605,423,415
    515,917,-361
    -336,658,858
    95,138,22
    -476,619,847
    -340,-569,-846
    567,-361,727
    -460,603,-452
    669,-402,600
    729,430,532
    -500,-761,534
    -322,571,750
    -466,-666,-811
    -429,-592,574
    -355,545,-477
    703,-491,-529
    -328,-685,520
    413,935,-424
    -391,539,-444
    586,-435,557
    -364,-763,-893
    807,-499,-711
    755,-354,-619
    553,889,-390

    --- scanner 2 ---
    649,640,665
    682,-795,504
    -784,533,-524
    -644,584,-595
    -588,-843,648
    -30,6,44
    -674,560,763
    500,723,-460
    609,671,-379
    -555,-800,653
    -675,-892,-343
    697,-426,-610
    578,704,681
    493,664,-388
    -671,-858,530
    -667,343,800
    571,-461,-707
    -138,-166,112
    -889,563,-600
    646,-828,498
    640,759,510
    -630,509,768
    -681,-892,-333
    673,-379,-804
    -742,-814,-386
    577,-820,562

    --- scanner 3 ---
    -589,542,597
    605,-692,669
    -500,565,-823
    -660,373,557
    -458,-679,-417
    -488,449,543
    -626,468,-788
    338,-750,-386
    528,-832,-391
    562,-778,733
    -938,-730,414
    543,643,-506
    -524,371,-870
    407,773,750
    -104,29,83
    378,-903,-323
    -778,-728,485
    426,699,580
    -438,-605,-362
    -469,-447,-387
    509,732,623
    647,635,-688
    -868,-804,481
    614,-800,639
    595,780,-596

    --- scanner 4 ---
    727,592,562
    -293,-554,779
    441,611,-461
    -714,465,-776
    -743,427,-804
    -660,-479,-426
    832,-632,460
    927,-485,-438
    408,393,-506
    466,436,-512
    110,16,151
    -258,-428,682
    -393,719,612
    -211,-452,876
    808,-476,-593
    -575,615,604
    -485,667,467
    -680,325,-822
    -627,-443,-432
    872,-547,-609
    833,512,582
    807,604,487
    839,-516,451
    891,-625,532
    -652,-548,-490
    30,-46,-14
    ";

    #[test]
    fn base_check() {
        assert_eq!("79", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("3621", process_data_adv(TEST_CASE.to_string()));
    }
}
