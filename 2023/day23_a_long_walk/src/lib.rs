use core::fmt::Display;
use std::{num::ParseIntError, collections::HashSet};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    InvalidChar(char),
    LineMalformed(&'a str),
    NoDestError,
    NoStartError,
    NoUniqueDestError,
    NoUniqueStartError,
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
            Self::InvalidChar(c) => write!(f, "Invalid Character detected: \"{c}\" is not a valid map item."),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::NoDestError => write!(f, "Input does not contain a \".\" in its last line"),
            Self::NoStartError => write!(f, "Input does not contain a \".\" in its first line"),
            Self::NoUniqueDestError => write!(f, "Input contains more than one \".\" in its last line"),
            Self::NoUniqueStartError => write!(f, "Input contains more than one \".\" in its first line"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate(usize, usize);

#[derive(PartialEq, Eq, Hash)]
enum Direction { North, West, East, South, }

impl Direction {
    fn is_direction(&self, from: Coordinate, to: Coordinate) -> bool {
        match self {
            Direction::North => from.0 == to.0 && from.1 == to.1+1,
            Direction::West => from.0+1 == to.0 && from.1 == to.1,
            Direction::East => from.0 == to.0+1 && from.1 == to.1,
            Direction::South => from.0 == to.0 && from.1+1 == to.1,
        }
    }
}

struct Map{
    path: HashSet<Coordinate>,
    slopes: HashSet<(Coordinate, Direction)>,
    start: Coordinate,
    dest: Coordinate,
}

impl<'a> TryFrom<&'a str> for Map {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut path = HashSet::new();
        let mut slopes = HashSet::new();

        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => (),
                    '.' => _ = path.insert(Coordinate(x, y)),
                    '^' => _ = slopes.insert((Coordinate(x, y), Direction::North)),
                    '<' => _ = slopes.insert((Coordinate(x, y), Direction::East)),
                    '>' => _ = slopes.insert((Coordinate(x, y), Direction::West)),
                    'v' => _ = slopes.insert((Coordinate(x, y), Direction::South)),
                    e => return Err(Self::Error::InvalidChar(e)),
                }
            }
        }

        let first: Vec<_> = path.iter().filter(|c| c.1 == 0).collect();
        let start = match first.len() {
            0 => Err(Self::Error::NoStartError),
            1 => Ok(*first[0]),
            _ => Err(Self::Error::NoUniqueStartError),
        }?;
        
        let last: Vec<_> = path.iter().filter(|c| c.1 == value.lines().count()-1).collect();
        let dest = match last.len() {
            0 => Err(Self::Error::NoDestError),
            1 => Ok(*last[0]),
            _ => Err(Self::Error::NoUniqueDestError),
        }?;
        

        Ok(Self{ path, slopes, start, dest })
    }
}

impl Map {
    fn distances(&self, steep_slopes: bool) -> Vec<Vec<(usize, usize)>> {
        let mut res = Vec::new();
        let mut nodes = vec![self.start, self.dest];
        self.path.iter().filter(|n| self.neighbours(**n).len() > 2).for_each(|n| nodes.push(*n));
        self.slopes.iter().for_each(|s| nodes.push(s.0));

        nodes.iter().for_each(|from_node| {
            let mut this = Vec::new();
            self.neighbours(*from_node).iter().for_each(|&n| {
                let slope = self.slopes.iter().find(|(pos, _dir)| pos == from_node);
                if !steep_slopes || slope.is_none() || slope.unwrap().1.is_direction(*from_node, n) {
                    let mut prev = *from_node;
                    let mut curr = n;
                    let mut len = 1;
                    loop {
                        if let Some(to_idx) = nodes.iter().position(|n| n == &curr) {
                            this.push((to_idx, len));
                            break;
                        }
                        if let Some(&next) = self.neighbours(curr).iter().find(|&&next| next != prev) {
                            len += 1;
                            prev = curr;
                            curr = next;
                        } else {
                            break;
                        }
                    }
                }
            });
            res.push(this);
        });
        while let Some(idx) = res.iter().position(|d| d.len() == 2) {
            let middle = &res[idx];
            let left_idx = middle[0].0;
            let right_idx = middle[1].0;
            let d = middle[0].1 + middle[1].1;
            res[idx] = Vec::new();
            res[left_idx].iter_mut().for_each(|(dest, len)| {
                if *dest == idx {
                    *dest = right_idx;
                    *len = d;
                }
            });
            res[right_idx].iter_mut().for_each(|(dest, len)| {
                if *dest == idx {
                    *dest = left_idx;
                    *len = d;
                }
            });
        }
        res
    }


    fn neighbours(&self, pos: Coordinate) -> Vec<Coordinate> {
        let mut res = Vec::new();
        if pos.0 > 0 && self.path.contains(&Coordinate(pos.0-1, pos.1)) { res.push(Coordinate(pos.0-1, pos.1)); }
        if pos.1 > 0 && self.path.contains(&Coordinate(pos.0, pos.1-1)) { res.push(Coordinate(pos.0, pos.1-1)); }
        if self.path.contains(&Coordinate(pos.0+1, pos.1)) { res.push(Coordinate(pos.0+1, pos.1)); }
        if self.path.contains(&Coordinate(pos.0, pos.1+1)) { res.push(Coordinate(pos.0, pos.1+1)); }

        if pos.0 > 0 && self.slopes.iter().any(|(p, _)| p == &Coordinate(pos.0-1, pos.1)) { res.push(Coordinate(pos.0-1, pos.1)); }
        if pos.1 > 0 && self.slopes.iter().any(|(p, _)| p == &Coordinate(pos.0, pos.1-1)) { res.push(Coordinate(pos.0, pos.1-1)); }
        if self.slopes.iter().any(|(p, _)| p == &Coordinate(pos.0+1, pos.1)) { res.push(Coordinate(pos.0+1, pos.1)); }
        if self.slopes.iter().any(|(p, _)| p == &Coordinate(pos.0, pos.1+1)) { res.push(Coordinate(pos.0, pos.1+1)); }

        res
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let map = Map::try_from(input)?;
    let distances = map.distances(true);
    let first = longest_route(&distances);
    let distances = map.distances(false);
    let second = longest_route(&distances);
    Ok((first, second))
}

fn longest_route(distances: &[Vec<(usize, usize)>]) -> usize {
    let mut longest_so_far = 0;
    let mut open_set = vec![vec![0]];
    while let Some(path) = open_set.pop() {
        let curr = *path.last().unwrap();
        if curr == 1 {
            let len = path.windows(2).map(|w| distances[w[0]].iter().find(|&&(to, _len)| to == w[1]).unwrap().1).sum();
            if len > longest_so_far {
                longest_so_far = len;
            }
            continue;
        }
        distances[curr].iter().filter(|(n, _)| !path.contains(n)).for_each(|(next, _len)| {
            let mut new = path.to_vec();
            new.push(*next);
            open_set.push(new);
        });
    }
    longest_so_far
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
        assert_eq!(run(&sample_input), Ok((94, 154)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((2222, 6590))); // >6066
    }
}
