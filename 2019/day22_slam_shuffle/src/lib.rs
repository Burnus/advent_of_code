use core::fmt::Display;
use std::{num::ParseIntError, collections::VecDeque};

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

enum Technique {
    Rev,
    Shift(isize),
    Zip(usize),
}

impl TryFrom<&str> for Technique {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let components: Vec<_> = value.split_whitespace().collect();
        match components[1] {
            "into" => Ok(Self::Rev),
            "with" => Ok(Self::Zip(components[3].parse()?)),
            n => Ok(Self::Shift(n.parse()?)),
        }
    }
}

impl Technique {
    fn perform(&self, cards: &mut VecDeque<usize>) {
        let len = cards.len();
        match self {
            Self::Rev => (0..len/2).for_each(|a| cards.swap(a, len-a-1)),
            Self::Shift(i) if i >= &0 => cards.rotate_left(i.unsigned_abs()),
            Self::Shift(n) => cards.rotate_right(n.unsigned_abs()),
            Self::Zip(n) => {
                let mut new: VecDeque<usize> = cards.clone();
                (0..len).for_each(|idx| new[(n*idx)%len] = cards[idx]);
                std::mem::swap(cards, &mut new);
            },
        }
    }

    fn get_parameters(&self, offset_diff: u128, increment_mul: u128, modulo: u128) -> (u128, u128) {
        match self {
            Self::Rev => ((offset_diff+modulo-increment_mul)%modulo, modulo-increment_mul),
            Self::Shift(p) if p >= &0 => ((offset_diff+(p.unsigned_abs() as u128)*increment_mul)%modulo, increment_mul),
            Self::Shift(n) => ((offset_diff+(modulo-(n.unsigned_abs()) as u128)*increment_mul)%modulo, increment_mul),
            Self::Zip(i) => (offset_diff, (increment_mul*pow_mod(*i as u128, modulo-2, modulo))%modulo),
        }
    }
}

pub fn run_1(input: &str, cards: usize, target: usize) -> Result<usize, ParseError> {
    let mut cards: VecDeque<usize> = (0..cards).collect();
    let instructions: Vec<_> = input.lines().map(Technique::try_from).collect::<Result<Vec<_>, _>>()?;
        instructions.iter().for_each(|instr| instr.perform(&mut cards));
    Ok(cards.iter().position(|card| *card == target).unwrap())
}

pub fn run_2(input: &str, cards: u128, target: u128, iterations: u128) -> Result<u128, ParseError> {
    let mut offset_diff = 0;
    let mut increment_mul = 1;
    let instructions: Vec<_> = input.lines().map(Technique::try_from).collect::<Result<Vec<_>, _>>()?;
    for instruction in instructions {
        (offset_diff, increment_mul) = instruction.get_parameters(offset_diff, increment_mul, cards);
    }

    let increment = pow_mod(increment_mul, iterations, cards);
    let offset = (((cards*cards - offset_diff * (increment-1)) % cards) * pow_mod(cards+1-increment_mul, cards-2, cards))%cards;

    Ok((offset+target*increment)%cards)
}

fn pow_mod(mantisse: u128, exponent: u128, modulo: u128) -> u128 {
    match exponent {
        0 => 1,
        e if e % 2 == 0 => pow_mod((mantisse*mantisse) % modulo, exponent/2, modulo),
        _ => (mantisse * pow_mod((mantisse * mantisse) % modulo, exponent/2, modulo)) % modulo,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    fn pow_test() {
        for mantisse in 10..23 {
            for exponent in 5..14 {
                for modulo in (3..1_000_003).step_by(1_000) {
                    eprintln!("Testing {mantisse}^{exponent}%{modulo}");
                    assert_eq!(pow_mod(mantisse, exponent as u128, modulo), mantisse.pow(exponent)%modulo);
                }
            }
        }
    }

    #[test]
    fn perform_test() {
        let master: VecDeque<usize> = (0..10).collect();
        let reversed = VecDeque::from([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
        let shifted = VecDeque::from([3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
        let zipped = VecDeque::from([0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);

        let mut cards = master.clone();
        Technique::Rev.perform(&mut cards);
        assert_eq!(cards, reversed);

        let mut cards = master.clone();
        Technique::Shift(3).perform(&mut cards);
        assert_eq!(cards, shifted);

        let mut cards = master;
        Technique::Zip(3).perform(&mut cards);
        assert_eq!(cards, zipped);

    }

    #[test]
    fn get_parameters_samples() {
        assert_eq!(Technique::Rev.get_parameters(0, 1, 10), (9, 9));
        assert_eq!(Technique::Shift(3).get_parameters(0, 1, 10), (3, 1));
        assert_eq!(Technique::Shift(-4).get_parameters(0, 1, 10), (6, 1));
        assert_eq!(Technique::Zip(7).get_parameters(0, 1, 11), (0, 8));
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run_1(&sample_input, 10, 5), Ok(2));
    }

    #[test]
    fn test_challenge_part_1() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run_1(&challenge_input, 10007, 2019), Ok(8191));
        assert_eq!(run_2(&challenge_input, 10007, 2019, 1), Ok(1545));
    }

    #[test]
    fn test_challenge_part_2() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run_2(&challenge_input, 119315717514047, 2020, 101741582076661), Ok(1644352419829));
    }
}
