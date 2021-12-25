use std::fs;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let mut cucumbers = parse(input);

    for i in 1.. {
        if !step(&mut cucumbers) {
            return i.to_string();
        }
    }

    "Infinite iterator reached an end".to_owned()
}

fn process_data_adv(input: String) -> String {
    "".to_string()
}

fn step(cucumbers: &mut Cucumbers) -> bool {
    let eastern_indexes: Vec<(usize, usize)> = cucumbers
        .area
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if matches!(c, Cucumber::Eastern) {
                let next_i = i + ((i + 1) % cucumbers.dimension) - (i % cucumbers.dimension);
                if let Some(next_c) = cucumbers.area.get(next_i) {
                    if matches!(next_c, Cucumber::None) {
                        Some((i, next_i))
                    } else {
                        None
                    }
                } else {
                    panic!("Invalid index: {}", next_i);
                }
            } else {
                None
            }
        })
        .collect();

    for (i, next_i) in eastern_indexes.iter() {
        cucumbers.area[*i] = Cucumber::None;
        cucumbers.area[*next_i] = Cucumber::Eastern;
    }

    let southern_indexes: Vec<(usize, usize)> = cucumbers
        .area
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if matches!(c, Cucumber::Southern) {
                let next_i = (i + cucumbers.dimension) % cucumbers.size;
                if let Some(next_c) = cucumbers.area.get(next_i) {
                    if matches!(next_c, Cucumber::None) {
                        Some((i, next_i))
                    } else {
                        None
                    }
                } else {
                    panic!("Invalid index: {}", next_i);
                }
            } else {
                None
            }
        })
        .collect();

    for (i, next_i) in southern_indexes.iter() {
        cucumbers.area[*i] = Cucumber::None;
        cucumbers.area[*next_i] = Cucumber::Southern;
    }

    !eastern_indexes.is_empty() || !southern_indexes.is_empty()
}

fn parse(input: String) -> Cucumbers {
    let cucumbers: Vec<Cucumber> = input
        .trim()
        .lines()
        .flat_map(|l| l.trim().chars())
        .map(|c| match c {
            'v' => Cucumber::Southern,
            '>' => Cucumber::Eastern,
            '.' => Cucumber::None,
            x => panic!("Unknown char: {}", x),
        })
        .collect();

    Cucumbers {
        dimension: input.trim().lines().next().unwrap().len(),
        size: cucumbers.len(),
        area: cucumbers,
    }
}

enum Cucumber {
    None,
    Eastern,
    Southern,
}

struct Cucumbers {
    dimension: usize,
    size: usize,
    area: Vec<Cucumber>,
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "v...>>.vv>
    .vv>>.vv..
    >>.>v>...v
    >>v>>.>.v.
    v>v.vv.v..
    >.>>..v...
    .vv..>.>v.
    v.v..>>v.v
    ....v..v.>
    ";

    #[test]
    fn base_check() {
        assert_eq!("58", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("", process_data_adv(TEST_CASE.to_string()));
    }
}
