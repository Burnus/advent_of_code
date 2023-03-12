pub fn run(input: &str, count: u8) -> String {
    let mut look = String::from(input);
    for _ in 0..count {
        look = look_and_say(&look);
    }

    look
}

fn look_and_say(number: &str) -> String {
    let mut say = String::new();
    let mut last_digit = ' ';
    let mut count = 0;

    for digit in number.chars() {
        if digit == last_digit {
            count += 1;
        } else {
            say += &(count.to_string() + &last_digit.to_string());
            count = 1;
            last_digit = digit;
        }
    }
    say += &(count.to_string() + &last_digit.to_string());
    say[2..].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    /*use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
    }*/

    #[test]
    fn test_sample() {
        let sample_input = "1";
        assert_eq!(run(sample_input, 5), "312211".to_string());
    }

    #[test]
    fn test_challenge() {
        let challenge_input = "1113122113";
        let after_40 = run(challenge_input, 40);
        assert_eq!(after_40.len(), 360154);
        assert_eq!(run(&after_40, 10).len(), 5103798);
    }
}
