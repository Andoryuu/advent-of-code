use std::fs;

use indicatif::ProgressIterator;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    //let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    //println!("Adv result is: {}", adv_output);
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
                strategy(b, resources.clone(), robots.clone(), TIME_LIMIT),
            )
        })
        .map(|(id, geodes)| id * geodes.1)
        .sum::<u32>()
        .to_string()
}

// fn process_data_adv(input: String) -> String {
//     const TIME_LIMIT: u32 = 32;

//     let blueprints = parse(input);
//     let resources = Resources::new();
//     let robots = Robots::new();

//     blueprints
//         .iter()
//         .take(3)
//         .progress()
//         .map(|b| {
//             let (msg, val) = strategy(b, resources.clone(), robots.clone(), TIME_LIMIT);
//             println!("{msg}");
//             val
//         })
//         .product::<u32>()
//         .to_string()
// }

fn strategy(
    blueprint: &Blueprint,
    resources: Resources,
    robots: Robots,
    time: u32,
) -> (String, u32) {
    if time == 0 {
        return (
            format!("time: {time} - completed"),
            resources.cracked_geodes,
        );
    }

    let mut branches = Vec::<(String, u32)>::new();
    let (geo_ore_cost, geo_obs_cost) = blueprint.geode_robot_cost;
    let (obs_ore_cost, obs_clay_cost) = blueprint.obsidian_robot_cost;

    // if time > 2
    //     && resources.ores >= geo_ore_cost
    //     && resources.obsidian >= geo_obs_cost
    //     && robots.obsidian_robots < geo_obs_cost
    //     && resources.ores >= obs_ore_cost
    //     && resources.clay >= obs_clay_cost
    // {
    //     {
    //         let mut new_res = resources.clone();
    //         let mut new_bot = robots.clone();
    //         new_res.ores -= geo_ore_cost;
    //         new_res.obsidian -= geo_obs_cost;
    //         new_res.inc(&robots);
    //         new_bot.geode_robots += 1;

    //         let msg = format!("time: {time} - {new_res:?} - {new_bot:?} - add geo");
    //         let (i_msg, val) = strategy(blueprint, new_res, new_bot, time - 1);
    //         branches.push((msg + "\n" + &i_msg, val));
    //     }

    //     {
    //         let mut new_res = resources.clone();
    //         let mut new_bot = robots.clone();
    //         new_res.ores -= obs_ore_cost;
    //         new_res.clay -= obs_clay_cost;
    //         new_res.inc(&robots);
    //         new_bot.obsidian_robots += 1;

    //         let msg = format!("time: {time} - {new_res:?} - {new_bot:?} - add obs");
    //         let (i_msg, val) = strategy(blueprint, new_res, new_bot, time - 1);
    //         branches.push((msg + "\n" + &i_msg, val));
    //     }

    // } else {
        let mut reserved_ore = 0u32;

        {
            if resources.ores >= geo_ore_cost && resources.obsidian >= geo_obs_cost {
                let mut new_res = resources;
                let mut new_bot = robots.clone();
                new_res.ores -= geo_ore_cost;
                new_res.obsidian -= geo_obs_cost;
                new_res.inc(&robots);
                new_bot.geode_robots += 1;

                let msg = format!("time: {time} - {new_res:?} - {new_bot:?} - add geo");
                let (i_msg, val) = strategy(blueprint, new_res, new_bot, time - 1);
                return (msg + "\n" + &i_msg, val);
            }

            if (resources.ores + (robots.ore_robots * time)) >= geo_ore_cost
                && (resources.obsidian + (robots.obsidian_robots * time)) >= geo_obs_cost
            {
                let req_time = if resources.obsidian >= geo_obs_cost {
                    0
                } else {
                    let missing = geo_obs_cost - resources.obsidian;
                    missing / robots.obsidian_robots + (missing % robots.obsidian_robots != 0) as u32
                };

                reserved_ore += geo_ore_cost;
                reserved_ore -= (robots.ore_robots * req_time).min(reserved_ore);
            }
        }

        if robots.obsidian_robots < geo_obs_cost
            && reserved_ore < resources.ores
            && (resources.ores - reserved_ore) >= obs_ore_cost
            && resources.clay >= obs_clay_cost
        {
            let mut new_res = resources;
            let mut new_bot = robots.clone();
            new_res.ores -= obs_ore_cost;
            new_res.clay -= obs_clay_cost;
            new_res.inc(&robots);
            new_bot.obsidian_robots += 1;

            let msg = format!("time: {time} - {new_res:?} - {new_bot:?} - add obs");
            let (i_msg, val) = strategy(blueprint, new_res, new_bot, time - 1);
            return (msg + "\n" + &i_msg, val);
        }
    //}

    let max_ore = blueprint
        .clay_robot_cost
        .max(blueprint.obsidian_robot_cost.0)
        .max(blueprint.geode_robot_cost.0);


    if robots.ore_robots < max_ore && resources.ores >= blueprint.ore_robot_cost {
        let mut new_res = resources.clone();
        let mut new_bot = robots.clone();
        new_res.ores -= blueprint.ore_robot_cost;
        new_res.inc(&robots);
        new_bot.ore_robots += 1;

        let msg = format!("time: {time} - {new_res:?} - {new_bot:?} - add ore");
        let (i_msg, val) = strategy(blueprint, new_res, new_bot, time - 1);
        branches.push((msg + "\n" + &i_msg, val));
    }

    if robots.clay_robots < blueprint.obsidian_robot_cost.1
        && resources.ores >= blueprint.clay_robot_cost
    {
        let mut new_res = resources.clone();
        let mut new_bot = robots.clone();
        new_res.ores -= blueprint.clay_robot_cost;
        new_res.inc(&robots);
        new_bot.clay_robots += 1;

        let msg = format!("time: {time} - {new_res:?} - {new_bot:?} - add clay");
        let (i_msg, val) = strategy(blueprint, new_res, new_bot, time - 1);
        branches.push((msg + "\n" + &i_msg, val));
    }

    {
        let mut new_res = resources;
        new_res.inc(&robots);
        let msg = format!("time: {time} - {new_res:?} - {robots:?} - add wait");
        let (i_msg, val) = strategy(blueprint, new_res, robots.clone(), time - 1);
        branches.push((msg + "\n" + &i_msg, val));
    }

    branches.iter().max_by_key(|(_, val)| val).cloned().unwrap()
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

    // #[rstest]
    // #[case(TEST_CASE, "3472")]
    // fn adv_check(#[case] input: &str, #[case] expected: &str) {
    //     assert_eq!(expected, process_data_adv(input.to_owned()));
    // }
}
