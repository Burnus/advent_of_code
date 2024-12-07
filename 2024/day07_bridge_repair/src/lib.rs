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
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}\nShould be of format \"190: 10 19\""),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

struct Calibration {
    result: usize,
    operands: Vec<usize>,
}

impl<'a> TryFrom<&'a str> for Calibration {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some((result, operands)) = value.split_once(": ") {
            let result = result.parse::<usize>()?;
            let operands: Vec<_> = operands.split_whitespace().map(|op| op.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
            Ok(Self { result, operands, })
        } else {
            Err(Self::Error::LineMalformed(value))
        }
    }
}

impl Calibration {
    fn combine(so_far: usize, operands: &[usize], target: usize, with_concat: bool) -> bool {
        if so_far > target {
            false
        } else if operands.is_empty() {
            so_far == target
        } else {
            Self::combine(so_far + operands[0], &operands[1..], target, with_concat) ||
            Self::combine(so_far * operands[0], &operands[1..], target, with_concat) ||
            with_concat && Self::combine(
                so_far * 10_usize.pow(operands[0].ilog10() + 1) + operands[0], 
                &operands[1..], 
                target, 
                with_concat)
        }
    }
    fn can_evaluate(&self, with_concat: bool) -> bool {
        Self::combine(self.operands[0], &self.operands[1..], self.result, with_concat)
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let calibrations: Vec<_> = input.lines().map(Calibration::try_from).collect::<Result<Vec<_>, _>>()?;
    let first = calibrations.iter().filter(|c| c.can_evaluate(false)).map(|c| c.result).sum();
    let second = calibrations.iter().filter(|c| c.can_evaluate(true)).map(|c| c.result).sum();
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
        assert_eq!(run(&sample_input), Ok((3749, 11387)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((1153997401072, 97902809384118)));
    }
}
