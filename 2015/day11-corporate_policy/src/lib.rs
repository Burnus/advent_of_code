pub fn run(input: &str) -> String {
    let mut next_password = increment(input);
    while !meets_requirements(&next_password) {
        next_password = increment(&next_password);
    }
    next_password
}

fn to_number(password: &str) -> usize {
    password.bytes().map(|b| (b - b'a') as usize).fold(0, |acc, b| 26*acc+b)
}

fn to_password(number: usize) -> String {
    let mut password = String::new();
    let mut number = number;
    while number > 0 {
        password += &char::from(b'a' + (number % 26) as u8).to_string();
        number /= 26;
    }
    password += &("a".repeat(8-password.len()));
    password.chars().rev().collect()
}

fn increment(password: &str) -> String {
    to_password(to_number(password) + 1)
}

fn meets_requirements(password: &str) -> bool {
    let mut includes_straight = false;
    for idx in 0..password.bytes().len()-2 {
        if *password.as_bytes().get(idx+2).unwrap() == password.as_bytes().get(idx).unwrap() + 2 &&
            *password.as_bytes().get(idx+1).unwrap() == password.as_bytes().get(idx).unwrap() + 1 {
                includes_straight = true;
            }
    }
    
    let includes_confusing = password.chars().any(|c| ['i', 'o', 'l'].contains(&c));

    let mut pairs = 0;
    let mut iter = 0..password.chars().count()-1;
    while let Some(idx) = iter.next() {
        if password.chars().nth(idx+1) == password.chars().nth(idx) {
            pairs += 1;
            iter.next();
        }
    }

    includes_straight && !includes_confusing && pairs > 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requirements_tests() {
        let samples = [ "hijklmmn", "abbceffg", "abbcegjk" ];
        for s in samples {
            assert_eq!(to_password(to_number(s)), s.to_string());
            assert!(!meets_requirements(s));
        }
    }

    #[test]
    fn test_sample() {
        let sample_input = ["abcdefgh", "ghjaaaaa" /*"ghijklmn"*/ ];
        let expected = ["abcdffaa", "ghjaabcc"];
        for (idx, s) in sample_input.iter().enumerate() {
            assert_eq!(run(s), expected[idx].to_string());
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = "cqjxjnds";
        let next_password = run(challenge_input);
        assert_eq!(next_password, "cqjxxyzz".to_string());
        assert_eq!(run(&next_password), "cqkaabcc".to_string());
    }
}
