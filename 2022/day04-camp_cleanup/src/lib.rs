use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    LineMalformed(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

fn fully_contained(pair: &str) -> Result<bool, ParseError> {
    let ((l1, r1), (l2, r2)) = parse_into_tuples(pair)?;
    Ok(l1<=l2 && r1>=r2 || l2<=l1 && r2>=r1)
}

fn overlapping(pair: &str) -> Result<bool, ParseError> {
    let ((l1, r1), (l2, r2)) = parse_into_tuples(pair)?;
    Ok(l1<=l2 && r1>=l2 || l2<=l1 && r2>=l1)
}

fn parse_into_tuples(pair: &str) -> Result<((u32, u32), (u32, u32)), ParseError> {
    if let Some((first, second)) = pair.split_once(',') {
        if let Some ((l1, r1)) = first.split_once('-') {
            if let Some((l2, r2)) = second.split_once('-') {
                let l1 = l1.parse::<u32>().expect("Malformed ID: Not a number");
                let l2 = l2.parse::<u32>().expect("Malformed ID: Not a number");
                let r1 = r1.parse::<u32>().expect("Malformed ID: Not a number");
                let r2 = r2.parse::<u32>().expect("Malformed ID: Not a number");
                return Ok(((l1, r1), (l2, r2)));
            }
        }
    }
    Err(ParseError::LineMalformed(pair.to_string()))
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let first = input.lines().map(fully_contained).collect::<Result<Vec<_>, _>>()?.iter().filter(|l| **l).count();
    let second = input.lines().map(overlapping).collect::<Result<Vec<_>, _>>()?.iter().filter(|l| **l).count();
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
        assert_eq!(run(&sample_input), Ok((2, 4)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((560, 839)));
    }
}
