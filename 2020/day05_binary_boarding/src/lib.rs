use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    LineMalformed(String),
    InvalidRowChar(char),
    InvalidColChar(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Boarding Pass is malformed: {v}"),
            Self::InvalidRowChar(c) => write!(f, "Invalid Character in Row Part: {c} (Should be 'B' or 'F')."),
            Self::InvalidColChar(c) => write!(f, "Invalid Character in Col Part: {c} (Should be 'L' or 'R')."),
        }
    }
}

struct Pass {
    row: usize,
    col: usize,
}

impl TryFrom<&str> for Pass {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(partition) = value.find(['L', 'R']) {
            let bin_row: String = value[..partition].chars().map(|c| match c {
                'F' => Ok('0'),
                'B' => Ok('1'),
                _ => Err(Self::Error::InvalidRowChar(c)),
            }).collect::<Result<String, _>>()?;
            let bin_col: String = value[partition..].chars().map(|c| match c {
                'L' => Ok('0'),
                'R' => Ok('1'),
                _ => Err(Self::Error::InvalidColChar(c)),
            }).collect::<Result<String, _>>()?;
            Ok(Self {
                row: usize::from_str_radix(&bin_row, 2).unwrap(),
                col: usize::from_str_radix(&bin_col, 2).unwrap(),
            })
        } else {
            Err(Self::Error::LineMalformed(value.to_string()))
        }
    }
}

impl Pass {
    fn seat_id(&self) -> usize {
        self.row*8 + self.col
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut ids: Vec<_> = input.lines()
                                .map(|line| Pass::try_from(line).map(|pass| pass.seat_id()))
                                .collect::<Result<Vec<_>, _>>()?;
    ids.sort();
    let first = *ids.last().unwrap();
    let second = ids.windows(2).find(|w| w[1]-w[0] > 1).map(|w| w[0]+1).unwrap();
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
        assert_eq!(run(&sample_input), Ok((820, 120)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((928, 610)));
    }
}
