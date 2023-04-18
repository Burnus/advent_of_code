use core::fmt::Display;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    LineMalformed(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    north: isize,
    east: isize,
}

impl TryFrom<&str> for Position {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut north = 0;
        let mut east = 0;

        let line = value.as_bytes();
        let mut idx = 0;
        while idx < line.len() {
            match line[idx] {
                b'e' => east += 2,
                b'w' => east -= 2,
                b'n' => {
                    north += 1;
                    idx += 1;
                    match line[idx] {
                        b'e' => east += 1,
                        b'w' => east -= 1,
                        _ => return Err(Self::Error::LineMalformed(value.to_string())),
                    }
                },
                b's' => {
                    north -= 1;
                    idx += 1;
                    match line[idx] {
                        b'e' => east += 1,
                        b'w' => east -= 1,
                        _ => return Err(Self::Error::LineMalformed(value.to_string())),
                    }
                },
                _ => return Err(Self::Error::LineMalformed(value.to_string())),
            }
            idx += 1;
        }

        Ok(Self { north, east, })
    }
}

impl Position {
    fn get_neighbours(&self) -> [Self; 6] {
        [
            Self {north: self.north, east: self.east+2},        // E
            Self {north: self.north, east: self.east-2},        // W
            Self {north: self.north+1, east: self.east+1},      // NE
            Self {north: self.north+1, east: self.east-1},      // NW
            Self {north: self.north-1, east: self.east+1},      // SE
            Self {north: self.north-1, east: self.east-1},      // SW
        ]
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut flipped = set_pattern(input)?;
    let first = flipped.len();

    for _ in 0..100 {
        apply_rules(&mut flipped);
    }

    let second = flipped.len();
    Ok((first, second))
}

fn set_pattern(input: &str) -> Result<HashSet<Position>, ParseError> {
    let mut flipped = HashSet::new();
    for line in input.lines() {
        let pos = Position::try_from(line)?;
        if flipped.contains(&pos) {
            flipped.remove(&pos);
        } else {
            flipped.insert(pos);
        }
    }
    Ok(flipped)
}

fn apply_rules(flipped: &mut HashSet<Position>) {
    let mut new = HashSet::new();

    flipped.iter().for_each(|tile| {
        let neighbours = tile.get_neighbours();
        if (1..=2).contains(&neighbours.iter().filter(|n| flipped.contains(&n)).count()) {
            new.insert(*tile);
        }
        neighbours.iter().for_each(|neighbour| {
            if neighbour.get_neighbours().iter().filter(|nn| flipped.contains(&nn)).count() == 2 {
                new.insert(*neighbour);
            }
        });
    });

    std::mem::swap(&mut new, flipped);
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
        assert_eq!(run(&sample_input), Ok((10, 2208)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((427, 3837)));
    }
}
