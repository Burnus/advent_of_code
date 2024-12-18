use core::fmt::Display;
use std::{collections::{HashSet, VecDeque}, num::ParseIntError};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    ParseIntError(std::num::ParseIntError),
    LineMalformed(&'a str),
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

type Coordinates = (i8, i8);

fn try_parse_pair(value: &str) -> Result<Coordinates, ParseError> {
    if let Some ((lhs, rhs)) = value.split_once(',') {
        Ok((lhs.parse()?, rhs.parse()?))
    } else {
        Err(ParseError::LineMalformed(value))
    }
}

fn find_path(blocked: &HashSet<Coordinates>, destination: Coordinates) -> usize {
    let mut open_set = VecDeque::from([((0, 0), 0)]);
    let mut visited = HashSet::new();
    while let Some(((x, y), steps)) = open_set.pop_front() {
        if (x, y) == destination {
            return steps;
        }
        [(x-1, y), (x+1, y), (x, y-1), (x, y+1)]
            .into_iter()
            .filter(|&(x, y)| !blocked.contains(&(x, y)) &&
                x >= 0 && y >= 0 && x <= destination.0 && y <= destination.1 )
            .for_each(|new_pos| if !visited.contains(&new_pos) {
                visited.insert(new_pos);
                open_set.push_back((new_pos, steps+1));
            });
    }
    usize::MAX
}

pub fn run(input: &str) -> Result<(usize, String), ParseError> {
    run_challenge(input, (70, 70), 1024)
}

pub fn run_sample(input: &str) -> Result<(usize, String), ParseError> {
    run_challenge(input, (6, 6), 12)
}

fn run_challenge(input: &str, destination: Coordinates, simulate_bytes: usize) -> Result<(usize, String), ParseError> {
    let blocked: HashSet<_> = input.lines().take(simulate_bytes).map(try_parse_pair).collect::<Result<HashSet<_>, _>>()?;
    let first = find_path(&blocked, destination);
    let rest: Vec<_> = input.lines().skip(simulate_bytes).map(try_parse_pair).collect::<Result<Vec<_>, _>>()?;
    let step = (0..rest.len()).collect::<Vec<_>>().partition_point(|&steps| {
        let mut blocked = blocked.clone();
        rest[..=steps].iter().for_each(|byte| _ = blocked.insert(*byte));
        find_path(&blocked, destination) < usize::MAX
    });
    let second = format!("{},{}", rest[step].0, rest[step].1);
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
        assert_eq!(run_sample(&sample_input), Ok((22, "6,1".to_string())));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((302, "24,32".to_string())));
    }
}
