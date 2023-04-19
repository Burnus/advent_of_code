use core::fmt::Display;
use std::{num::ParseIntError, collections::{HashMap, HashSet}};

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

struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

impl TryFrom<&str> for Line {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let ends: Vec<_> = value.split_whitespace().collect();
        if ends.len() != 3 {
            return Err(Self::Error::LineMalformed(value.to_string()));
        }

        let start: Vec<_> = ends[0].split(',').collect();
        let end: Vec<_> = ends[2].split(',').collect();

        if start.len() != 2 || end.len() != 2 {
            return Err(Self::Error::LineMalformed(value.to_string()));
        }

        Ok(Self { 
            start: (start[0].parse()?, start[1].parse()?), 
            end: (end[0].parse()?, end[1].parse()?), 
        })
    }
}

impl Line {
    fn get_horizontal_vertical_coordinates(&self) -> Vec<(usize, usize)> {
        if self.start.0 == self.end.0 {
            let x = self.start.0;
            let min = self.start.1.min(self.end.1);
            let max = self.start.1.max(self.end.1);

            (min..=max).map(|y| (x, y)).collect()
        } else if self.start.1 == self.end.1 {
            let y = self.start.1;
            let min = self.start.0.min(self.end.0);
            let max = self.start.0.max(self.end.0);

            (min..=max).map(|x| (x, y)).collect()
        } else {
            Vec::new()
        }
    }

    fn get_diagonal_coordingates(&self) -> Vec<(usize, usize)> {
        if self.start.0 < self.end.0 && self.start.1 < self.end.1 {
            (0..=self.end.0-self.start.0).map(|f| (self.start.0+f, self.start.1+f)).collect()
        } else if self.start.0 < self.end.0 && self.start.1 > self.end.1 {
            (0..=self.end.0-self.start.0).map(|f| (self.start.0+f, self.start.1-f)).collect()
        } else if self.start.0 > self.end.0 && self.start.1 > self.end.1 {
            (0..=self.start.0-self.end.0).map(|f| (self.start.0-f, self.start.1-f)).collect()
        } else if self.start.0 > self.end.0 && self.start.1 < self.end.1 {
            (0..=self.start.0-self.end.0).map(|f| (self.start.0-f, self.start.1+f)).collect()
        } else {
            Vec::new()
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let lines: Vec<_> = input.lines().map(Line::try_from).collect::<Result<Vec<_>, _>>()?;
    let mut all_points = HashSet::new();
    let mut duplicates = HashSet::new();
    lines.iter().for_each(|line| {
        line.get_horizontal_vertical_coordinates().iter().for_each(|coord| {
            if all_points.contains(coord) {
                duplicates.insert(*coord);
            }
            all_points.insert(*coord);
        });
    });
    let first = duplicates.len();
    lines.iter().for_each(|line| {
        line.get_diagonal_coordingates().iter().for_each(|coord| {
            if all_points.contains(coord) {
                duplicates.insert(*coord);
            }
            all_points.insert(*coord);
        });
    });
    let second = duplicates.len();
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
        assert_eq!(run(&sample_input), Ok((5, 12)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((6007, 19349)));
    }
}
