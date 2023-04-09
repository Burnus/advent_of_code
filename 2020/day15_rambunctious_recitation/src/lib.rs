use std::num::ParseIntError;

pub fn run(input: &str) -> Result<(usize, usize), ParseIntError> {
    let initial: Vec<_> = input.split(',').map(|n| n.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
    let mut said_last = vec![0; 30_000_000];
    initial.iter().enumerate().for_each(|(idx, i)| {said_last[*i] = idx+1});
    let mut last_number = initial[initial.len()-1];
    let mut next_number = initial.iter().rev().skip(1).position(|n| *n == last_number).map(|n| n+1).unwrap_or(0);
    (initial.len()+1..=2020).for_each(|turn| {
        last_number = next_number;
        if said_last[last_number] == 0 {
            next_number = 0;
        } else {
            next_number = turn - said_last[last_number];
        }
        said_last[last_number] = turn;
    });
    let first = last_number;
    (2021..=30_000_000).for_each(|turn| {
        last_number = next_number;
        if said_last[last_number] == 0 {
            next_number = 0;
        } else {
            next_number = turn - said_last[last_number];
        }
        said_last[last_number] = turn;
    });
    let second = last_number;
    Ok((first, second))
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
        assert_eq!(run(&sample_input), Ok((436, 175594)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((959, 116590)));
    }
}
