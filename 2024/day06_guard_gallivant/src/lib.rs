use core::fmt::Display;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyMap,
    InvalidChar(char),
    NoGuard,
    NonRectangular,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyMap => write!(f, "Input can't be empty"),
            Self::InvalidChar(e) => write!(f, "Unable to parse {e} into a map item. Vaid items are '.', '#', and '^'."),
            Self::NoGuard => write!(f, "No guard found. Expected exactly one '^'."),
            Self::NonRectangular => write!(f, "All input lines must be of equal length"),
        }
    }
}

type Coordinates = (isize, isize);

#[derive(Clone)]
struct Map {
    obstacles: HashSet<Coordinates>,
    width: isize,
    height: isize,
    guard_position: Coordinates,
    guard_facing: Coordinates,
}

impl TryFrom<&str> for Map {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut obstacles = HashSet::new();
        let mut guard_position = None;
        let mut guard_facing = None;
        let height = value.lines().count();
        if height == 0 {
            return Err(Self::Error::EmptyMap);
        }
        let width = value.lines().next().unwrap().len();

        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if x > width {
                    return Err(Self::Error::NonRectangular);
                }
                match c {
                    '.' => (),
                    '#' => _ = obstacles.insert((x as isize, y as isize)),
                    '^' => {
                        guard_position = Some((x as isize, y as isize));
                        guard_facing = Some((0, -1));
                    },
                    e => return Err(Self::Error::InvalidChar(e)),
                }
            }
        }
        if guard_position.is_none() {
            return Err(Self::Error::NoGuard);
        }
        Ok(Self {
            obstacles,
            height: height as isize,
            width: width as isize,
            guard_position: guard_position.unwrap(),
            guard_facing: guard_facing.unwrap(),
        })
    }
}

impl Map {
    /// Return the next `facing` by turning right. Panics if called with an invalid facing.
    fn turn_right(facing: Coordinates) -> Coordinates {
        match facing {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => unreachable!(),
        }
    }

    fn guard_way(&self) ->Option<HashSet<Coordinates>> {
        let mut curr = self.guard_position;
        let mut facing = self.guard_facing;
        let mut route = HashSet::from([(curr, facing)]);
        loop {
            let next = (curr.0+facing.0, curr.1+facing.1);
            if next.0 < 0 || next.1 < 0 || next.0 == self.width || next.1 == self.height {
                break;
            }
            let right = Self::turn_right(facing);
            if self.obstacles.contains(&next) {
                facing = right;
            } else {
                curr = next;
            }
            if route.contains(&(curr, facing)) {
                return None;
            }
            route.insert((curr, facing));
        }
        let route_positions = route.iter().map(|(position, _facing)| *position).collect();
        Some(route_positions)
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let map = Map::try_from(input)?;
    let guard_way = map.guard_way().unwrap();
    let first = guard_way.len();
    let second = guard_way.iter().map(|pos| {
        let mut new_map = map.clone();
        new_map.obstacles.insert(*pos);
        new_map
    }).filter(|map| map.guard_way().is_none())
    .count();
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
        assert_eq!(run(&sample_input), Ok((41, 6)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((5177, 1686)));
    }
}
