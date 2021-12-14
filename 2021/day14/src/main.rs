use std::{collections::BTreeMap, fs};

use lazy_static::lazy_static;
use regex::{Match, Regex};

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let (mut template, rules, mut counts) = parse(input);

    for _ in 0..10 {
        apply_rules(&rules, &mut template, &mut counts)
    }

    (counts.values().max().unwrap() - counts.values().min().unwrap()).to_string()
}

fn process_data_adv(input: String) -> String {
    let (mut template, rules, mut counts) = parse(input);

    for _ in 0..40 {
        apply_rules(&rules, &mut template, &mut counts)
    }

    (counts.values().max().unwrap() - counts.values().min().unwrap()).to_string()
}

type RuleMap = BTreeMap<(char, char), char>;
type PolymerPairs = BTreeMap<(char, char), u64>;
type RunningCount = BTreeMap<char, u64>;

fn apply_rules(rules: &RuleMap, pairs: &mut PolymerPairs, counts: &mut RunningCount) {
    let mut to_remove = Vec::new();
    let mut to_add = Vec::new();

    for (template, &replacement) in rules.iter() {
        if let Some(&current) = pairs.get(template) {
            to_remove.push(template);
            to_add.push(((template.0, replacement), current));
            to_add.push(((replacement, template.1), current));

            let count = counts.entry(replacement).or_insert(0);
            *count += current;
        }
    }

    for rem in to_remove.iter() {
        pairs.remove(rem);
    }

    for (key, value) in to_add.iter() {
        let pair = pairs.entry(*key).or_insert(0);
        *pair += value;
    }
}

fn parse(input: String) -> (PolymerPairs, RuleMap, RunningCount) {
    lazy_static! {
        static ref RULE_RE: Regex = Regex::new("^([A-Z])([A-Z]) -> ([A-Z])$").unwrap();
    }

    let lines: Vec<&str> = input.trim().lines().map(|l| l.trim()).collect();

    let template: Vec<char> = lines.first().map(|l| l.chars().collect()).unwrap();

    let mut running_count = BTreeMap::new();
    for &c in template.iter() {
        let count = running_count.entry(c).or_insert(0);
        *count += 1;
    }

    let mut template_pairs = BTreeMap::new();
    template
        .iter()
        .zip(template.iter().skip(1))
        .for_each(|(&c1, &c2)| {
            let entry = template_pairs.entry((c1, c2)).or_insert(0);
            *entry += 1;
        });

    let rules = BTreeMap::from_iter(
        lines
            .iter()
            .skip(2)
            .filter_map(|l| RULE_RE.captures(l))
            .map(|c| {
                (
                    (
                        parse_as_char(c.get(1)).unwrap(),
                        parse_as_char(c.get(2)).unwrap(),
                    ),
                    parse_as_char(c.get(3)).unwrap(),
                )
            }),
    );

    (template_pairs, rules, running_count)
}

fn parse_as_char(cap: Option<Match>) -> Option<char> {
    cap.and_then(|m| m.as_str().chars().next())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C
    ";

    #[test]
    fn base_check() {
        assert_eq!("1588", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("2188189693529", process_data_adv(TEST_CASE.to_string()));
    }
}
