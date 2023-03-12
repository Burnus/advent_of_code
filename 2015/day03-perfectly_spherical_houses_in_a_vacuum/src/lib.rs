use std::{fs::read_to_string, collections::HashSet};

fn read_file(name: &str) -> String {
    read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
}

fn movement((x, y): (isize, isize), direction: char) -> (isize, isize) {
    match direction {
        '^' => (x, y-1),
        'v' => (x, y+1),
        '<' => (x-1, y),
        '>' => (x+1, y),
        _ => (x, y),
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let mut first: HashSet<(isize, isize)> = input.chars().scan((0, 0), |curr, c| {
        *curr = movement(*curr, c);
        Some(*curr)
    }).collect();
    first.insert((0, 0));

    let mut second: HashSet<(isize, isize)> = input.chars().enumerate().scan(((0,0), (0,0)), |(santa, robo), (idx, c)| {
        match idx % 2 {
            0 => { *santa = movement(*santa, c); Some(*santa) },
            1 => { *robo = movement(*robo, c); Some(*robo) },
            _ => unreachable!(),
        }
    }).collect();
    second.insert((0, 0));

    (first.len(), second.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        let sample_input: Vec<&str> = sample_input.lines().collect();
        let expected = [(2,2), (4,3), (2,11)];
        for (index, input) in sample_input.iter().enumerate() {
            assert_eq!(run(input), expected[index]);
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (2592, 2360));
    }
}
