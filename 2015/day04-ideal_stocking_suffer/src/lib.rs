use md5::{Md5, Digest};

fn hash_has_leading_zeroes(input: &str, counter: usize, zeroes: usize) -> bool {
        let mut hasher = Md5::new();
        hasher.update(input.to_owned() + &(counter.to_string())[..]);
        let hash = hasher.finalize();
        for i in 0..zeroes/2 {
            if hash[i] != 0 {
                return false;
            }
            if zeroes % 2 == 1 && hash[zeroes/2] >= 16 {
                return false;
            }
        }
        true
}

pub fn run(input: &str) -> (usize, usize) {
    let first = (0..).find(|i| hash_has_leading_zeroes(input, *i, 5)).unwrap();

    let second = (first..).find(|i| hash_has_leading_zeroes(input, *i, 6)).unwrap();

    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        let expected = [(609043, 6742839), (1048970, 5714438)];
        for (idx, input) in sample_input.lines().enumerate() {
            assert_eq!(run(input), expected[idx]);
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        for input in challenge_input.lines() {
            assert_eq!(run(input), (254575, 1038736));
        }
    }
}
