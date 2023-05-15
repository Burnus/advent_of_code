use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    LineMalformed(&'a str),
    ParseIntError(std::num::ParseIntError),
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

use std::collections::HashSet;

#[derive(PartialEq)]
enum Status { Resting, Falling, Blocked }

#[derive(PartialEq)]
enum Mode { EndlessVoid, WithFloor }

#[derive(PartialEq, Eq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl <'a> TryFrom<&'a str> for Position {
    type Error = ParseError<'a>; 

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let components = value.split(',').collect::<Vec<_>>().iter().map(|i| i.parse()).collect::<Result<Vec<usize>, _>>()?;
        if !components.len() == 2 { 
            return Err(Self::Error::LineMalformed(value));
        }

        Ok(Self {
            x: components[0],
            y: components[1],
        })
    }
}

struct Sand {
    position: Position,
    ymax: usize,
}

const ORIGIN: Position = Position {
    x: 500,
    y: 0,
};

impl Sand {
    fn fall(&mut self, cave: &HashSet<Position>, other_sand: &mut HashSet<Position>, mode: &Mode) -> Status {
        // return if we fall below all structures
        if *mode == Mode::EndlessVoid && self.position.y >= self.ymax {
            return Status::Falling;
        }
        // or we reached the floor. 
        if *mode == Mode::WithFloor && self.position.y > self.ymax {
            other_sand.insert(self.position.clone());
            return Status::Resting;
        }
        // Fall down if possible
        if !cave.contains(&Position{ x: self.position.x, y: self.position.y+1 }) && !other_sand.contains(&Position { x: self.position.x, y: self.position.y+1 }) {
            self.position.y += 1;
            return self.fall(cave, other_sand, mode);
        }
        // Next try falling left
        if !cave.contains(&Position{ x: self.position.x-1, y: self.position.y+1 }) && !other_sand.contains(&Position { x: self.position.x-1, y: self.position.y+1 }) {
            self.position.x -= 1;
            self.position.y += 1;
            return self.fall(cave, other_sand, mode);
        }
        // Next try falling right
        if !cave.contains(&Position{ x: self.position.x+1, y: self.position.y+1 }) && !other_sand.contains(&Position { x: self.position.x+1, y: self.position.y+1 }) {
            self.position.x += 1;
            self.position.y += 1;
            return self.fall(cave, other_sand, mode);
        }
        // Else we can't fall any more.
        other_sand.insert(self.position.clone());
        if self.position == ORIGIN {
            Status::Blocked
        } else {
            Status::Resting
        }
    }

    fn spawn(cave: &HashSet<Position>, ymax: usize, mode: &Mode) -> HashSet<Position> {
        let mut other_sand = HashSet::new();
        loop {
            let mut new_unit = Sand {
                position: ORIGIN,
                ymax,
            };
            let new_status = new_unit.fall(cave, &mut other_sand, mode);
            if new_status != Status::Resting {
                break;
            }
        }
        other_sand
    }
}

fn positions_of_formation(formation: &str) -> Result<Vec<Position>, ParseError> {
    let mut blocked = Vec::new();
    let corners = formation.split(" -> ")
                    .map(Position::try_from)
                    .collect::<Result<Vec<Position>, _>>()?;
    if corners.len() == 1 {
        return Ok(corners);
    }
    for pair in corners.windows(2).collect::<Vec<&[Position]>>() {
        let minx = pair[0].x.min(pair[1].x);
        let maxx = pair[0].x.max(pair[1].x);
        let miny = pair[0].y.min(pair[1].y);
        let maxy = pair[0].y.max(pair[1].y);

        for x in minx..=maxx {
            for y in miny..=maxy {
                blocked.push(Position{ x, y });
            }
        }
    }
    Ok(blocked)
}

fn get_cave(scan: &str) -> Result<(HashSet<Position>, usize), ParseError> {
    let mut cave = HashSet::new();
    for line in scan.lines() {
        cave.extend(positions_of_formation(line)?.into_iter());
    }
    let ymax = cave.iter()
                    .map(|pos| pos.y)
                    .max()
                    .unwrap_or_default();
    Ok((cave, ymax))
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    // let items: Vec<_> = input.lines().map(::try_from).collect::<Result<Vec<_>, _>>()?;
    let (cave, ymax) = get_cave(input)?;
    let first = Sand::spawn(&cave, ymax, &Mode::EndlessVoid).len();
    let second = Sand::spawn(&cave, ymax, &Mode::WithFloor).len();
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
        assert_eq!(run(&sample_input), Ok((24, 93)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((979, 29044)));
    }
}
// fn main() {
//     let scan = read_file("input");
//
//     let (cave, ymax) = get_cave(&scan);
//
//     let endless_sand = Sand::spawn(&cave, ymax, &Mode::EndlessVoid);
//     println!("In Case of an endless void, {} units of sand will come to a rest", endless_sand.len());
//
//     let sand_with_floor = Sand::spawn(&cave, ymax, &Mode::WithFloor);
//     println!("In Case of a floor, {} units of sand will be spawned", sand_with_floor.len());
// }
//
// #[test]
// fn sample_input() {
//     let scan = read_file("tests/sample_input");
//     let (cave, ymax) = get_cave(&scan);
//     assert_eq!(Sand::spawn(&cave, ymax, &Mode::EndlessVoid).len(), 24);
//     assert_eq!(Sand::spawn(&cave, ymax, &Mode::WithFloor).len(), 93);
// }
//
// #[test]
// fn challenge_input() {
//     let scan = read_file("tests/input");
//     let (cave, ymax) = get_cave(&scan);
//     assert_eq!(Sand::spawn(&cave, ymax, &Mode::EndlessVoid).len(), 979);
//     assert_eq!(Sand::spawn(&cave, ymax, &Mode::WithFloor).len(), 29044);
// }
