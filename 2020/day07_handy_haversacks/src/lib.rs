use core::fmt::Display;
use std::{num::ParseIntError, collections::{HashMap, HashSet}};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ParseIntError(std::num::ParseIntError),
    LineMalformed(String),
    MissingEntry(String),
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
            Self::MissingEntry(e) => write!(f, "Required entry not found: {e}"),
        }
    }
}

struct Rules {
    must_contain: Vec<Vec<(usize, usize)>>,
    shiny_gold_id: usize,
}

impl TryFrom<&str> for Rules {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut ids = HashMap::new();
        let mut must_contain = Vec::new();

        for line in value.lines() {
            let words: Vec<_> = line.split_whitespace().collect();
            if words.len() < 7 || words[2] != "bags" {
                return Err(Self::Error::LineMalformed(line.to_string()));
            }
            let mut next_id = ids.len();
            let mut this_rule = Vec::new();
            let this_id = *ids.entry(words[0].to_string() + " " + words[1]).or_insert(next_id);

            match words.len() {
                7 => (),
                n if n % 4 == 0 => {
                    for part in words.chunks(4).skip(1) {
                        next_id = ids.len();
                        let count = part[0].parse::<usize>()?;
                        let that_id = *ids.entry(part[1].to_string() + " " + part[2]).or_insert(next_id);
                        this_rule.push((that_id, count));
                    }
                },
                _ => return Err(Self::Error::LineMalformed(line.to_string())),
            }
            while must_contain.len() <= next_id {
                must_contain.push(Vec::new());
            }
            must_contain[this_id] = this_rule;
        }
        if let Some(&shiny_gold_id) = ids.get("shiny gold") {
            Ok(Self { must_contain, shiny_gold_id })
        } else {
            Err(Self::Error::MissingEntry("shiny gold".to_string()))
        }
    }
}

impl Rules {
    fn outside_recursive(&self, id: usize) -> HashSet<usize> {
        let direct: HashSet<usize> = self.must_contain.iter()
                                            .enumerate()
                                            .filter(|(_idx, rules)| rules.iter()
                                                .any(|(cid, _count)| *cid == id))
                                            .map(|(idx, _rules)| idx)
                                            .collect();
        let mut recursive = direct.clone();
        direct.iter().for_each(|idx| {
            self.outside_recursive(*idx).into_iter().for_each(|next| {
                recursive.insert(next);
            });
        });
        recursive
    }

    fn inside_recursive(&self, id: usize) -> usize {
        self.must_contain[id].iter()
            .map(|(inner, count)| *count * (1+self.inside_recursive(*inner)))
            .sum() 
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let rules = Rules::try_from(input)?;
    let first = rules.outside_recursive(rules.shiny_gold_id).len();
    let second = rules.inside_recursive(rules.shiny_gold_id);
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
        assert_eq!(run(&sample_input), Ok((4, 32)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((348, 18885)));
    }
}
