use core::fmt::Display;
use std::num::ParseIntError;

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

enum Action {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forwd(isize),
}

impl TryFrom<&str> for Action {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.chars().next() {
            Some('N') => Ok(Self::North(value[1..].parse()?)),
            Some('S') => Ok(Self::South(value[1..].parse()?)),
            Some('E') => Ok(Self::East(value[1..].parse()?)),
            Some('W') => Ok(Self::West(value[1..].parse()?)),
            Some('L') => Ok(Self::Left(value[1..].parse()?)),
            Some('R') => Ok(Self::Right(value[1..].parse()?)),
            Some('F') => Ok(Self::Forwd(value[1..].parse()?)),
            _ => Err(Self::Error::LineMalformed(value.to_string())),
        }
    }
}

struct Ferry {
    pos: (isize, isize),
    dir: isize,
    waypoint: (isize, isize),
}

impl Default for Ferry {
    fn default() -> Self {
        Self {
            pos: (0, 0),
            dir: 0,
            waypoint: (10, -1),
        }
    }
}

impl Ferry {
    fn perform_action(&mut self, action: &Action) {
        match action {
            Action::North(i) => self.pos.1 -= i,
            Action::South(i) => self.pos.1 += i,
            Action::East(i) => self.pos.0 += i,
            Action::West(i) => self.pos.0 -= i,
            Action::Left(i) => self.dir = (self.dir-i) % 360,
            Action::Right(i) => self.dir = (self.dir+i) % 360,
            Action::Forwd(i) => match self.dir {
                -90 | 270 => self.pos.1 -= i,
                90 | -270 => self.pos.1 += i,
                0 => self.pos.0 += i,
                180 | -180 => self.pos.0 -= i,
                _ => panic!("Trying to move in unknown direction {}", self.dir),
            },
        }
    }

    fn perform_waypoint_action(&mut self, action: &Action) {
        match action {
            Action::North(i) => self.waypoint.1 -= i,
            Action::South(i) => self.waypoint.1 += i,
            Action::East(i) => self.waypoint.0 += i,
            Action::West(i) => self.waypoint.0 -= i,
            Action::Left(i) => match i {
                90 => {
                    let x = self.waypoint.0;
                    self.waypoint.0 = self.waypoint.1;
                    self.waypoint.1 = -x;
                },
                180 => {
                    self.waypoint.0 *= -1;
                    self.waypoint.1 *= -1;
                },
                270 => {
                    let x = self.waypoint.0;
                    self.waypoint.0 = -self.waypoint.1;
                    self.waypoint.1 = x;
                },
                _ => panic!("Trying to turn left by unknown amount {i}"),
            },
            Action::Right(i) => match i {
                90 => {
                    let x = self.waypoint.0;
                    self.waypoint.0 = -self.waypoint.1;
                    self.waypoint.1 = x;
                },
                180 => {
                    self.waypoint.0 *= -1;
                    self.waypoint.1 *= -1;
                },
                270 => {
                    let x = self.waypoint.0;
                    self.waypoint.0 = self.waypoint.1;
                    self.waypoint.1 = -x;
                },
                _ => panic!("Trying to turn right by unknown amount {i}"),
            },
            Action::Forwd(i) => {
                self.pos.0 += i*self.waypoint.0;
                self.pos.1 += i*self.waypoint.1;
            },
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut ferry = Ferry::default();
    let actions: Vec<_> = input.lines().map(Action::try_from).collect::<Result<Vec<_>, _>>()?;
    actions.iter().for_each(|a| ferry.perform_action(a));
    let first = ferry.pos.0.unsigned_abs() + ferry.pos.1.unsigned_abs();
    ferry = Ferry::default();
    actions.iter().for_each(|a| ferry.perform_waypoint_action(a));
    let second = ferry.pos.0.unsigned_abs() + ferry.pos.1.unsigned_abs();
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
        assert_eq!(run(&sample_input), Ok((25, 286)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((1032, 156735)));
    }
}
