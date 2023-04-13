use core::fmt::Display;
use std::{num::ParseIntError, collections::{HashSet, BTreeMap}};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InputMalformed(String),
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
            Self::InputMalformed(v) => write!(f, "Input doesn't consist of 2 parts seperated by an empty line: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

struct Rules {
    char_replacements: Vec<(char, usize)>,
    reductions: BTreeMap<usize, Vec<Vec<usize>>>,
    beginnings: HashSet<Vec<usize>>,
    ends: HashSet<Vec<usize>>,
}

impl TryFrom<&str> for Rules {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut char_replacements = Vec::new();
        let mut reductions = BTreeMap::new();
        let beginnings = HashSet::from([Vec::from([42])]);
        let ends = HashSet::from([Vec::from([31])]);
        
        for line in value.lines() {
            let components: Vec<_> = line.split_whitespace().collect();
            if components.len() < 2 {
                return Err(Self::Error::LineMalformed(line.to_string()));
            }
            let rule_idx = components[0][..components[0].len()-1].parse::<usize>()?;
            if components[1].chars().next() == Some('"') {
                if let Some(c) = components[1].chars().nth(1) {
                    char_replacements.push((c, rule_idx));
                } else {
                    return Err(Self::Error::LineMalformed(line.to_string()));
                }
            } else {
                let mut variants = Vec::new();
                let partitions: Vec<_> = components.iter().enumerate().filter(|(_idx, c)| c == &&"|").map(|(idx, _c)| idx).collect();
                if partitions.is_empty() {
                    variants.push(components.iter().skip(1).map(|i| i.parse::<usize>()).collect::<Result<Vec<_>, _>>()?);
                } else {
                    let mut last_part_idx = 0;
                    for part_idx in partitions {
                        variants.push(components.iter().skip(last_part_idx+1).take(part_idx-last_part_idx-1).map(|i| i.parse::<usize>()).collect::<Result<Vec<_>, _>>()?);
                        last_part_idx = part_idx;
                    }
                    variants.push(components.iter().skip(last_part_idx+1).map(|i| i.parse::<usize>()).collect::<Result<Vec<_>, _>>()?);
                }
                reductions.insert(rule_idx, variants);
            }
        }

        Ok(Self { 
            char_replacements,
            reductions,
            beginnings,
            ends,
        })
    }
}

impl Rules {
    fn find_beginnings(&mut self) {
        let mut open_set = self.beginnings.iter().cloned().collect::<Vec<_>>();
        let mut visited = self.beginnings.clone();
        let mut beginnings = HashSet::new();
        while let Some(current) = open_set.pop() {
            if current.iter().all(|n| self.char_replacements.iter().any(|(_c, i)| n == i)) {
                beginnings.insert(current);
            } else {
                for (idx, rule) in current.iter().enumerate() {
                    if let Some(expansions) = self.reductions.get(rule) {
                        for exp in expansions {
                            let mut next = current[..idx].to_vec();
                            next.append(&mut exp.to_vec());
                            next.append(&mut current[idx+1..].to_vec());
                            if !visited.contains(&next) {
                                visited.insert(next.to_vec());
                                open_set.push(next);
                            }
                        }
                    }
                }
            }
        }
        self.beginnings = beginnings;
    }

    fn find_ends(&mut self) {
        let mut open_set = self.ends.iter().cloned().collect::<Vec<_>>();
        let mut visited = self.ends.clone();
        let mut ends = HashSet::new();
        while let Some(current) = open_set.pop() {
            if current.iter().all(|n| self.char_replacements.iter().any(|(_c, i)| n == i)) {
                ends.insert(current);
            } else {
                for (idx, rule) in current.iter().enumerate() {
                    if let Some(expansions) = self.reductions.get(rule) {
                        for exp in expansions {
                            let mut next = current[..idx].to_vec();
                            next.append(&mut exp.to_vec());
                            next.append(&mut current[idx+1..].to_vec());
                            if !visited.contains(&next) {
                                visited.insert(next.to_vec());
                                open_set.push(next);
                            }
                        }
                    }
                }
            }
        }
        self.ends = ends;
    }

