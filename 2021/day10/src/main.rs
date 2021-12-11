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
        .trim()
        .lines()
        .map(|l| l.trim())
        .map(try_parse_line)
        .filter_map(|r| r.err().map(get_miss_score))
        .sum::<u32>()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    let mut scores: Vec<u64> = input
        .trim()
        .lines()
        .map(|l| l.trim())
        .map(try_parse_line)
        .filter_map(|r| r.ok())
        .map(|s| {
            s.iter()
                .rev()
                .fold(0u64, |sum, &c| sum * 5 + get_compl_score(c) as u64)
        })
        .collect();

    scores.sort_unstable();

    scores.get(scores.len() / 2).unwrap().to_string()
}

fn try_parse_line(input: &str) -> Result<Vec<char>, char> {
    let mut stack = Vec::new();

    for c in input.chars() {
        if is_open(c) {
            stack.push(c)
        } else if let Some(top) = stack.pop() {
            if top != get_open(c) {
                return Err(c);
            }
        } else {
            return Err(c);
        }
    }

    Ok(stack)
}

fn get_miss_score(bracket: char) -> u32 {
    match bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_compl_score(bracket: char) -> u32 {
    match bracket {
        ')' | '(' => 1,
        ']' | '[' => 2,
        '}' | '{' => 3,
        '>' | '<' => 4,
        _ => 0,
    }
}

fn is_open(bracket: char) -> bool {
    matches!(bracket, '(' | '[' | '{' | '<')
}

fn get_open(close: char) -> char {
    match close {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Invalid bracket: {}", close),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    ";

    #[test]
    fn base_check() {
        assert_eq!("26397", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("288957", process_data_adv(TEST_CASE.to_string()));
    }
}
