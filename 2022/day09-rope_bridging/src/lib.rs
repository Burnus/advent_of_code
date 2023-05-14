use core::fmt::Display;
use std::num::ParseIntError;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    ParseIntError(std::num::ParseIntError),
    UnexpectedDirection(&'a str),
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
            Self::UnexpectedDirection(c) => write!(f, "Trying to parse unexpected character {c} into direction"),
        }
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn perform_motion(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.x-=1,
            Direction::Right => self.x+=1,
            Direction::Up => self.y+=1,
            Direction::Down => self.y-=1,
        }
    }

    fn follow(&self, head: &Position) -> Self {
        let (dx, dy) = (head.x-self.x, head.y-self.y);
        match (dx, dy) {
            (2,0) => Self { x: self.x+1, y: self.y },
            (0,2) => Self { x: self.x, y: self.y+1 },
            (-2,0) => Self { x: self.x-1, y: self.y },
            (0,-2) => Self { x: self.x, y: self.y-1},
            (2,2)| (2,1) | (1,2) => Self { x: self.x+1, y: self.y+1 },
            (2,-2) | (2,-1) | (1,-2) => Self { x: self.x+1, y: self.y-1 },
            (-2,-2) | (-2,-1) | (-1,-2) => Self { x: self.x-1, y: self.y-1 },
            (-2,2) | (-2,1) |(-1,2) => Self { x: self.x-1, y: self.y+1 },
            _ => *self,
        }
    }
}

fn parse_head_movement(instruction: &str) -> Result<(Direction, i32), ParseError> {
    let direction = match &instruction[0..=0] {
        "L" => Ok(Direction::Left),
        "R" => Ok(Direction::Right),
        "U" => Ok(Direction::Up),
        "D" => Ok(Direction::Down),
        e => Err(ParseError::UnexpectedDirection(e)),
    }?;

    let count = instruction[2..].parse()?;
    Ok((direction, count))
}

fn get_visited(head_movements: &[(Direction, i32)], rope_length: usize) -> HashSet<Position> {
    let mut positions = vec![Position { x:0, y: 0 }; rope_length];
    let mut visited = HashSet::new();
    visited.insert(positions[0]);
    for (head_direction, count) in head_movements {
        for _ in 0..*count {
            positions[0].perform_motion(head_direction);
            for i in 1..rope_length {
                positions[i] = positions[i].follow(&positions[i-1]);
            }
            visited.insert(positions[rope_length-1]);
        }
    }
    visited
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let movements: Vec<_> = input.lines().map(parse_head_movement).collect::<Result<Vec<_>, _>>()?;
    let first = get_visited(&movements, 2).len();
    let second = get_visited(&movements, 10).len();
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
        assert_eq!(run(&sample_input), Ok((13, 1)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((6376, 2607)));
    }
}
