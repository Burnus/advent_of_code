#[derive(Clone, Copy)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Move {
    fn parse(line: &str) -> Self {
        match line.chars().next() {
            Some('s') => Self::Spin(line[1..].parse().unwrap()),
            Some('x') => {
                    let (l, r) = line[1..].split_once('/').unwrap();
                    Self::Exchange(l.parse().unwrap(), r.parse().unwrap())
                },
            Some('p') => Self::Partner(line.chars().nth(1).unwrap(), line.chars().nth(3).unwrap()),
            _ => panic!("Unexpected Move: {line}"),
        }
    }

    fn perform(self, target: &mut [char]) {
        match self {
            Self::Spin(by) => target.rotate_right(by),
            Self::Exchange(a, b) => target.swap(a, b),
            Self::Partner(a, b) => {
                let pos_a = target.iter().position(|c| *c==a).unwrap();
                let pos_b = target.iter().position(|c| *c==b).unwrap();
                target[pos_a] = b;
                target[pos_b] = a;
            },
        }
    }
}

pub fn run(input: &str) -> (String, String) {
    let initial_line = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'];
    let mut line = initial_line;
    let moves: Vec<_> = input.split(',').map(Move::parse).collect();
    for mov in &moves {
        mov.perform(&mut line);
    }
    let first: String = line.iter().collect();

    let mut remainder = None;
    for iter in 2..=1_000_000_000 {
        for mov in &moves {
            mov.perform(&mut line);
        }
        if line == initial_line {
            eprintln!("Arrangement from 0 repeated at {iter}");
            remainder = Some(1_000_000_000%iter);
            break;
        }
    }
    if let Some(r) = remainder {
        for _ in 0..r {
            for mov in &moves {
                mov.perform(&mut line);
            }
        }
    }
    let second = line.iter().collect();
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
        assert_eq!(run(&sample_input), ("paedcbfghijklmno".to_string(), "ghidjklmnopabcef".to_string()));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), ("cgpfhdnambekjiol".to_string(), "gjmiofcnaehpdlbk".to_string()));
    }
}
