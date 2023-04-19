use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ParseIntError(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer {e}"),
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let bits: Vec<_> = input.lines().map(|line| line.chars().map(|i| i.to_digit(10).ok_or(ParseError::ParseIntError(i))).collect::<Result<Vec<_>, _>>()).collect::<Result<Vec<_>, _>>()?;
    let mut ones = vec![0; bits[0].len()];
    bits.iter().for_each(|number| number.iter().enumerate().for_each(|(idx, bit)| ones[idx] += bit));
    let half = bits.len() as u32/2;
    let mut gamma = 0;
    let mut epsilon = 0;
    ones.iter().for_each(|bit_count| {
        gamma *= 2;
        epsilon *= 2;
        if bit_count > &half {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    });
    let first = gamma as usize * epsilon as usize;

    let bit_count = bits.len();
    let mut oxygen_rating = bits.clone();
    for idx in 0..bit_count {
        if oxygen_rating.len() > 1 {
            if oxygen_rating.iter().map(|bits| bits[idx]).sum::<u32>() >= (oxygen_rating.len() as u32 + 1) / 2 {
                oxygen_rating.retain(|bits| bits[idx] == 1);
            } else {
                oxygen_rating.retain(|bits| bits[idx] == 0);
            }
        }
    }
    let oxygen_rating = oxygen_rating[0].iter().fold(0, |acc, bit| 2 * acc + *bit);

    let mut co2_rating = bits.clone();
    for idx in 0..bit_count {
        if co2_rating.len() > 1 {
            if co2_rating.iter().map(|bits| bits[idx]).sum::<u32>() >= (co2_rating.len() as u32 + 1) / 2 {
                co2_rating.retain(|bits| bits[idx] == 0);
            } else {
                co2_rating.retain(|bits| bits[idx] == 1);
            }
        }
    }
    let co2_rating = co2_rating[0].iter().fold(0, |acc, bit| 2 * acc + *bit);

    let second = oxygen_rating as usize * co2_rating as usize;
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
        assert_eq!(run(&sample_input), Ok((198, 230)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((2498354, 3277956)));
    }
}
