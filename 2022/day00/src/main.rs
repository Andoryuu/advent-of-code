use std::fs;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    "".to_string()
}

fn process_data_adv(input: String) -> String {
    "".to_string()
}


#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    const TEST_CASE: &str = "";

    #[rstest]
    fn base_check() {
        assert_eq!("", process_data(TEST_CASE.to_string()));
    }

    #[rstest]
    fn adv_check() {
        assert_eq!("", process_data_adv(TEST_CASE.to_string()));
    }
}
