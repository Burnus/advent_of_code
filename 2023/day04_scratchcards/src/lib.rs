use core::fmt::Display;
use std::{num::ParseIntError, collections::HashSet};

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

struct Scratchcard {
    count: usize,
    winning_numbers: HashSet<usize>,
    own_numbers: HashSet<usize>,
}

impl<'a> TryFrom<&'a str> for Scratchcard {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let parts: Vec<_> = value.split(&[':', '|']).collect();
        if parts.len() != 3 {
            return Err(Self::Error::LineMalformed(value));
        }
        let winning_numbers: HashSet<_> = parts[1].split_whitespace().map(|n| n.parse::<usize>()).collect::<Result<HashSet<_>, std::num::ParseIntError>>()?;
        let own_numbers: HashSet<_> = parts[2].split_whitespace().map(|n| n.parse::<usize>()).collect::<Result<HashSet<_>, std::num::ParseIntError>>()?;

        Ok(Scratchcard{
            count: 1,
            winning_numbers,
            own_numbers,
        })
    }
}

impl Scratchcard {
    fn points(&self) -> usize {
        let correct_count = self.winning_numbers.intersection(&self.own_numbers).count();
        match correct_count {
            0 => 0,
            n => 2_usize.pow(<usize as TryInto<u32>>::try_into(n).unwrap() - 1),
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut cards: Vec<_> = input.lines().map(Scratchcard::try_from).collect::<Result<Vec<_>, _>>()?;
    let first = cards.iter().map(|s| s.points()).sum();
    for i in 0..cards.len() {
        let this_count = cards[i].count;
        let correct = cards[i].winning_numbers.intersection(&cards[i].own_numbers).count();
        if correct > 0 {
            for j in i+1..=(i+correct).min(cards.len()-1) {
                cards[j].count += this_count;
            }
        }
    }
    let second = cards.iter().map(|c| c.count).sum();
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
        assert_eq!(run(&sample_input), Ok((13, 30)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((21158, 6050769)));
    }
}
