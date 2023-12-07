use core::fmt::Display;
use std::{num::ParseIntError, char::from_digit};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    ParseIntError(std::num::ParseIntError),
    ParseCardError(char),
    LineMalformed(&'a str),
    InvalidCardCount(usize, &'a str),
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCardCount(c, h) => write!(f, "The Hand \"{h}\" has {c} cards instead of 5."),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseCardError(c) => write!(f, "Unable to parse into Card: {c}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq)]
pub struct Hand {
    cards: [u8; 5],
    bid: usize,
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        self.cards.iter().for_each(|c| out.push(match c {
            1 | 11 => 'J',
            n if (2..=9).contains(n) => from_digit(*n as u32, 10).unwrap(),
            10 => 'T',
            12 => 'Q',
            13 => 'K',
            14 => 'A',
            _ => '?',
        }));
        write!(f, "{out} {}", self.bid)
    }
}

impl<'a> TryFrom<&'a str> for Hand {
    type Error = ParseError<'a>;

    /// Construct a hand from a str like `T243A 42`, listing the cards in order first, and then the
    /// bid, separated by whitespace. The cards part needs to be 5 characters exactly and consist
    /// of numerals 2-9 or letters 'T', 'J', 'Q', 'K', or 'A' only. The bid needs to be a positive
    /// integer up to `usize::MAX`. 
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let parts: Vec<_> = value.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(Self::Error::LineMalformed(value));
        }
        if parts[0].chars().count() != 5 {
            return Err(Self::Error::InvalidCardCount(parts[0].chars().count(), parts[0]));
        }
        let mut cards = [0; 5];
        for (idx, c) in parts[0].chars().enumerate() {
            match c {
                n if n.is_ascii_digit() && !['0', '1'].contains(&n) => cards[idx] = n.to_digit(10).unwrap() as u8,
                'T' => cards[idx] = 10,
                'J' => cards[idx] = 11,
                'Q' => cards[idx] = 12,
                'K' => cards[idx] = 13,
                'A' => cards[idx] = 14,
                c => return Err(Self::Error::ParseCardError(c)),
            };
        }
        let bid = parts[1].parse()?;
        Ok(Hand{ cards, bid, })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_type().cmp(&other.get_type()).then_with(|| self.cards.cmp(&other.cards))
    }
}

impl Hand {
    /// Return the Type of this hand (e. g. Full House or Four of a Kind) as a `HandType` variant.
    pub fn get_type(&self) -> HandType {
        let mut card_count = [0_u8; 13];
        let mut jokers = 0_u8;
        self.cards.iter().for_each(|c| {
            match c {
                1 => jokers += 1,
                n => card_count[*n as usize - 2] += 1,
            }
        });

        card_count.sort_by(|a, b| b.cmp(a));
        match (card_count[0] + jokers, card_count[1]) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, _) =>HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    /// Convert the hand into version 1 of the game (`J`s are Jacks). This function does nothing if
    /// the hand includes no `J`s, or is already in version 1 format.
    pub fn set_v1(&mut self) {
        self.cards.iter_mut().for_each(|card| if *card == 1 { *card = 11 });
    }

    /// Convert the hand into version 2 of the game (`J`s are Jokers). This function does nothing if
    /// the hand includes no `J`s, or is already in version 2 format.
    pub fn set_v2(&mut self) {
        self.cards.iter_mut().for_each(|card| if *card == 11 { *card = 1 });
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut hands: Vec<_> = input.lines().map(Hand::try_from).collect::<Result<Vec<_>, _>>()?;
    hands.sort();
    let first = hands.iter().enumerate().map(|(rank, hand)| (rank+1)*hand.bid).sum();
    hands.iter_mut().for_each(|hand| hand.set_v2());
    hands.sort();
    let second = hands.iter().enumerate().map(|(rank, hand)| (rank+1)*hand.bid).sum();
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
        assert_eq!(run(&sample_input), Ok((6440, 5905)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((248179786, 247885995)));
    }
}
