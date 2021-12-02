use std::{cmp, fs};

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let SlidingResultBase { depth, horizontal } =
        input
            .lines()
            .map(convert_to_command)
            .fold(SlidingResultBase::default(), |res, cmd| match cmd {
                MovementCommand::None => res,
                MovementCommand::Down(a) => SlidingResultBase {
                    depth: res.depth + a,
                    ..res
                },
                MovementCommand::Forward(a) => SlidingResultBase {
                    horizontal: res.horizontal + a,
                    ..res
                },
                MovementCommand::Up(a) => SlidingResultBase {
                    depth: cmp::max(0, res.depth - a),
                    ..res
                },
            });

    (depth * horizontal).to_string()
}

fn process_data_adv(input: String) -> String {
    let SlidingResult {
        depth, horizontal, ..
    } = input.lines().map(convert_to_command).fold(
        SlidingResult::default(),
        |res, cmd| match cmd {
            MovementCommand::None => res,
            MovementCommand::Down(a) => SlidingResult {
                aim: res.aim + a,
                ..res
            },
            MovementCommand::Forward(a) => SlidingResult {
                depth: cmp::max(0, res.depth + (res.aim * a)),
                horizontal: res.horizontal + a,
                ..res
            },
            MovementCommand::Up(a) => SlidingResult {
                aim: res.aim - a,
                ..res
            },
        },
    );

    (depth * horizontal).to_string()
}

fn convert_to_command(input: &str) -> MovementCommand {
    let parts: Vec<&str> = input.trim().split(' ').collect();

    parts
        .get(0)
        .and_then(|c| {
            parts
                .get(1)
                .and_then(|a| (*a).parse::<i32>().ok())
                .map(|a| (*c, a))
        })
        .map(|(cmd, amount)| match cmd {
            "down" => MovementCommand::Down(amount),
            "forward" => MovementCommand::Forward(amount),
            "up" => MovementCommand::Up(amount),
            _ => MovementCommand::None,
        })
        .unwrap_or(MovementCommand::None)
}

#[derive(PartialEq, Eq, Debug)]
enum MovementCommand {
    None,
    Down(i32),
    Forward(i32),
    Up(i32),
}

#[derive(Default)]
struct SlidingResultBase {
    depth: i32,
    horizontal: i32,
}

#[derive(Default)]
struct SlidingResult {
    depth: i32,
    aim: i32,
    horizontal: i32,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_check() {
        let parsed = convert_to_command("   forward 3");
        assert_eq!(MovementCommand::Forward(3), parsed);
    }

    #[test]
    fn base_check() {
        let test_case = "forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2";

        assert_eq!("150", process_data(test_case.to_string()));
    }

    #[test]
    fn adv_check() {
        let test_case = "forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2";

        assert_eq!("900", process_data_adv(test_case.to_string()));
    }
}
