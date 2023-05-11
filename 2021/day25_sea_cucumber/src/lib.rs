use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidChar(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChar(c) => write!(f, "Tried to parse invalid character {c} into a space"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    East,
    South,
}

impl Direction {
    fn next(&self, y: usize, x: usize, height: usize, width: usize) -> (usize, usize) {
        match self {
            Self::East => (y, (x+1)%width),
            Self::South => ((y+1)%height, x),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Space {
    Free,
    Occupied(Direction),
}

impl TryFrom<char> for Space {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Free),
            '>' => Ok(Self::Occupied(Direction::East)),
            'v' => Ok(Self::Occupied(Direction::South)),
            o => Err(Self::Error::InvalidChar(o)),
        }
    }
}

pub fn run(input: &str) -> Result<usize, ParseError> {
    let mut map = input.lines().map(|line| line.chars().map(Space::try_from).collect::<Result<Vec<_>, _>>()).collect::<Result<Vec<_>, _>>()?;
    for step in 1.. {
        let to_move_east = consider_movements(&mut map, Direction::East);
        perform_movements(&mut map, &to_move_east);
        let to_move_south = consider_movements(&mut map, Direction::South);
        if to_move_east.is_empty() && to_move_south.is_empty() {
            return Ok(step);
        }
        perform_movements(&mut map, &to_move_south);
    }
    unreachable!("The loop always runs and only breaks by returning")
}

fn consider_movements(map: &mut [Vec<Space>], direction: Direction) -> Vec<(usize, usize)> {
    let height = map.len();
    let width = map[0].len();
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter()
             .enumerate()
             .filter(|(x, space)| {
                 let (next_y, next_x) = direction.next(y, *x, height, width);
                 **space == Space::Occupied(direction) && map[next_y][next_x] == Space::Free
                })
             .map(|(x, _)| (y, x))
             .collect::<Vec<_>>())
        .collect()
}

fn perform_movements(map: &mut [Vec<Space>], coords: &[(usize, usize)]) {
    coords.iter().for_each(|&(y, x)| {
        let height = map.len();
        let width = map[y].len();
        if let Space::Occupied(direction) = &map[y][x] {
            let (next_y, next_x) = direction.next(y, x, height, width);
            map[next_y][next_x] = Space::Occupied(*direction);
            map[y][x] = Space::Free;
        }
        
    });
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
        assert_eq!(run(&sample_input), Ok(58));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok(598));
    }
}
