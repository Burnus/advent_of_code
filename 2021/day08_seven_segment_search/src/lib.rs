use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    LineMalformed(String),
    ParseCharError(char),
    PatternMatchingError(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseCharError(c) => write!(f, "Unable to parse into integer: {c}"),
            Self::PatternMatchingError(p) => write!(f, "Error while trying to match the pattern: {p}"),
        }
    }
}

const A_MASK: u8 = 0b0000001;
const B_MASK: u8 = 0b0000010;
const C_MASK: u8 = 0b0000100;
const D_MASK: u8 = 0b0001000;
const E_MASK: u8 = 0b0010000;
const F_MASK: u8 = 0b0100000;
const G_MASK: u8 = 0b1000000;

type DigitMask = u8;

struct Entry {
    signal_pattern: [DigitMask; 10],
    output_value: [DigitMask; 4],
}

impl TryFrom<&str> for Entry {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((signal_pattern, output_value)) = value.split_once(" | ") {
            let signals: Vec<_> = signal_pattern.split_whitespace().collect();
            let outputs: Vec<_> = output_value.split_whitespace().collect();
            if signals.len() != 10 || outputs.len() != 4 {
                return Err(Self::Error::LineMalformed(value.to_string()));
            }
            let mut signal_pattern = [0; 10];
            for idx in 0..10 {
                signal_pattern[idx] = to_digit_mask(signals[idx])?;
            }
            let mut output_value = [0; 4];
            for idx in 0..4 {
                output_value[idx] = to_digit_mask(outputs[idx])?;
            }

            Ok(Self { signal_pattern, output_value, })
        } else {
            Err(Self::Error::LineMalformed(value.to_string()))
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let entries: Vec<_> = input.lines().map(Entry::try_from).collect::<Result<Vec<_>, _>>()?;
    entries.iter().map(decode).reduce(|acc, cur| match (acc, cur) {
            (Ok(acc), Ok(cur)) => Ok((acc.0+cur.0, acc.1+cur.1)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }).unwrap()
}

fn to_digit_mask(value: &str) -> Result<DigitMask, ParseError> {
    value.chars()
         .map(|c| match c {
                'a' => Ok(A_MASK),
                'b' => Ok(B_MASK),
                'c' => Ok(C_MASK),
                'd' => Ok(D_MASK),
                'e' => Ok(E_MASK),
                'f' => Ok(F_MASK),
                'g' => Ok(G_MASK),
                _ => Err(ParseError::ParseCharError(c)),
            }).reduce(|acc, cur| match (acc, cur) {
                (Ok(acc), Ok(cur)) => Ok(acc | cur),
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }).unwrap()
}

fn decode(entry: &Entry) -> Result<(usize, usize), ParseError> {
    let cf = *entry.signal_pattern.iter().find(|n| n.count_ones() == 2).ok_or(ParseError::PatternMatchingError("Signal pattern doesn't contain a 2-segment value (needed for digit 1.)".to_string()))?;
    let acf = *entry.signal_pattern.iter().find(|n| n.count_ones() == 3).ok_or(ParseError::PatternMatchingError("Signal pattern doesn't contain a 3-segment value (needed for digit 3.)".to_string()))?;
    let bcdf = *entry.signal_pattern.iter().find(|n| n.count_ones() == 4).ok_or(ParseError::PatternMatchingError("Signal pattern doesn't contain a 4-segment value (needed for digit 4.)".to_string()))?;

    let bd = bcdf - cf;


    let digit_patterns = [
    	*entry.signal_pattern.iter().find(|n|n.count_ones() == 6 && *n & bd < bd).ok_or(ParseError::PatternMatchingError("Unable to find a pattern that fits digit 0.".to_string()))?,
    	cf,
    	*entry.signal_pattern.iter().find(|n|n.count_ones() == 5 && *n & cf < cf && *n & bd < bd).ok_or(ParseError::PatternMatchingError("Unable to find a pattern that fits digit 2.".to_string()))?,
    	*entry.signal_pattern.iter().find(|n|n.count_ones() == 5 && *n & cf == cf).ok_or(ParseError::PatternMatchingError("Unable to find a pattern that fits digit 3.".to_string()))?,
    	bcdf,
    	*entry.signal_pattern.iter().find(|n|n.count_ones() == 5 && *n & bd == bd).ok_or(ParseError::PatternMatchingError("Unable to find a pattern that fits digit 5.".to_string()))?,
    	*entry.signal_pattern.iter().find(|n|n.count_ones() == 6 && *n & cf < cf).ok_or(ParseError::PatternMatchingError("Unable to find a pattern that fits digit 6.".to_string()))?,
    	acf,
    	0b1111111,
    	*entry.signal_pattern.iter().find(|n|n.count_ones() == 6 && *n & bcdf == bcdf).ok_or(ParseError::PatternMatchingError("Unable to find a pattern that fits digit 9.".to_string()))?,
    ];

    let number = (0..4).map(|idx| digit_patterns.iter().position(|pat| *pat == entry.output_value[idx]).ok_or(ParseError::PatternMatchingError(format!("Unknown pattern in output: {val}", val = entry.output_value[idx]))).map(|n| n * 10_usize.pow(3-idx as u32))).sum::<Result<usize, ParseError>>()?;
    Ok((entry.output_value.iter().filter(|o| [2, 3, 4, 7].contains(&o.count_ones())).count(), number))
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
        assert_eq!(run(&sample_input), Ok((26, 61229)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((367, 974512)));
    }
}
