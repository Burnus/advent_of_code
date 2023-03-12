use std::fs::read_to_string;

fn read_file(name: &str) -> String {
    read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
}

fn is_vowel(c: char) -> bool {
    ['a', 'e', 'i', 'o', 'u'].contains(&c)
}

fn is_nice_v2(line: &str) -> bool {
    let mut pair = false;
    for j in 0..line.len()-3 {
        let this_pair = &line[j..=j+1];
        if line[j+2..].contains(this_pair) {
            pair = true;
            break;
        }
    }
    let mut repeat = false;
    for i in 0..line.len()-2 {
        if line.chars().nth(i) == line.chars().nth(i+2) {
            repeat = true;
            break;
        }
    }
    
    pair && repeat
}

fn is_nice_v1(line: &str) -> bool {
    let mut repeat = false;
    for i in 0..line.len()-1 {
        if line.chars().nth(i) == line.chars().nth(i+1) {
            repeat = true;
            break;
        }
    }
    line.chars().filter(|c| is_vowel(*c)).count() > 2 &&
    repeat &&
    !line.contains("ab") && !line.contains("cd") && !line.contains("pq") && !line.contains("xy")
}

pub fn run(input: &str) -> (usize, usize) {
    let first = input.lines().filter(|l| is_nice_v1(l)).count();
    let second = input.lines().filter(|l| is_nice_v2(l)).count();
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), (2, 2));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (255, 55));
    }
}
