use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidChar(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChar(c) => write!(f, "Encountered invalid Character: {c}"),
        }
    }
}

#[derive(PartialEq)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angle,
}

impl Bracket {
    fn from(c: char) -> Self {
        match c {
            '(' | ')' => Self::Round,
            '[' | ']' => Self::Square,
            '{' | '}' => Self::Curly,
            '<' | '>' => Self::Angle,
            _ => panic!("Tried to cast Unexpected character {c} into Bracket."),
        }
    }

    fn corrupt_score(&self) -> usize {
        match self {
            Self::Round => 3,
            Self::Square => 57,
            Self::Curly => 1197,
            Self::Angle => 25137,
        }
    }

    fn incomplete_score(&self) -> usize {
        match self {
            Self::Round => 1,
            Self::Square => 2,
            Self::Curly => 3,
            Self::Angle => 4,
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let first = input.lines().map(|line| corrupted_score(line)).sum::<Result<usize, ParseError>>()?;
    let mut incomplete: Vec<_> = input.lines().map(|line| incomplete_score(line)).filter(|score| *score != Ok(0)).collect::<Result<Vec<_>, ParseError>>()?;
    incomplete.sort();
    let second = incomplete[incomplete.len()/2];
    Ok((first, second))
}

fn corrupted_score(line: &str) -> Result<usize, ParseError> {
    let mut to_close = Vec::new();

    for c in line.chars() {
        match c {
            o if is_opening(o) => to_close.push(Bracket::from(o)),
            c if is_closing(c) => {
                let c = Bracket::from(c);
                if let Some(o) = to_close.pop() {
                    if o != c {
                        return Ok(c.corrupt_score());
                    }
                } else {
                    return Ok(c.corrupt_score());
                }
            },
            e => return Err(ParseError::InvalidChar(e)),
        }
    }
    Ok(0)
}

fn incomplete_score(line: &str) -> Result<usize, ParseError> {
    let mut to_close = Vec::new();

    for c in line.chars() {
        match c {
            o if is_opening(o) => to_close.push(Bracket::from(o)),
            c if is_closing(c) => {
                let c = Bracket::from(c);
                if let Some(o) = to_close.pop() {
                    if o != c {
                        return Ok(0);
                    }
                } else {
                    return Ok(0);
                }
            },
            e => return Err(ParseError::InvalidChar(e)),
        }
    }
    let mut score = 0;
    while let Some(o) = to_close.pop() {
        score *= 5;
        score += o.incomplete_score();
    }
    Ok(score)
}

fn is_opening(c: char) -> bool {
    ['(', '[', '{', '<'].contains(&c)
}

fn is_closing(c: char) -> bool {
    [')', ']', '}', '>'].contains(&c)
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
        assert_eq!(run(&sample_input), Ok((26397, 288957)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((392367, 2192104158)));
    }
}
