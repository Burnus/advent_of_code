use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidChar(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChar(c) => write!(f, "Unexpected character: {c} should not be part of a SNAFU number."),
        }
    }
}

#[derive(Clone, Copy)]
enum SnafuDigit { Zero, One, Two, Minus, DoubleMinus }
struct Snafu(Vec<SnafuDigit>);

impl TryFrom<&str> for Snafu {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(value.chars().map(|c| match c {
            '0' => Ok(SnafuDigit::Zero),
            '1' => Ok(SnafuDigit::One),
            '2' => Ok(SnafuDigit::Two),
            '-' => Ok(SnafuDigit::Minus),
            '=' => Ok(SnafuDigit::DoubleMinus),
            c => Err(Self::Error::InvalidChar(c)),
        }).collect::<Result<Vec<_>, _>>()?))
    }
}

impl From<SnafuDigit> for char {
    fn from(value: SnafuDigit) -> Self {
        match value {
            SnafuDigit::Zero => '0',
            SnafuDigit::One => '1',
            SnafuDigit::Two => '2',
            SnafuDigit::Minus => '-',
            SnafuDigit::DoubleMinus => '=',
        }
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().cloned().map(char::from).collect::<String>())
    }
}

impl From<Snafu> for isize {
    fn from(value: Snafu) -> Self {
        let mut res = 0;

        for d in value.0 {
            res *= 5;
            match d {
                SnafuDigit::Zero => (),
                SnafuDigit::One => res += 1,
                SnafuDigit::Two => res += 2,
                SnafuDigit::Minus => res -= 1,
                SnafuDigit::DoubleMinus => res -= 2,
            }
        }

        res
    }
}

impl From<isize> for Snafu {
    fn from(value: isize) -> Self {
        let mut digits = Vec::new();
        let mut value = value;

        while value != 0 {
            let digit = value % 5;
            match digit {
                0 => digits.push(SnafuDigit::Zero),
                1 => digits.push(SnafuDigit::One),
                2 => digits.push(SnafuDigit::Two),
                3 => digits.push(SnafuDigit::DoubleMinus),
                4 => digits.push(SnafuDigit::Minus),
                _ => unreachable!("value%5 can only ever be one of the values above"),
            }
            if digit > 2 {
                value += 2;
            }
            value /= 5;
        }
        digits.reverse();

        Self(digits)
    }
}

pub fn run(input: &str) -> Result<String, ParseError> {
    let total = input.lines()
        .map(|s| Snafu::try_from(s).map(isize::from))
        .sum::<Result<isize, _>>()?;

    Ok(format!("{}", Snafu::from(total)))
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
        assert_eq!(run(&sample_input), Ok("2=-1=0".to_string()));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok("2-0=11=-0-2-1==1=-22".to_string()));
    }
}
