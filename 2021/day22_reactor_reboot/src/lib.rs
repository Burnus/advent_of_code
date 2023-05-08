use core::fmt::Display;
use std::{num::ParseIntError, collections::HashSet};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidToggle(String),
    LineMalformed(String),
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
            Self::InvalidToggle(v) => write!(f, "Invalid Toggle Instruction: {v}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Toggle {
    On,
    Off,
}

impl TryFrom<&str> for Toggle {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            o => Err(Self::Error::InvalidToggle(o.to_string())),
        }
    }
}

impl Toggle {
    fn target_state(&self) -> bool {
        match self {
            Self::On => true,
            Self::Off => false,
        }
    }
}

struct Instruction {
    toggle: Toggle,
    x_range: (isize, isize),
    y_range: (isize, isize),
    z_range: (isize, isize),
}

impl TryFrom<&str> for Instruction {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((toggle, rest)) = value.split_once(' ') {
            let toggle = Toggle::try_from(toggle)?;

            let parameters: Vec<_> = rest.split(&['=', ',', '.']).collect();
            if parameters.len() != 12 {
                return Err(Self::Error::LineMalformed(value.to_string()));
            }
            Ok(Self { 
                toggle,
                x_range: (parameters[1].parse()?, parameters[3].parse()?), 
                y_range: (parameters[5].parse()?, parameters[7].parse()?), 
                z_range: (parameters[9].parse()?, parameters[11].parse()?) 
            })
        } else {
            Err(Self::Error::LineMalformed(value.to_string()))
        }
    }
}

impl Instruction {
    fn perform_bounded(&self, target: &mut HashSet<(isize, isize, isize)>, lower_bound: isize, upper_bound: isize) {
        if self.x_range.0 < upper_bound && self.x_range.1 > lower_bound &&
            self.y_range.0 < upper_bound && self.y_range.1 > lower_bound &&
            self.z_range.0 < upper_bound && self.z_range.1 > lower_bound {
                if self.toggle.target_state() {
                    (self.x_range.0.max(lower_bound)..=self.x_range.1.min(upper_bound)).for_each(|x| 
                        (self.y_range.0.max(lower_bound)..=self.y_range.1.min(upper_bound)).for_each(|y| 
                            (self.z_range.0.max(lower_bound)..=self.z_range.1.min(upper_bound)).for_each(|z| {
                                target.insert((x, y, z));
                            })));
                } else {
                    (self.x_range.0.max(lower_bound)..=self.x_range.1.min(upper_bound)).for_each(|x| 
                        (self.y_range.0.max(lower_bound)..=self.y_range.1.min(upper_bound)).for_each(|y| 
                            (self.z_range.0.max(lower_bound)..=self.z_range.1.min(upper_bound)).for_each(|z| {
                                target.remove(&(x, y, z));
                            })));
                }
            }
    }

    fn volume(&self) -> isize {
        (self.x_range.1 - self.x_range.0 + 1) * (self.y_range.1 - self.y_range.0 + 1) * (self.z_range.1 - self.z_range.0 + 1)
    }

    fn intersection(&self, rhs: &Self) -> Self {
        let x_min = self.x_range.0.max(rhs.x_range.0);
        let y_min = self.y_range.0.max(rhs.y_range.0);
        let z_min = self.z_range.0.max(rhs.z_range.0);
        let x_max = self.x_range.1.min(rhs.x_range.1);
        let y_max = self.y_range.1.min(rhs.y_range.1);
        let z_max = self.z_range.1.min(rhs.z_range.1);
        if x_min <= x_max && y_min <= y_max && z_min <= z_max {
            Self {
                toggle: Toggle::On,
                x_range: (x_min, x_max),
                y_range: (y_min, y_max),
                z_range: (z_min, z_max),
            }
        } else {
            Self {
                toggle: Toggle::Off,
                x_range: (0, -1),
                y_range: (0, -1),
                z_range: (0, -1),
            }
        }
    }
}

pub fn run(input: &str) -> Result<(usize, isize), ParseError> {
    let instructions: Vec<_> = input.lines().map(Instruction::try_from).collect::<Result<Vec<_>, _>>()?;
    let mut reactor = HashSet::new();
    instructions.iter().for_each(|i| i.perform_bounded(&mut reactor, -50, 50));
    let first = reactor.len();
    let second = count_cubes(&instructions);
    Ok((first, second))
}

fn count_cubes(instructions: &[Instruction]) -> isize {
    if instructions.is_empty() {
        0
    } else if instructions[0].toggle == Toggle::Off {
        count_cubes(&instructions[1..])
    } else {
        let current = &instructions[0];
        current.volume() + count_cubes(&instructions[1..]) - count_cubes(&instructions.iter().skip(1).map(|other| current.intersection(other)).collect::<Vec<_>>())
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
        assert_eq!(run(&sample_input), Ok((474140, 2758514936282235)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((588200, 1207167990362099)));
    }
}
