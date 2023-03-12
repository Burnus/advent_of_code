use std::fs;

#[derive(Copy, Clone)]
enum Hand {
    Rock = 1, 
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn that_beats(other: Self) -> Self {
        match other {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
    fn that_is_beaten_by(other: Self) -> Self {
        Self::that_beats(Self::that_beats(other))
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

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn parse_round(line: &str, strat: u8) -> Round {
    let line = line.as_bytes();
    let opponent_hand = match line[0] {
        b'A' => Hand::Rock,
        b'B' => Hand::Paper,
        b'C' => Hand::Scissors,
        _ => panic!("Unexpected Token"),
    };
    let player_hand = match strat {
        1 => {
            match line[2] {
                b'X' => Hand::Rock,
                b'Y' => Hand::Paper,
                b'Z' => Hand::Scissors,
                _ => panic!("Unexpected Token"),
            }
            },
        2 => {
            match line[2] {
                b'X' => Hand::that_is_beaten_by(opponent_hand),
                b'Y' => opponent_hand,
                b'Z' => Hand::that_beats(opponent_hand),
                _ => panic!("Unexpected Token"),
            }
        },
        _ => panic!("Unexpected Strat"),
    };
    Round { opponent_hand, player_hand }
}

fn get_tally(moves: &str, strat: u8) -> u32 {
    moves.lines()
        .filter(|l| l.len() == 3)
        .map(|l| parse_round(l, strat).points())
        .sum()
}

fn main() {
    let contents = read_file("input");
    println!("Total Points (Strat 1): {}", get_tally(&contents, 1));
    println!("Total Points (Strat 2): {}", get_tally(&contents, 2));
}

#[test]
fn sample_input() {
    let contents = read_file("tests/sample_input");
    assert_eq!(get_tally(&contents, 1), 15);
    assert_eq!(get_tally(&contents, 2), 12);
}

#[test]
fn challenge_input() {
    let contents = read_file("tests/input");
    assert_eq!(get_tally(&contents, 1), 12458);
    assert_eq!(get_tally(&contents, 2), 12683);
}
