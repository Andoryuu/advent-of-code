use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs,
};

use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let base_valve = "AA".to_owned();

    let valves = parse(input);
    let distances = get_distances(&valves);
    let candidates = BTreeSet::from_iter(
        valves
            .iter()
            .filter(|(k, v)| *k != &base_valve && v.flow > 0)
            .map(|(k, _)| k.to_owned()),
    );
    let start_valve = valves.get(&base_valve).unwrap();
    let mut results = BTreeMap::new();
    results.insert((base_valve.clone(), 30), 0);

    analyze(
        &valves,
        &distances,
        candidates,
        &mut results,
        base_valve,
        start_valve,
        30,
    );

    results.values().max().unwrap().to_string()
}

fn process_data_adv(input: String) -> String {
    let base_valve = "AA".to_owned();

    let valves = parse(input);
    let distances = get_distances(&valves);
    let candidates = BTreeSet::from_iter(
        valves
            .iter()
            .filter(|(k, v)| *k != &base_valve && v.flow > 0)
            .map(|(k, _)| k.to_owned()),
    );
    let start_valve = valves.get(&base_valve).unwrap();

    let res = analyze_in_pair(
        &valves,
        &distances,
        candidates,
        start_valve,
        start_valve,
        26,
        26,
    );

    res.to_string()
}

fn analyze(
    valves: &BTreeMap<String, Valve>,
    dists: &BTreeMap<String, BTreeMap<String, u32>>,
    candidates: BTreeSet<String>,
    results: &mut BTreeMap<(String, u32), u32>,
    path: String,
    current: &Valve,
    time_rem: u32,
) {
    let cur_dists = dists.get(&current.name).unwrap();
    let base_val = *results.get(&(path.clone(), time_rem)).unwrap();

    for can in candidates.iter() {
        let dist = cur_dists.get(can).unwrap();
        if (dist + 1) > time_rem {
            continue;
        }

        let prefix = path.clone() + can;
        let after_can_open = time_rem - dist - 1;

        if results.contains_key(&(prefix.clone(), after_can_open)) {
            continue;
        }

        let new_current = valves.get(can).unwrap();
        let can_val = base_val + (new_current.flow * after_can_open);

        results.insert((prefix.clone(), after_can_open), can_val);

        let mut new_cands = candidates.clone();
        new_cands.remove(can);

        analyze(
            valves,
            dists,
            new_cands,
            results,
            prefix,
            new_current,
            after_can_open,
        );
    }
}

fn analyze_in_pair(
    valves: &BTreeMap<String, Valve>,
    dists: &BTreeMap<String, BTreeMap<String, u32>>,
    candidates: BTreeSet<String>,
    current_me: &Valve,
    current_ot: &Valve,
    time_rem_me: u32,
    time_rem_ot: u32,
) -> u32 {
    let cur_dists = dists.get(&current_me.name).unwrap();

    candidates
        .iter()
        .map(|can| (*cur_dists.get(can).unwrap(), can))
        .filter(|(dist, _)| (dist + 1) <= time_rem_me)
        .map(|(dist, can)| {
            let new_current = valves.get(can).unwrap();
            let after_can_open = time_rem_me - dist - 1;
            let can_val = new_current.flow * after_can_open;

            (can, new_current, after_can_open, can_val)
        })
        .sorted_by_key(|t| t.3)
        .rev()
        .take(5) // smallest for correct result
        .map(|(can, new_current, after_can_open, can_val)| {
            if candidates.len() == 1 {
                return can_val;
            }

            let mut new_cands = candidates.clone();
            new_cands.remove(can);

            let (new_me, new_ot, new_time_me, new_time_ot) = if after_can_open > time_rem_ot {
                (new_current, current_ot, after_can_open, time_rem_ot)
            } else {
                (current_ot, new_current, time_rem_ot, after_can_open)
            };

            analyze_in_pair(
                valves,
                dists,
                new_cands,
                new_me,
                new_ot,
                new_time_me,
                new_time_ot,
            ) + can_val
        })
        .max()
        .unwrap_or(0)
}

fn get_distances(valves: &BTreeMap<String, Valve>) -> BTreeMap<String, BTreeMap<String, u32>> {
    let valves_count = valves.len();
    let mut res = BTreeMap::new();

    for val in valves.values() {
        let dists = res.entry(val.name.clone()).or_insert_with(BTreeMap::new);

        let mut queue = VecDeque::new();
        queue.push_front((val, 0));

        while dists.len() < valves_count {
            let (cur, depth) = queue.pop_front().unwrap();
            let depth = depth + 1;

            for tun in cur.tunnels.iter() {
                if dists.contains_key(tun) {
                    continue;
                }

                dists.insert(tun.clone(), depth);
                queue.push_back((valves.get(tun).unwrap(), depth));
            }
        }
    }

    res
}

fn parse(input: String) -> BTreeMap<String, Valve> {
    let valve_rgx = Regex::new("Valve (?P<name>[A-Z]{2}) .* rate=(?P<flow>\\d+); .* valves? (?P<tunnels>(?:[A-Z]{2}(?:, )?)+)").unwrap();

    BTreeMap::from_iter(
        input
            .lines()
            .filter_map(|line| valve_rgx.captures(line))
            .map(|cap| {
                let name = cap["name"].to_owned();
                (
                    name.clone(),
                    Valve {
                        name,
                        flow: cap["flow"].parse::<u32>().unwrap(),
                        tunnels: cap["tunnels"]
                            .split(", ")
                            .map(|n| n.to_owned())
                            .collect_vec(),
                    },
                )
            }),
    )
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow: u32,
    tunnels: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[rstest]
    #[case(TEST_CASE, "1651")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned()));
    }

    #[rstest]
    #[case(TEST_CASE, "1707")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned()));
    }
}
