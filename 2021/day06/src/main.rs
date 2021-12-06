use std::fs;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    grow(input, 80).to_string()
}

fn process_data_adv(input: String) -> String {
    grow(input, 256).to_string()
}

fn grow(input: String, days: u32) -> u64 {
    let mut school = [0u64; 7];
    let mut maturing = [0u64; 7];

    for t in input.trim().split(',') {
        let i = t.parse::<usize>().unwrap();
        school[i] += 1;
    }

    let mut r = 0usize;

    for _d in 0..(days + 2) {
        maturing[(r + 2) % 7] = school[r];
        school[r] += maturing[r];
        r = (r + 1) % 7;
    }

    school.iter().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "3,4,3,1,2
    ";

    #[test]
    fn base_check() {
        assert_eq!("5934", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("26984457539", process_data_adv(TEST_CASE.to_string()));
    }
}
