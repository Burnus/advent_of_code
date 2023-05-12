use std::collections::HashSet;
use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidToken(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken(t) => write!(f, "Invalid Item encountered: {t}"),
        }
    }
}

pub fn run(input: &str) -> Result<(u32, u32), ParseError> {
    let first = input.lines().map(duplicate_prio).sum::<Result<u32, ParseError>>()?;
    let second = get_badge_prios(input)?;
    Ok((first, second))
}

fn item_priority(item: char) -> Result<u32, ParseError> {
    match item {
        lc if lc.is_ascii_lowercase() => Ok(lc as u32 - 96),
        uc if uc.is_ascii_uppercase() => Ok(uc as u32 - 38),
        e => Err(ParseError::InvalidToken(e)),
    }
}

fn duplicate_prio(line: &str) -> Result<u32, ParseError> {
    if line.len() % 2 != 0 {
        panic!("Odd number of items!");
    }
    let comp1 = &line[..line.len()/2].chars().collect::<HashSet<char>>();
    let comp2 = &line[line.len()/2..].chars().collect::<HashSet<char>>();

    comp1.iter()
        .filter(|c| comp2.contains(*c))
        .map(|c| item_priority(*c))
        .sum()
}

fn badge_prio(e1: &str, e2: &str, e3: &str) -> Result<u32, ParseError> {
    e1.chars()
        .filter(|c| e2.contains(*c) && e3.contains(*c))
        .map(item_priority)
        .max_by(|a, b| match (a, b) {
            (Ok(a), Ok(b)) => a.cmp(b),
            (Ok(_), _err) => std::cmp::Ordering::Less,
            (_err, _) => std::cmp::Ordering::Greater,
        })
        .unwrap()
}

fn get_badge_prios(list: &str) -> Result<u32, ParseError> {
    let mut badge_prios = 0;
    let mut iter = list.lines();
    while let (Some(e1), Some(e2), Some(e3)) = (iter.next(), iter.next(), iter.next()) { 
        let this_badge = badge_prio(e1, e2, e3);
        badge_prios += this_badge?;
    }
    Ok(badge_prios)
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
        assert_eq!(run(&sample_input), Ok((157, 70)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((7746, 2604)));
    }
}
