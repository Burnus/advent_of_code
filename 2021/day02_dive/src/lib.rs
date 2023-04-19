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

enum Movement {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl TryFrom<&str> for Movement {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let words: Vec<_> = value.split_whitespace().collect();
        if words.len() != 2 {
            Err(Self::Error::LineMalformed(value.to_string()))
        } else {
            match words[0] {
                "forward" => Ok(Self::Forward(words[1].parse()?)),
                "down" => Ok(Self::Down(words[1].parse()?)),
                "up" => Ok(Self::Up(words[1].parse()?)),
                _ => Err(Self::Error::LineMalformed(value.to_string())),
            }
        }
    }
}

pub fn run(input: &str) -> Result<(isize, isize), ParseError> {
    let course: Vec<_> = input.lines().map(Movement::try_from).collect::<Result<Vec<_>, _>>()?;

    // Fold the movements into (horizontal, depth)
    let target_1 = course.iter().fold((0, 0), |pos, movement| match movement {
        Movement::Forward(x) => (pos.0 + x, pos.1),
        Movement::Down(x)    => (pos.0, pos.1 + x),
        Movement::Up(x)      => (pos.0, pos.1 - x),
    });
    let first = target_1.0 * target_1.1;

    // Now fold into (horizontal, depth, aim)
    let target_2 = course.iter().fold((0, 0, 0), |pos, movement| match movement {
        Movement::Forward(x) => (pos.0 + x, pos.1 + pos.2*x, pos.2),
        Movement::Down(x)    => (pos.0, pos.1, pos.2 + x),
        Movement::Up(x)      => (pos.0, pos.1, pos.2 - x),
    });
    let second = target_2.0 * target_2.1;
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
        assert_eq!(run(&sample_input), Ok((150, 900)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((1714950, 1281977850)));
    }
}
