use core::fmt::Display;
use std::{num::ParseIntError, collections::VecDeque};

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

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut add_list = VecDeque::from([0; 11]);
    let mut total_score = 0;
    let mut total_cards = 0;
    for line in input.lines() {
        let parts: Vec<_> = line.split(&[':', '|']).collect();
        if parts.len() != 3 {
            return Err(ParseError::LineMalformed(line));
        }
        let mut winning_numbers = 0_u128;
        for wn in parts[1].split_whitespace() {
            let n = wn.parse::<u32>()?;
            winning_numbers |= 2_u128.pow(n);
        }
        let mut own_numbers = 0_u128;
        for on in parts[2].split_whitespace() {
            let n = on.parse::<u32>()?;
            own_numbers |= 2_u128.pow(n);
        }
        let correct = (winning_numbers & own_numbers).count_ones();
        let this_count = 1 + add_list.pop_front().unwrap_or(0);
        total_cards += this_count;
        if correct > 0 {
            total_score += 2_usize.pow(correct-1);
            add_list.iter_mut().take(correct as usize).for_each(|l| *l += this_count);
        }
        add_list.push_back(0);
    }
    Ok((total_score, total_cards))
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