    // All valid messages have the format 42 42 31 for part 1, or 42+ 42{n} 31{n} for part 2 (where
    // both n's are equal and greater than zero. Hence we can split the message up and check if it
    // consists of 2 valid beginning parts (expansions of rule 42) and 1 valid ending (expansions
    // of rule 31). For part 2 we allow additional beginnings and endings between those parts, so
    // effectively any number of beginnings and endings (in that order), where the number of
    // endings is at least 1 and the number of beginnings is at least 1 more than that.
    fn is_valid(&mut self, message: &str, part_2: bool) -> bool {
        let target: Vec<usize> = message.chars().map(|c| self.char_replacements.iter().find(|(rule, _idx)| rule == &c).unwrap().1).collect();
        let mut targets = Vec::new();
        let beginnings: Vec<_> = self.beginnings.iter().filter(|b| target.starts_with(b)).collect();
        if beginnings.is_empty() {
            return false;
        } else {
            for beginning in beginnings {
                targets.push(target[beginning.len()..].to_vec());
            }
        }
        let mut new_targets = Vec::new();
        for target in &targets {
            let beginnings: Vec<_> = self.beginnings.iter().filter(|b| target.starts_with(b)).collect();
            if beginnings.is_empty() {
                return false;
            } else {
                for beginning in beginnings {
                    new_targets.push(target[beginning.len()..].to_vec());
                }
            }
        }
        std::mem::swap(&mut targets, &mut new_targets);
        let endings: Vec<_> = self.ends.iter().filter(|e| target.ends_with(e)).collect();
        if endings.is_empty() {
            return false;
        } else {
            let mut new_targets = Vec::new();
            for ending in endings {
                for target in &targets {
                    new_targets.push(target[..target.len()-ending.len()].to_vec());
                }
            }
            std::mem::swap(&mut targets, &mut new_targets);
        }
        if part_2 {
            while let Some(current) = targets.pop() {
                if current.is_empty() {
                    return true;
                }
                // Ensure we reduce the beginning and end the same number of times to make sure we
                // reduce the beginnings at least as often as the endings. We can still reduce
                // the beginning even more in the else case.
                let endings: Vec<_> = self.ends.iter().filter(|e| current.ends_with(e)).collect();
                if !endings.is_empty() {
                    for ending in endings {
                        let current = &current[..current.len()-ending.len()];
                        let beginnings: Vec<_> = self.beginnings.iter().filter(|b| current.starts_with(b)).collect();
                        if !beginnings.is_empty() {
                            for beginning in beginnings {
                                targets.push(current[beginning.len()..].to_vec());
                            }
                        }
                    }
                } else {
                    // We only ever enter this branch if we have (a) exhausted all endings, in
                    // which case we might as well retry searching for them -- they won't become
                    // available again since we don't touch the end, or (b) didn't get a fitting
                    // beginning. In that latter case, we either (ba) don't have a beginning at
                    // all, so we fail here as well, or (bb) have one without ending. That however
                    // breaks down into (bba) we will find a chain of valid beginnings, so we
                    // should pass, or (bbb) we don't, in which case we will fail by removing from
                    // the front.
                    let beginnings: Vec<_> = self.beginnings.iter().filter(|b| current.starts_with(b)).collect();
                    if !beginnings.is_empty() {
                        for beginning in beginnings {
                            targets.push(current[beginning.len()..].to_vec());
                        }
                    }
                }
            }
            return false;
        } else {
            targets.iter().any(|target| target.is_empty())
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    if let Some((rules_str, msg_str)) = input.split_once("\n\n") {
        let mut rules = Rules::try_from(rules_str)?;
        rules.find_beginnings();
        rules.find_ends();
        let messages: Vec<_> = msg_str.lines().collect();
        let first = messages.iter().filter(|m| rules.is_valid(m, false)).count();
        let second = messages.iter().filter(|m| rules.is_valid(m, true)).count();
        Ok((first, second))
    } else {
        Err(ParseError::InputMalformed(input.to_string()))
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
        assert_eq!(run(&sample_input), Ok((3, 12)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((200, 407)));
    }
}
