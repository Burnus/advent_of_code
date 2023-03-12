pub fn run(input: &str) -> (usize, usize) {
    let range: Vec<_> = input.split('-').map(|n| n.parse::<usize>().unwrap()).collect();
    let valid_1: Vec<_> = (range[0]..=range[1]).filter(is_valid_1).collect();
    let first = valid_1.len();
    let second = valid_1.into_iter().filter(is_valid_2).count();
    (first, second)
}

fn is_valid_1(password: &usize) -> bool {
    let mut double_found = false;
    let mut last_digit = password % 10;
    let mut remaining = password / 10;
    while remaining > 0 {
        let this_digit = remaining % 10;
        if this_digit > last_digit {
            return false;
        }
        if this_digit == last_digit {
            double_found = true;
        }
        last_digit = this_digit;
        remaining /= 10;
    }
    double_found
}

fn is_valid_2(password: &usize) -> bool {
    let mut current_group_len = 1;
    let mut last_digit = password % 10;
    let mut remaining = password / 10;
    while remaining > 0 {
        let this_digit = remaining % 10;
        if this_digit == last_digit {
            current_group_len += 1;
        } else if current_group_len == 2 {
            return true;
        } else {
            current_group_len = 1;
        }
        last_digit = this_digit;
        remaining /= 10;
    }
    current_group_len == 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), (36, 14));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (921, 603));
    }
}
