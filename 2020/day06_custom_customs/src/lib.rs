pub fn run(input: &str) -> (usize, usize) {
    let groups: Vec<_> = input.split("\n\n").collect();
    let first = groups.iter().map(|group| count_unique_chars(group)).sum();
    let second = groups.iter().map(|group| count_common_chars(group)).sum();
    (first, second)
}

fn count_unique_chars(input: &str) -> usize {
    ('a'..='z').filter(|c| input.contains(*c)).count()
}

fn count_common_chars(input: &str) -> usize {
    ('a'..='z').filter(|c| input.lines().all(|line| line.contains(*c))).count()
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
        assert_eq!(run(&sample_input), (11, 6));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (6521, 3305));
    }
}
