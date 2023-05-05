use core::fmt::Display;
use std::{num::ParseIntError, collections::BTreeMap};

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

const WINNING_SCORE_V1: usize = 1000;
const WINNING_SCORE_V2: usize = 21;

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Player {
    score: usize,
    position: usize,
    won: bool,
}

impl TryFrom<&str> for Player {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (_, pos) = value.rsplit_once(' ').ok_or(Self::Error::LineMalformed(value.to_string()))?;
        Ok(Self { 
            position: pos.parse()?, 
            score: 0,
            won: false, 
        })
    }
}

impl Player {
    fn play(&mut self, die: &mut DeterministicDie) -> bool {
        self.roll(die.roll() + die.roll() + die.roll());
        if self.score >= WINNING_SCORE_V1 {
            self.won = true;
            true
        } else {
            false
        }
    }

    fn roll(&mut self, face: usize) {
        self.position = (self.position + face - 1) % 10 + 1;
        self.score += self.position;
    }
}

#[derive(Default)]
struct DeterministicDie {
    roll_counter: usize,
}

impl DeterministicDie {
    fn roll(&mut self) -> usize {
        self.roll_counter += 1;
        (self.roll_counter - 1) % 100 + 1
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut players: Vec<_> = input.lines().map(Player::try_from).collect::<Result<Vec<_>, _>>()?;
    let second = max_wins(&players);
    let mut die = DeterministicDie::default();
    'outer: loop {
        for player in players.iter_mut() {
            if player.play(&mut die) {
                break 'outer;
            }
        }
    }
    let first = players.iter().find(|p| !p.won).map(|p| p.score).unwrap() * die.roll_counter;
    Ok((first, second))
}

fn max_wins(players: &[Player]) -> usize {
    let mut wins = [0, 0];
    let mut open_set = BTreeMap::from([(players.to_vec(), 1)]);

    let roll_results = [
        (3, 1),
        (4, 3),
        (5, 6),
        (6, 7),
        (7, 6),
        (8, 3),
        (9, 1),
    ];

    while let Some(current) = open_set.pop_first() {
        let players = current.0;
        let count = current.1;
        for d1 in roll_results {
            let mut player_1 = players[0].clone();
            player_1.roll(d1.0);
            if player_1.score >= WINNING_SCORE_V2 {
                wins[0] += d1.1 * count;
            } else {
                for d2 in roll_results {
                    let mut player_2 = players[1].clone();
                    player_2.roll(d2.0);
                    if player_2.score >= WINNING_SCORE_V2 {
                        wins[1] += d1.1 * d2.1 * count;
                    } else {
                        open_set.entry(vec![player_1.clone(), player_2]).and_modify(|ct| *ct += count * d1.1 * d2.1).or_insert(count * d1.1 * d2.1);
                    }
                }
            }
        }
    }

    *wins.iter().max().unwrap()
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
        assert_eq!(run(&sample_input), Ok((739785, 444356092776315)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((518418, 116741133558209)));
    }
}
