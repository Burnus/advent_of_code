use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidShape,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidShape => write!(f, "Input must be 5 chars wide and 7 chars tall"),
        }
    }
}

fn try_parse_inputs(input: &str) -> Result<(Vec<[u8; 5]>, Vec<[u8; 5]>), ParseError> {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for item in input.split("\n\n") {
        let bytes = item.as_bytes();
        if bytes.len() != 41 {
            return Err(ParseError::InvalidShape)
        }
        let key = !bytes[..5].contains(&b'.');
        let mut this = [0; 5];
        if key {
            // The substraction never underflows because entering this branch means bytes[0..5]
            // doesn't contain a b'.'
            (0..5).for_each(|col| this[col] = bytes.iter().skip(col).step_by(6).position(|&b| b == b'.').unwrap_or(7) as u8 - 1);
            keys.push(this);
        } else {
            // The substraction never underflows because bytes has len() 41, so position() cannot
            // return anything above 7.
            (0..5).for_each(|col| this[col] = 7 - bytes.iter().skip(col).step_by(6).position(|&b| b != b'.').unwrap_or(6) as u8);
            locks.push(this);
        }
    }

    Ok((locks, keys))
}

fn lock_matches_key(lock: &[u8; 5], key: &[u8; 5]) -> bool {
    lock.iter().zip(key.iter()).all(|(l_pin, r_pin)| l_pin + r_pin <= 6)
}

pub fn run(input: &str) -> Result<usize, ParseError> {
    let (locks, keys) = try_parse_inputs(input)?;
    let first = locks
        .iter()
        .map(|l| keys.iter().filter(|k| lock_matches_key(l, k)).count())
        .sum();
    Ok(first)
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
        assert_eq!(run(&sample_input), Ok(3));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok(3307));
    }
}
