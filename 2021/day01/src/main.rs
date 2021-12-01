use std::fs;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    input
        .lines()
        .fold((0, Option::<u32>::None), |t, line| {
            let new = line.trim().parse::<u32>().unwrap();

            let new_count =
                t.1.map(|old| if old < new { t.0 + 1 } else { t.0 })
                    .unwrap_or(0);

            (new_count, Some(new))
        })
        .0
        .to_string()
}

fn process_data_adv(input: String) -> String {
    input
        .lines()
        .fold(SlidingTotal::default(), |t, line| {
            let new = line.trim().parse::<u32>().unwrap();

            let new_count = t
                .oldest
                .and_then(|v| t.older.map(|iv| v < (iv + new)))
                .map(|b| if b { t.count + 1 } else { t.count })
                .unwrap_or(0);

            SlidingTotal {
                count: new_count,
                oldest: t.older.map(|v| v + new),
                older: t.old.map(|v| v + new),
                old: Some(new),
            }
        })
        .count
        .to_string()
}

#[derive(Default)]
struct SlidingTotal {
    count: i32,
    old: Option<u32>,
    older: Option<u32>,
    oldest: Option<u32>,
}

#[cfg(test)]
mod tests {
    use crate::{process_data, process_data_adv};

    #[test]
    fn base_check() {
        let test_case = "199
            200
            208
            210
            200
            207
            240
            269
            260
            263";

        assert_eq!("7", process_data(test_case.to_string()));
    }

    #[test]
    fn adv_check() {
        let test_case = "199
            200
            208
            210
            200
            207
            240
            269
            260
            263";

        assert_eq!("5", process_data_adv(test_case.to_string()));
    }
}
