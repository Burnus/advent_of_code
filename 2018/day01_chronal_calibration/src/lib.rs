use std::collections::HashSet;

pub fn run(input: &str) -> (isize, isize) {
    let list: Vec<_> = input.lines().map(|i| i.parse::<isize>().unwrap()).collect();
    let first = list.iter().sum();
    let mut seen = HashSet::new();
    let mut second = 0;
    'outer: loop {
        for next in &list {
            second += next;
            if seen.contains(&second) {
                break 'outer;
            }
            seen.insert(second);
        }
    }
    (first, second)
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
        assert_eq!(run(&sample_input), (3, 2));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (437, 655));
    }
}
