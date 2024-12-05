use core::fmt::Display;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::num::ParseIntError;

type Page = u16;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    InputMalformed,
    ParseIntError(std::num::ParseIntError),
    LineMalformed(&'a str),
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputMalformed => write!(f, "Input must consist of the page ordering rules, an empty line, and then the print queues"),
            Self::LineMalformed(e) => write!(f, "Rules must consist of two integers, seperated by a '|'. Offending rule: {e}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

struct PageOrderingRules {
    rules: HashSet<(Page, Page)>,
}

impl<'a> TryFrom<&'a str> for PageOrderingRules {
    type Error = ParseError<'a>;

    fn try_from(value: &'a  str) -> Result<Self, Self::Error> {
        let rules = value.lines().map(|line| {
            if let Some((first, then)) = line.split_once('|') {
                let first = first.parse::<Page>()?;
                let then = then.parse::<Page>()?;
                Ok((first, then,))
            } else {
                Err(Self::Error::LineMalformed(value))
            }
        }).collect::<Result<HashSet<_>, _>>()?;
        Ok(Self { rules, })
    }
}

impl PageOrderingRules {
    fn is_ordered(&self, lhs: &Page, rhs: &Page) -> bool {
        !self.rules.contains(&(*rhs, *lhs))
    }

    fn is_sorted(&self, queue: &[Page]) -> bool {
        queue.is_sorted_by(|a, b| self.is_ordered(a, b))
    }

    fn ordering(&self, lhs: &Page, rhs: &Page) -> Ordering {
        if self.is_ordered(lhs, rhs) {
            Ordering::Less
        } else if self.is_ordered(lhs, rhs) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

pub fn run(input: &str) -> Result<(Page, Page), ParseError> {
    if let Some((rules, queues)) = input.split_once("\n\n") {
        let rules = PageOrderingRules::try_from(rules)?;
        let mut queues: Vec<_> = queues.lines().map(|line| line.split(',').map(|n| n.parse::<Page>()).collect::<Result<Vec<_>, _>>()).collect::<Result<Vec<_>, _>>()?;
        let first = queues.iter().filter(|q| rules.is_sorted(q)).map(|q| q[q.len()/2]).sum();
        let second = queues.iter_mut().filter(|q| !rules.is_sorted(q)).map(|q| {
            q.sort_by(|a, b| rules.ordering(a, b));
            q[q.len()/2]
        }).sum();
        Ok((first, second))
    } else {
        Err(ParseError::InputMalformed)
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
        assert_eq!(run(&sample_input), Ok((143, 123)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((4774, 6004)));
    }
}
