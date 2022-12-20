use std::fs;

use indicatif::ProgressIterator;
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
    const TIME_LIMIT: u32 = 24;

    let blueprints = parse(input);
    let resources = Resources::new();
    let robots = Robots::new();

    blueprints
        .iter()
        .progress()
        .map(|b| {
            (
                b.id,
                strategy(
                    b,
                    resources.clone(),
                    robots.clone(),
                    TIME_LIMIT,
                    true,
                    true,
                    true,
                ),
            )
        })
        .map(|(id, geodes)| id * geodes)
        .sum::<u32>()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    const TIME_LIMIT: u32 = 32;

    let blueprints = parse(input);
    let resources = Resources::new();
    let robots = Robots::new();

    blueprints
        .iter()
        .take(3)
        .progress()
        .map(|b| {
            strategy(
                b,
                resources.clone(),
                robots.clone(),
                TIME_LIMIT,
                true,
                true,
                true,
            )
        })
        .product::<u32>()
        .to_string()
}

fn strategy(
    blueprint: &Blueprint,
    resources: Resources,
    robots: Robots,
    time: u32,
    can_build_ore: bool,
    can_build_clay: bool,
    can_build_obs: bool,
) -> u32 {
    if time == 0 {
        return resources.cracked_geodes;
    }

    let (geo_ore_cost, geo_obs_cost) = blueprint.geode_robot_cost;
    let (obs_ore_cost, obs_clay_cost) = blueprint.obsidian_robot_cost;
    let max_ore = blueprint
        .clay_robot_cost
        .max(obs_ore_cost)
        .max(geo_ore_cost);

    let can_build_ore =
        can_build_ore && robots.ore_robots < max_ore && resources.ores >= blueprint.ore_robot_cost;
    let can_build_clay = can_build_clay
        && robots.clay_robots < obs_clay_cost
        && resources.ores >= blueprint.clay_robot_cost;
    let can_build_obs = can_build_obs
        && robots.obsidian_robots < geo_obs_cost
        && resources.ores >= obs_ore_cost
        && resources.clay >= obs_clay_cost;
    let can_build_geo = resources.ores >= geo_ore_cost && resources.obsidian >= geo_obs_cost;

    let mut branches = Vec::<u32>::new();

    if can_build_geo || can_build_obs {
        if can_build_geo {
            let mut new_res = resources.clone();
            let mut new_bot = robots.clone();
            new_res.ores -= geo_ore_cost;
            new_res.obsidian -= geo_obs_cost;
            new_res.inc(&robots);
            new_bot.geode_robots += 1;

            branches.push(strategy(
                blueprint,
                new_res,
                new_bot,
                time - 1,
                true,
                true,
                true,
            ));
        }

        if can_build_obs {
            let mut new_res = resources.clone();
            let mut new_bot = robots.clone();
            new_res.ores -= obs_ore_cost;
            new_res.clay -= obs_clay_cost;
            new_res.inc(&robots);
            new_bot.obsidian_robots += 1;

            branches.push(strategy(
                blueprint,
                new_res,
                new_bot,
                time - 1,
                true,
                true,
                true,
            ));
        }

        if can_build_geo && can_build_obs {
            return branches.iter().max().cloned().unwrap();
        }

    } else {
        if can_build_clay {
            let mut new_res = resources.clone();
            let mut new_bot = robots.clone();
            new_res.ores -= blueprint.clay_robot_cost;
            new_res.inc(&robots);
            new_bot.clay_robots += 1;

            branches.push(strategy(
                blueprint,
                new_res,
                new_bot,
                time - 1,
                true,
                true,
                true,
            ));
        }

        if can_build_ore {
            let mut new_res = resources.clone();
            let mut new_bot = robots.clone();
            new_res.ores -= blueprint.ore_robot_cost;
            new_res.inc(&robots);
            new_bot.ore_robots += 1;

            branches.push(strategy(
                blueprint,
                new_res,
                new_bot,
                time - 1,
                true,
                true,
                true,
            ));
        }
    }

    {
        let can_build_ore = resources.ores < blueprint.ore_robot_cost;
        let can_build_clay = resources.ores < blueprint.clay_robot_cost;
        let can_build_obs = resources.ores < obs_ore_cost || resources.clay < obs_clay_cost;

        let mut new_res = resources;
        new_res.inc(&robots);
        branches.push(strategy(
            blueprint,
            new_res,
            robots.clone(),
            time - 1,
            can_build_ore,
            can_build_clay,
            can_build_obs,
        ));
    }

    branches.iter().max().cloned().unwrap()
}

fn parse(input: String) -> Vec<Blueprint> {
    const PARSE_RGX: &str = concat!(
        "Blueprint (?P<id>\\d+):",
        ".+costs (?P<orebot>\\d+) ore",
        ".+costs (?P<claybot>\\d+) ore",
        ".+costs (?P<obsbotore>\\d+) ore and (?P<obsbotclay>\\d+) clay",
        ".+costs (?P<geobotore>\\d+) ore and (?P<geobotobs>\\d+) obsidian."
    );

    let rgx = Regex::new(PARSE_RGX).unwrap();

    input
        .lines()
        .filter_map(|line| rgx.captures(line))
        .map(|caps| Blueprint {
            id: caps["id"].parse::<u32>().unwrap(),
            ore_robot_cost: caps["orebot"].parse::<u32>().unwrap(),
            clay_robot_cost: caps["claybot"].parse::<u32>().unwrap(),
            obsidian_robot_cost: (
                caps["obsbotore"].parse::<u32>().unwrap(),
                caps["obsbotclay"].parse::<u32>().unwrap(),
            ),
            geode_robot_cost: (
                caps["geobotore"].parse::<u32>().unwrap(),
                caps["geobotobs"].parse::<u32>().unwrap(),
            ),
        })
        .collect_vec()
}

#[derive(Clone, Debug)]
struct Resources {
    ores: u32,
    clay: u32,
    obsidian: u32,
    cracked_geodes: u32,
}

impl Resources {
    fn new() -> Self {
        Resources {
            ores: 0,
            clay: 0,
            obsidian: 0,
            cracked_geodes: 0,
        }
    }

    fn inc(&mut self, robots: &Robots) {
        self.ores += robots.ore_robots;
        self.clay += robots.clay_robots;
        self.obsidian += robots.obsidian_robots;
        self.cracked_geodes += robots.geode_robots;
    }
}

#[derive(Clone, Debug)]
struct Robots {
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl Robots {
    fn new() -> Self {
        Robots {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    obsidian_robot_cost: (u32, u32),
    geode_robot_cost: (u32, u32),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[rstest]
    #[case(TEST_CASE, "33")]
    fn base_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data(input.to_owned()));
    }

    #[rstest]
    #[case(TEST_CASE, "3472")]
    fn adv_check(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_data_adv(input.to_owned()));
    }
}
