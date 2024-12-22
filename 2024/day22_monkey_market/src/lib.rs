use core::fmt::Display;
use std::{collections::{HashMap, HashSet}, num::ParseIntError};

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

fn next_secret(secret: usize) -> usize {
    let mut secret = ((secret * 64) ^ secret) % 16777216;
    secret = ((secret / 32) ^ secret) % 16777216;
   ((secret * 2048) ^ secret) % 16777216
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut secrets: Vec<_> = input.lines().map(|n| n.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
    let mut mem = HashMap::new();
    secrets.iter_mut().for_each(|s| {
        let mut seen = HashSet::new();
        let mut last_changes = Vec::with_capacity(4);
        (0..4).for_each(|_| {
            let next_secret = next_secret(*s);
            let change = (next_secret % 10) as i8 - (*s % 10) as i8;
            last_changes.push(change);
            *s = next_secret;
        });
        let last = (last_changes[0], last_changes[1], last_changes[2], last_changes[3]);
        seen.insert(last);
                mem.entry(last)
                    .and_modify(|price| *price += *s % 10)
                    .or_insert(*s % 10);
        (4..2000).for_each(|_| {
            let next_secret = next_secret(*s);
            let change = (next_secret % 10) as i8 - (*s % 10) as i8;
            last_changes.rotate_left(1);
            last_changes[3] = change;
            let last = (last_changes[0], last_changes[1], last_changes[2], last_changes[3]);
            if !seen.contains(&last) {
                mem.entry(last)
                    .and_modify(|price| *price += next_secret % 10)
                    .or_insert(next_secret % 10);
                seen.insert(last);
            }
            *s = next_secret;
        });
    });
    let first = secrets.iter().sum();
    let second = *mem.values().max().unwrap_or(&0);
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
    fn test_next() {
        let expected = [
            123,
            15887950,
            16495136,
            527345,
            704524,
            1553684,
            12683156,
            11100544,
            12249484,
            7753432,
            5908254,
        ];
        expected.windows(2).for_each(|secrets| 
            assert_eq!(next_secret(secrets[0]), secrets[1])
        );
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), Ok((37990510, 23)));
    }

    #[test]
    // #[ignore = "reason"]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((20071921341, 2242)));
    }
}
