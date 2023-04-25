use core::fmt::Display;
use std::{num::ParseIntError, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidInput(String),
    LineMalformed(String),
    ParseIntError(std::num::ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(v) => write!(f, "Input is invalid: {v}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

struct Rule {
    left: u8,
    right: u8,
    insert: u8,
}

impl TryFrom<&str> for Rule {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((condition, insert)) = value.split_once(" -> ").map(|(c, i)| (c.as_bytes(), i.as_bytes())) {
            if condition.len() == 2 && insert.len() == 1 {
                Ok(Self {
                    left: condition[0],
                    right: condition[1],
                    insert: insert[0],
                })
            } else {
                Err(Self::Error::LineMalformed(value.to_string()))
            }
        } else {
            Err(Self::Error::LineMalformed(value.to_string()))
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    if let Some((template, rules)) = input.split_once("\n\n") {

        // We don't actually care about the order of elements, except for their immediate
        // neighbours. So we are fine splitting the polymer into windows of size 2 and just
        // tallying up how often a given pairing occurs. This speeds things up significantly, once
        // the polymer grows large and the pairings occur multiple times.
        let mut polymer = HashMap::new();
        template.as_bytes().windows(2).for_each(|w| {
            polymer.entry((w[0], w[1])).and_modify(|count| *count += 1).or_insert(1);
        });

        let rules: Vec<_> = rules.lines().map(Rule::try_from).collect::<Result<Vec<_>, _>>()?;
        for _ in 0..10 {
            polymerize(&mut polymer, &rules);
        }
        let elements = count_elements(&polymer);
        let first = elements.values().max().unwrap_or(&0) - elements.values().min().unwrap_or(&0);

        for _ in 10..40 {
            polymerize(&mut polymer, &rules);
        }
        let elements = count_elements(&polymer);
        let second = elements.values().max().unwrap_or(&0) - elements.values().min().unwrap_or(&0);
        Ok((first, second))
    } else {
        Err(ParseError::InvalidInput("Unable to split into template and rules".to_string()))
    }
}

fn polymerize(polymer: &mut HashMap<(u8, u8), usize>, rules: &[Rule]) {
    let mut new = HashMap::new();
    polymer.iter().for_each(|(&(lhs, rhs), &pair_count)| {
        let insert = rules.iter().find(|r| r.left == lhs && r.right == rhs).map(|r| r.insert).unwrap();
        new.entry((lhs, insert)).and_modify(|count| *count += pair_count).or_insert(pair_count);
        new.entry((insert, rhs)).and_modify(|count| *count += pair_count).or_insert(pair_count);
    });
    std::mem::swap(&mut new, polymer);
}

fn count_elements(polymer: &HashMap<(u8, u8), usize>) -> HashMap<u8, usize> {
    let mut counts = HashMap::new();

    polymer.iter().for_each(|(&(lhs, rhs), &pair_count)| {
        counts.entry(lhs).and_modify(|count| *count += pair_count).or_insert(pair_count);
        counts.entry(rhs).and_modify(|count| *count += pair_count).or_insert(pair_count);
    });

    // We have counted every element twice so far, except for the very first and last one, which
    // have been counted twice minus one (because they were lhs or rhs once less than if they'd
    // been in the middle). Divide by 2, rounding up, to accomodate for that.
    counts.iter_mut().for_each(|(_elem, count)| *count = (*count+1) / 2);

    counts
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
        assert_eq!(run(&sample_input), Ok((1588, 2188189693529)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((2408, 2651311098752)));
    }
}
