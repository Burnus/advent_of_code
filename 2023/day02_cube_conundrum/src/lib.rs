use core::fmt::Display;
use std::num::ParseIntError;

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

struct Game {
    id: usize,
    min_cubes: [u8; 3],
}

impl<'a> TryFrom<&'a str> for Game {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let parts: Vec<_> = value.split(&[':',';']).collect();
        if parts.len() < 2 {
            return Err(Self::Error::LineMalformed(value));
        }
        let id = parts[0].split_whitespace().last().ok_or(Self::Error::LineMalformed(value))?.parse()?;
        let mut min_cubes = [0; 3];
        for reveal in parts.iter().skip(1) {
            let pairs: Vec<_> = reveal.split(", ").collect();
            for pair in pairs {
                let elements: Vec<_> = pair.split_whitespace().collect();
                if elements.len() != 2 {
                    return Err(Self::Error::LineMalformed(value));
                }
                let count = elements[0].parse()?;
                let colour_id = match elements[1].chars().next() {
                    Some('r') => Ok(0),
                    Some('g') => Ok(1),
                    Some('b') => Ok(2),
                    _ => Err(Self::Error::LineMalformed(value)),
                }?;
                min_cubes[colour_id] = min_cubes[colour_id].max(count);
            }
        }
        Ok(Self {
            id,
            min_cubes,
        })
    }
}

impl Game {
    fn validate(&self, max_dice: [u8; 3]) -> bool {
        max_dice.iter().enumerate().all(|(id, count)| self.min_cubes[id] <= *count)
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let items: Vec<_> = input.lines().map(Game::try_from).collect::<Result<Vec<_>, _>>()?;
    let first = items.iter().filter(|g| g.validate([12, 13, 14])).map(|g| g.id).sum();
    let second = items.iter().map(|g| g.min_cubes.iter().map(|c| *c as usize).product::<usize>()).sum();
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
        assert_eq!(run(&sample_input), Ok((8, 2286)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((2101, 58269)));
    }
}
