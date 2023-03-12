pub fn run(input: &str) -> (usize, usize) {
    let passphrases: Vec<Vec<&str>> = input.lines().map(|line| line.split_whitespace().collect()).collect();
    let first = passphrases.iter().filter(|phrase| is_duplicate_free(phrase)).count();
    let second = passphrases.iter().filter(|phrase| is_anagram_free(phrase)).count();
    (first, second)
}

fn is_anagram_free(words: &[&str]) -> bool {
    !words.iter().enumerate().any(|(idx, word)| {
        let mut this = word.as_bytes().to_vec();
        this.sort();
        words.iter().skip(idx+1).any(|that| {
           let mut other = that.as_bytes().to_vec();
           other.sort();
           this == other
        })
    })
}

fn is_duplicate_free(words: &[&str]) -> bool {
    !words.iter().enumerate().any(|(idx, word)| words[idx+1..].contains(word))
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
        assert_eq!(run(&sample_input), (7, 5));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (337, 231));
    }
}
