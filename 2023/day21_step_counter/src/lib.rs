use core::fmt::Display;
use std::collections::{HashSet, BinaryHeap};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidTile(u8),
    NoStartingPosition,
    TooManyStartingPositions
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTile(b) => write!(f, "\"{}\" is not a valid map tile", *b as char), 
            Self::NoStartingPosition => write!(f, "No starting position (marked \"S\") found"),
            Self::TooManyStartingPositions => write!(f, "More than one starting position (marked \"S\") found"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Position(isize, isize);

impl Position {
    fn get_neighbours(&self) -> [Self; 4] {
        [
            Self(self.0-1, self.1),
            Self(self.0, self.1-1),
            Self(self.0+1, self.1),
            Self(self.0, self.1+1),
        ]
    }
}

struct Map {
    rocks: HashSet<Position>,
    starting: Position,
    max: Position,
}

impl TryFrom<&str> for Map {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut rocks = HashSet::new();
        let mut starting = None;
        let mut max = Position(0, value.lines().count() as isize);
        for (y, line) in value.lines().enumerate() {
            max.0 = max.0.max(line.len() as isize);
            for (x, b) in line.bytes().enumerate() {
                match b {
                    b'.' => (),
                    b'#' => _ = rocks.insert(Position(x as isize, y as isize)),
                    b'S' => if starting.is_none() {
                        starting = Some(Position(x as isize, y as isize));
                    } else {
                        return Err(Self::Error::TooManyStartingPositions);
                    },
                        e => return Err(Self::Error::InvalidTile(e)),
                }
            }
        }
        match starting {
            None => Err(Self::Error::NoStartingPosition),
            Some(s) => Ok(Self{ rocks, starting: s, max })
        }
    }
}

#[derive(PartialEq, Eq)]
struct SearchState {
    steps: usize,
    position: Position
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.steps.cmp(&self.steps)
    }
}

pub fn run(input: &str, steps: usize) -> Result<usize, ParseError> {
    let map = Map::try_from(input)?;
    Ok(count_positions_after(steps, &map))
}

fn count_positions_after(steps: usize, map: &Map) -> usize {
    let size = map.max.0.max(map.max.1) as usize;
    let mut visited_even = HashSet::from([map.starting]);
    let mut visited_odd = HashSet::new();
    let mut open_set = BinaryHeap::from([SearchState{ steps: 0, position: map.starting }]);
    let mut last_step = 0;
    let mut results = Vec::new();
    while let Some(state) = open_set.pop() {
        let (step, position) = (state.steps, state.position);
        if step == steps {
            continue;
        }
        if step > last_step && (step-1) % size == steps % size { 
            let curr = if step % 2 == 1 {
                visited_even.len()
            } else {
                visited_odd.len()
            };
            if results.len() > 2 {
                let prev_1 = results[results.len()-1];
                let prev_2 = results[results.len()-2];
                let prev_3 = results[results.len()-3];
                if curr+3*prev_2 == 3*prev_1+prev_3 {
                    let remaining_cycles = steps/size - results.len();
                    return curr + remaining_cycles*(curr-prev_1) + triangular(remaining_cycles)*(curr+prev_2-2*prev_1);
                }
            }
            results.push(curr);
        }
        last_step = step;
        for neighbour in position.get_neighbours() {
            if step % 2 == 0 {
                if !visited_odd.contains(&neighbour) && !map.rocks.contains(&Position(neighbour.0.rem_euclid(map.max.0), neighbour.1.rem_euclid(map.max.1))) {
                    visited_odd.insert(neighbour);
                    open_set.push(SearchState{ steps: step+1, position: neighbour });
                }
            } else if !visited_even.contains(&neighbour) && !map.rocks.contains(&Position(neighbour.0.rem_euclid(map.max.0), neighbour.1.rem_euclid(map.max.1))) {
                visited_even.insert(neighbour);
                open_set.push(SearchState{ steps: step+1, position: neighbour });
            }
        }
    }
    if steps % 2 == 0 {
        visited_even.len()
    } else {
        visited_odd.len()
    }
}

fn triangular(n: usize) -> usize {
    (n*n+n)/2
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
        let samples = [(6, 16), (10, 50), (50, 1594), (100, 6536), (500, 167004), (1000, 668697), (5000, 16733044)];
        for (steps, expected) in samples {
            assert_eq!(run(&sample_input, steps), Ok(expected));
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        let samples = [(64, 3617), (26501365, 596857397104703)];
        for (steps, expected) in samples {
            assert_eq!(run(&challenge_input, steps), Ok(expected));
        }
    }
}
