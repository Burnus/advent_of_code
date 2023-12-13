use core::fmt::Display;
use std::{num::ParseIntError, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    ParseIntError(std::num::ParseIntError),
    LineMalformed(&'a str),
    InvalidChar(char),
}

struct InvalidChar{ offending_char: char }

impl From<InvalidChar> for ParseError<'_> {
    fn from(value: InvalidChar) -> Self {
        Self::InvalidChar(value.offending_char)
    }
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChar(c) => write!(f, "Character {c} could not be converted into Spring Condition."),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Condition {
    type Error = InvalidChar;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            v => Err(Self::Error{offending_char: v}),
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::Operational => write!(f, "."),
            Condition::Damaged => write!(f, "#"),
            Condition::Unknown => write!(f, "?"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct Record {
    springs: Vec<Condition>,
    checksums: Vec<u8>,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut springs = String::new();
        let checksums = self.checksums.iter().map(|c| c.to_string()).fold(String::new(), |acc, a| acc + &a + ", ");
        self.springs.iter().for_each(|spring| springs += &spring.to_string());

        write!(f, "{springs} {checksums}")
    }
}

impl<'a> TryFrom<&'a str> for Record {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let components: Vec<_> = value.split_whitespace().collect();
        if components.len() != 2 {
            return Err(Self::Error::LineMalformed(value));
        }
        let springs = components[0].chars().map(Condition::try_from).collect::<Result<Vec<_>, _>>()?;
        let checksums = components[1].split(',').map(|s| s.parse()).collect::<Result<Vec<_>, _>>()?;

        Ok(Self { springs, checksums, })
    }
}

impl Record {
    fn possible_arrangements(&self, mem: &mut HashMap<Self, usize>) -> usize {
        if self.springs.iter().filter(|&spring| *spring != Condition::Operational).count() < self.checksums.iter().map(|c| *c as usize).sum() {
            return 0;
        }
        if self.springs.iter().filter(|&spring| *spring == Condition::Damaged).count() > self.checksums.iter().map(|c| *c as usize).sum() {
            return 0;
        }
        if let Some(res) = mem.get(self) {
            return *res;
        }
        match self.springs.first() {
            None => {
                    1
                },
            Some(Condition::Operational) => (Record { springs: self.springs[1..].to_vec(), checksums: self.checksums.to_vec(), }).possible_arrangements(mem),
            Some(Condition::Damaged) => if !self.checksums.is_empty() && self.checksums[0] > 0 {
                    let len = self.checksums[0] as usize;
                    if self.springs.iter().take(len).any(|&spring| spring == Condition::Operational) || self.springs.get(len) == Some(&Condition::Damaged) {
                        0
                    } else if len == self.springs.len() {
                        if self.checksums.len() == 1 { 1 } else { 0 }
                    } else {
                        let res = (Record { springs: self.springs[len+1..].to_vec(), checksums: self.checksums[1..].to_vec(), }).possible_arrangements(mem);
                        mem.insert(self.clone(), res);
                        res
                    }
                } else {
                    0
                }, 
            Some(Condition::Unknown) => {
                    let next_known = self.springs.iter().position(|&spring| spring != Condition::Unknown);
                    match next_known {
                        Some(idx) if self.springs[idx] == Condition::Operational => {
                                let mut sub_arrangements = (Record{ springs: self.springs[idx+1..].to_vec(), checksums: self.checksums.to_vec() }).possible_arrangements(mem);
                                for group_idx in 0..self.checksums.len() {
                                    if self.checksums.iter().take(group_idx+1).map(|d| *d as usize).sum::<usize>() + group_idx > idx {
                                        break;
                                    }
                                    sub_arrangements += get_distributions(idx, &self.checksums[..=group_idx]) * (Record { springs: self.springs[idx..].to_vec(), checksums: self.checksums[group_idx+1..].to_vec(), }).possible_arrangements(mem)
                                }
                                mem.insert(self.clone(), sub_arrangements);
                                sub_arrangements
                            },
                        Some(idx) => {
                                if self.checksums.len() == 1 {
                                    let len = self.checksums[0] as usize;
                                    let r_idx = self.springs.iter().rposition(|&spring| spring == Condition::Damaged).unwrap();
                                    let fixed_len = r_idx + 1 - idx;
                                    if fixed_len > len || self.springs.iter().skip(idx).take(r_idx+1-idx).any(|&spring| spring == Condition::Operational) {
                                        0
                                    } else {
                                        // todo: calculate this directly
                                        (0..=idx).filter(|&i| i+len > r_idx && i+len <= self.springs.len() && (r_idx..i+len).all(|j| self.springs[j] != Condition::Operational)).count()
                                    }
                                } else {
                                    let len = self.checksums[0] as usize;
                                    let res = (Record { springs: self.springs[1..].to_vec(), checksums: self.checksums.to_vec() }).possible_arrangements(mem) + 
                                        if self.springs.iter().take(len).any(|&spring| spring == Condition::Operational) || self.springs.get(len) == Some(&Condition::Damaged) {
                                            0
                                        } else if len == self.springs.len() {
                                            if self.checksums.len() == 1 { 1 } else { 0 }
                                        } else {
                                            (Record { springs: self.springs[len+1..].to_vec(), checksums: self.checksums[1..].to_vec(), }).possible_arrangements(mem)
                                        };
                                    mem.insert(self.clone(), res);
                                    res
                                }
                            },
                        None => {
                                get_distributions(self.springs.len(), &self.checksums)
                            },
                    }
                },
        }
    }

    fn unfold(&self) -> Self {
        let mut springs_part = self.springs.to_vec();
        springs_part.push(Condition::Unknown);
        let springs: Vec<_> = (0..5).flat_map(|_| springs_part.to_vec()).collect();
        let checksums = (0..5).flat_map(|_| self.checksums.to_vec()).collect();
        Self { springs: springs[..springs.len()-1].to_vec(), checksums, }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let records: Vec<_> = input.lines().map(Record::try_from).collect::<Result<Vec<_>, _>>()?;
    let first = records.iter().map(|r| r.possible_arrangements(&mut HashMap::new())).sum();
    let unfolded: Vec<_> = records.iter().map(Record::unfold).collect();
    let second = unfolded.iter().map(|r| r.possible_arrangements(&mut HashMap::new())).sum();
    Ok((first, second))
}

fn get_distributions(free: usize, to_distribute: &[u8]) -> usize {
    if to_distribute.is_empty() {
        1
    } else if free < to_distribute.iter().map(|d| *d as usize).sum::<usize>() + to_distribute.len() - 1 {
        0
    } else if to_distribute.len() == 1 && to_distribute[0] as usize == free {
        1
    } else {
        get_distributions(free-1, to_distribute) + get_distributions(free - to_distribute[0] as usize - 1, &to_distribute[1..])
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
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), Ok((21, 525152)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((7084, 8414003326821)));
    }
}
