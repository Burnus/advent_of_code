use core::fmt::Display;
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    LineMalformed(String),
    ParseIntError(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseIntError(c) => write!(f, "Unable to parse {c} into integer"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    estimated_costs: usize,
    costs_so_far: usize,
    coordinates: (usize, usize),
}

impl Position {
    fn from(coordinates: (usize, usize), costs_so_far: usize, goal: (usize, usize)) -> Self {
        Self { 
            estimated_costs: costs_so_far + goal.0.abs_diff(coordinates.0) + goal.1.abs_diff(coordinates.1),
            costs_so_far, 
            coordinates,
        }
    }
}

struct Cavern {
    chitons: Vec<Vec<usize>>,
    max_x: usize,
    max_y: usize,
}

impl TryFrom<&str> for Cavern {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let chitons = value.lines().map(|line| line.chars().map(|c| c.to_digit(10).map(|i|i as usize).ok_or(Self::Error::ParseIntError(c))).collect::<Result<Vec<_>,_>>()).collect::<Result<Vec<_>, _>>()?;
        let max_y = chitons.len() - 1;
        let max_x = chitons.iter().map(|row| row.len()).min().unwrap() - 1;

        Ok(Self { chitons, max_x, max_y })
    }
}

impl Cavern {
    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        [(0, 1), (1, 0), (1, 2), (2, 1)].iter()
            .filter(|(dx, dy)| (1..=self.max_x+1).contains(&(x+dx)) && (1..=self.max_y+1).contains(&(y+dy)))
            .map(|(dx, dy)| (x+dx-1, y+dy-1))
            .collect()
    }

    fn cheapest_path(&self, start: (usize, usize), goal: (usize, usize)) -> usize {
        let mut open_set = BTreeSet::from([Position::from(start, 0, goal)]);
        let mut visited = HashSet::new();
        while let Some(current) = open_set.pop_first() {
            if visited.contains(&current.coordinates) {
                continue;
            }
            visited.insert(current.coordinates);
            if current.coordinates == goal {
                return current.costs_so_far;
            } else {
                for neighbour in self.neighbours(current.coordinates) {
                    let next = Position::from(neighbour, current.costs_so_far + self.chitons[neighbour.1][neighbour.0], goal);
                    open_set.insert(next);
                }
            }
        }
        panic!("Unable to find a way from ({}, {}) to ({}, {})", start.0, start.1, goal.0, goal.1);
    }

    fn expand(&mut self, factor: usize) {
        let mut chitons = (0..factor).flat_map(|y_factor| self.chitons.iter().map(|row| (0..factor).flat_map(|x_factor| row.iter().map(|orig| (orig + x_factor + y_factor - 1) % 9 + 1).collect::<Vec<_>>()).collect::<Vec<usize>>()).collect::<Vec<_>>()).collect();
        self.max_x = self.max_x * factor + factor - 1;
        self.max_y = self.max_y * factor + factor - 1;
        std::mem::swap(&mut chitons, &mut self.chitons);
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut cavern = Cavern::try_from(input)?;
    let first = cavern.cheapest_path((0, 0), (cavern.max_x, cavern.max_y));
    cavern.expand(5);
    let second = cavern.cheapest_path((0, 0), (cavern.max_x, cavern.max_y));
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
        assert_eq!(run(&sample_input), Ok((40, 315)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((410, 2809)));
    }
}
