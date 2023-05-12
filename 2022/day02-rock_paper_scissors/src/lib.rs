use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedToken(Option<char>),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(Some(c)) => write!(f, "Unexpected Hand encountered: {c}"),
            Self::UnexpectedToken(None) => write!(f, "Hand doesn't contain enough parameters"),
        }
    }
}

#[derive(Clone, Copy)]
enum Strategy {
    One,
    Two,
}

#[derive(Copy, Clone)]
enum Hand {
    Rock = 1, 
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<u8> for Hand {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Rock),
            2 => Ok(Self::Paper),
            3 => Ok(Self::Scissors),
            n => Err(Self::Error::UnexpectedToken(Some(n as char))),
        }
    }
}

impl Hand {
    fn that_beats(other: Self) -> Self {
        Hand::try_from((other as u8) % 3 + 1).unwrap()
    }
    fn that_is_beaten_by(other: Self) -> Self {
        Hand::try_from((other as u8 + 1) % 3 + 1).unwrap()
    }
}

enum Outcome { Win, Loss, Draw }

struct Round {
    opponent_hand: Hand,
    player_hand: Hand,
}

impl Round {
    fn outcome(&self) -> Outcome {
        match self.player_hand as i8 - self.opponent_hand as i8 {
            0 => Outcome::Draw,
            1 | -2 => Outcome::Win,
            _ => Outcome::Loss,
        }
    }
    fn points(&self) -> u32 {
        self.player_hand as u32 + match self.outcome() {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

fn try_parse_round(line: &str, strat: Strategy) -> Result<Round, ParseError> {
    let mut line = line.chars();
    let opponent_hand = match line.next() {
        Some('A') => Ok(Hand::Rock),
        Some('B') => Ok(Hand::Paper),
        Some('C') => Ok(Hand::Scissors),
        c => Err(ParseError::UnexpectedToken(c)),
    }?;
    let player_hand = match strat {
        Strategy::One => {
            match line.nth(1) {
                Some('X') => Ok(Hand::Rock),
                Some('Y') => Ok(Hand::Paper),
                Some('Z') => Ok(Hand::Scissors),
                c => Err(ParseError::UnexpectedToken(c)),
            }
            },
        Strategy::Two => {
            match line.nth(1) {
                Some('X') => Ok(Hand::that_is_beaten_by(opponent_hand)),
                Some('Y') => Ok(opponent_hand),
                Some('Z') => Ok(Hand::that_beats(opponent_hand)),
                c => Err(ParseError::UnexpectedToken(c)),
            }
        },
    }?;
    Ok(Round { opponent_hand, player_hand })
}

fn get_tally(moves: &str, strat: Strategy) -> Result<u32, ParseError> {
    moves.lines()
        .filter(|l| l.len() == 3)
        .map(|l| try_parse_round(l, strat).map(|r| r.points()))
        .sum::<Result<u32, _>>()
}

pub fn run(input: &str) -> Result<(u32, u32), ParseError> {
    let first = get_tally(input, Strategy::One)?;
    let second = get_tally(input, Strategy::Two)?;
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
        assert_eq!(run(&sample_input), Ok((15, 12)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((12458, 12683)));
    }
}
