use core::fmt::Display;
use std::{num::ParseIntError, collections::{VecDeque, HashSet}};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InputMalformed,
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
            Self::InputMalformed => write!(f, "Input is malformed"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let (player_1, player_2) = input.split_once("\n\n").ok_or(ParseError::InputMalformed)?;
    let player_1: VecDeque<_> = player_1.lines().skip(1).map(|i| i.parse::<usize>()).collect::<Result<VecDeque<_>, _>>()?;
    let player_2: VecDeque<_> = player_2.lines().skip(1).map(|i| i.parse::<usize>()).collect::<Result<VecDeque<_>, _>>()?;
    let first = score(play(&player_1, &player_2));
    let second = score(play_recursive(&player_1, &player_2));
    Ok((first, second))
}

fn play(player_1: &VecDeque<usize>, player_2: &VecDeque<usize>) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut player_1 = player_1.clone();
    let mut player_2 = player_2.clone();
    while !player_1.is_empty() && !player_2.is_empty() {
        let (card_1, card_2) = (player_1.pop_front().unwrap(), player_2.pop_front().unwrap());
        if card_1 > card_2 {
            player_1.push_back(card_1);
            player_1.push_back(card_2);
        } else {
            player_2.push_back(card_2);
            player_2.push_back(card_1);
        }
    }
    (player_1, player_2)
}

fn play_recursive(player_1: &VecDeque<usize>, player_2: &VecDeque<usize>) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut player_1 = player_1.clone();
    let mut player_2 = player_2.clone();
    let mut mem = HashSet::new();
    while !player_1.is_empty() && !player_2.is_empty() {
        if mem.contains(&(player_1.clone(), player_2.clone())) {
            return (player_1, VecDeque::new());
        }
        mem.insert((player_1.clone(), player_2.clone()));
        let (card_1, card_2) = (player_1.pop_front().unwrap(), player_2.pop_front().unwrap());
        if player_1.len() >= card_1 && player_2.len() >= card_2 {
            if play_recursive(&player_1.range(..card_1).copied().collect(), &player_2.range(..card_2).copied().collect()).0.is_empty() {
                player_2.push_back(card_2);
                player_2.push_back(card_1);
            } else {
                player_1.push_back(card_1);
                player_1.push_back(card_2);
            }
        } else if card_1 > card_2 {
            player_1.push_back(card_1);
            player_1.push_back(card_2);
        } else {
            player_2.push_back(card_2);
            player_2.push_back(card_1);
        }
    }
    (player_1, player_2)
}

fn score((player_1, player_2): (VecDeque<usize>, VecDeque<usize>)) -> usize {
    player_1.iter().rev().enumerate().map(|(idx, card)| (idx+1) * card).sum::<usize>() + player_2.iter().rev().enumerate().map(|(idx, card)| (idx+1) * card).sum::<usize>()
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
        assert_eq!(run(&sample_input), Ok((306, 291)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((33421, 33651)));
    }
}
