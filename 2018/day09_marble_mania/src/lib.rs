use std::collections::VecDeque;

pub fn run(input: &str) -> (usize, usize) {
    let (players, turns) = parse_game(input);
    let mut scores = vec![0; players];
    let mut circle = VecDeque::from([1, 0]);
    for turn in 2..=turns {
        if turn % 23 == 0 {
            circle.rotate_right(7);
            scores[turn % players] += turn + circle.pop_front().unwrap();
        } else {
            circle.rotate_left(2);
            circle.push_front(turn);
        }
    }
    let first = *scores.iter().max().unwrap();
    for turn in turns+1..=100*turns {
        if turn % 23 == 0 {
            circle.rotate_right(7);
            scores[turn % players] += turn + circle.pop_front().unwrap();
        } else {
            circle.rotate_left(2);
            circle.push_front(turn);
        }
    }
    let second = *scores.iter().max().unwrap();
    (first, second)
}

fn parse_game(input: &str) -> (usize, usize) {
    let words: Vec<_> = input.split_whitespace().collect();
    assert_eq!(words.len(), 8);
    (words[0].parse().unwrap(), words[6].parse().unwrap())
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
        let sample_inputs = read_file("tests/sample_input");
        let expected = [
            (8317, 74765078),
            (146373, 1406506154),
            (2764, 20548882),
            (54718, 507583214),
            (37305, 320997431),
        ];
        for (idx, input) in sample_inputs.lines().enumerate() {
            assert_eq!(run(input), expected[idx]);
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (404611, 3350093681));
    }
}
