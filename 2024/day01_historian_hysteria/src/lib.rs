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

struct Lists {
    lhs: Vec<usize>,
    rhs: Vec<usize>,
}

impl<'a> TryFrom<&'a str> for Lists {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut lhs = Vec::new();
        let mut rhs = Vec::new();
        for line in value.lines() {
            let elems: Vec<_> = line.split_whitespace().collect();
            if elems.len() == 2 {
            // if let Some((l, r)) = line.split_once("   ") {
                lhs.push(elems[0].parse::<usize>()?);
                rhs.push(elems[1].parse::<usize>()?);
            } else {
                return Err(Self::Error::LineMalformed(line));
            }
        }
        lhs.sort();
        rhs.sort();
        Ok(Self { lhs, rhs, })
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    // let lists: Vec<_> = input.lines().map(::try_from).collect::<Result<Vec<_>, _>>()?;
    let lists = Lists::try_from(input)?;
    let first = lists.lhs.iter().zip(lists.rhs.iter()).map(|(l, r)| l.abs_diff(*r)).sum();
    let second = lists.lhs.iter().map(|l| l * lists.rhs.iter().filter(|r| l == *r).count()).sum();
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
        assert_eq!(run(&sample_input), Ok((11, 31)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((0, 0)));
    }
}
