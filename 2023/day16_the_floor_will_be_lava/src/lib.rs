use core::fmt::Display;
use std::collections::{HashMap, BTreeSet};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyInput,
    InvalidChar(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "The specified input was empty"),
            Self::InvalidChar(c) => write!(f, "Unable to parse {c} into a tile"),
        }
    }
}

#[repr(u8)]
#[derive(PartialEq)]
enum Direction {
    North   = 0b0001,
    West    = 0b0010,
    East    = 0b0100,
    South   = 0b1000,
}

impl Direction {
    fn as_u8(&self) -> u8 {
        match self {
            Self::North =>  0b0001,
            Self::West =>   0b0010,
            Self::East =>   0b0100,
            Self::South =>  0b1000,
        }
    }

    fn from_u8(n: u8) -> Vec<Self> {
        let mut res = Vec::new();
        if n & 0b0001 > 0 {
            res.push(Self::North);
        }
        if n & 0b0010 > 0 {
            res.push(Self::West);
        }
        if n & 0b0100 > 0 {
            res.push(Self::East);
        }
        if n & 0b1000 > 0 {
            res.push(Self::South);
        }
        res
    }

    fn next_from(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Direction::North if y > 0 => Some((x, y-1)),
            Direction::North => None,
            Direction::West if x > 0 => Some((x-1, y)),
            Direction::West => None,
            Direction::East => Some((x+1, y)),
            Direction::South => Some((x, y+1)),
        }
    }
}

#[derive(PartialEq)]
enum Tile {
    Empty,      // .
    SplitterNS, // |
    SplitterWE, // -
    MirrorNWSE, // \
    MirrorNESW, // /
}

impl TryFrom<char> for Tile {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '|' => Ok(Self::SplitterNS),
            '-' => Ok(Self::SplitterWE),
            '\\' => Ok(Self::MirrorNWSE),
            '/' => Ok(Self::MirrorNESW),
            e => Err(Self::Error::InvalidChar(e)),
        }
    }
}

struct Grid {
    devices: HashMap<(usize, usize), Tile>,
    max: (usize, usize),
}

impl TryFrom<&str> for Grid {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::EmptyInput);
        }
        let mut max = (0, value.lines().count()-1);
        let mut devices = HashMap::new();

        for (y, line) in value.lines().enumerate() {
            max.0 = max.0.max(line.len()-1);
            for (x, c) in line.chars().enumerate() {
                let tile = Tile::try_from(c)?;
                if tile != Tile::Empty {
                    devices.insert((x, y), tile);
                }
            }
        }

        Ok(Self { devices, max, })
    }
}

impl Grid {
    fn get_energized(&self, starting_tile: (usize, usize), starting_direction: Direction) -> usize {
        let mut energized: HashMap<(usize, usize), u8> = HashMap::from([(starting_tile, starting_direction.as_u8())]);
        let mut open_set = BTreeSet::from([starting_tile]);
        while let Some((x, y)) = open_set.pop_last() {
            let incoming_directions = Direction::from_u8(*energized.get(&(x, y)).unwrap());
            let mut directions = Vec::new();
            for prev in incoming_directions {
                match self.devices.get(&(x, y)) {
                    None => directions.push(prev),
                    Some(Tile::SplitterNS) if [Direction::North, Direction::South].contains(&prev) => directions.push(prev),
                    Some(Tile::SplitterNS) => {
                        directions.push(Direction::North);
                        directions.push(Direction::South);
                    },
                    Some(Tile::SplitterWE) if [Direction::West, Direction::East].contains(&prev) => directions.push(prev),
                    Some(Tile::SplitterWE) => {
                        directions.push(Direction::West);
                        directions.push(Direction::East);
                    },
                    Some(Tile::MirrorNWSE) if prev == Direction::North => directions.push(Direction::West),
                    Some(Tile::MirrorNWSE) if prev == Direction::West => directions.push(Direction::North),
                    Some(Tile::MirrorNWSE) if prev == Direction::East => directions.push(Direction::South),
                    Some(Tile::MirrorNWSE) if prev == Direction::South => directions.push(Direction::East),
                    Some(Tile::MirrorNESW) if prev == Direction::North => directions.push(Direction::East),
                    Some(Tile::MirrorNESW) if prev == Direction::West => directions.push(Direction::South),
                    Some(Tile::MirrorNESW) if prev == Direction::East => directions.push(Direction::North),
                    Some(Tile::MirrorNESW) if prev == Direction::South => directions.push(Direction::West),
                    _ => unreachable!(),
                };
            }
            for direction in directions {
                if let Some(neighbour) = direction.next_from(x, y) {
                    if neighbour.0 <= self.max.0 && neighbour.1 <= self.max.1 {
                        let prev = *energized.get(&neighbour).unwrap_or(&0);
                        let new_direction = direction.as_u8();
                        if  prev & new_direction < new_direction {
                            open_set.insert(neighbour);
                            energized.insert(neighbour, new_direction | prev);
                        }
                    }
                }
            }
        }

        energized.len()
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let grid = Grid::try_from(input)?;
    let first = grid.get_energized((0,0), Direction::East);
    let second = (0..grid.max.0).map(|x| grid.get_energized((x, 0), Direction::South)).max().unwrap_or(0).max(
                    (0..grid.max.0).map(|x| grid.get_energized((x, grid.max.1), Direction::North)).max().unwrap_or(0).max(
                    (0..grid.max.1).map(|y| grid.get_energized((0, y), Direction::East)).max().unwrap_or(0).max(
                    (0..grid.max.1).map(|y| grid.get_energized((grid.max.0, y), Direction::West)).max().unwrap_or(0)
        )));
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
        assert_eq!(run(&sample_input), Ok((46, 51)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((8098, 8335)));
    }
}
