use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ParseIntError(std::num::ParseIntError),
    LineMalformed(String),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

struct Entry {
    left: usize,
    right: usize,
    letter: char,
    password: String,
}

impl TryFrom<&str> for Entry {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let components: Vec<_> = value.split(&['-', ' ']).collect();
        if components.len() != 4 {
            return Err(ParseError::LineMalformed(value.to_string()));
        }

        Ok(Self {
            left: components[0].parse()?,
            right: components[1].parse()?,
            letter: components[2].chars().next().unwrap(),
            password: components[3].to_string(),
        })
    }
}

impl Entry {
    fn is_valid_1(&self) -> bool {
        (self.left..=self.right).contains(&self.password.chars().filter(|&c| c == self.letter).count())
    }

    fn is_valid_2(&self) -> bool {
        (self.password.chars().nth(self.left-1) == Some(self.letter)) ^ (self.password.chars().nth(self.right-1) == Some(self.letter))
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let entries: Vec<_> = input.lines().map(Entry::try_from).collect::<Result<Vec<_>, _>>()?;
    let first = entries.iter().filter(|e| e.is_valid_1()).count();
    let second = entries.iter().filter(|e| e.is_valid_2()).count();
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
        assert_eq!(run(&sample_input), Ok((2, 1)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((517, 0)));
    }
}
