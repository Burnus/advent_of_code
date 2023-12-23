use core::fmt::Display;
use std::{num::ParseIntError, collections::{HashSet, BinaryHeap, HashMap}};

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
    fn longest_route(&self, steep_slopes: bool) -> usize {
        let mut open_set = BinaryHeap::from([SearchState{ len: 0, pos: self.start, visited: HashSet::from([self.start]), }]);
        // let mut longest = HashMap::new();
        let mut longest_so_far = 0;
        // let mut coming_from = HashMap::new();

        while let Some(state) = open_set.pop() {
            let (len, pos, vis) = (state.len, state.pos, state.visited, );
            if pos == self.dest {
                longest_so_far = longest_so_far.max(len);
            }

            for neighbour in self.neighbours(pos, steep_slopes) {
                // if !path_contains(&coming_from, pos, neighbour.0) && !open_set.iter().any(|p| p.len >= len+neighbour.1 && p.pos == neighbour.0) 
                if !vis.contains(&neighbour.0) { // && *longest.get(&(pos, neighbour.0)).unwrap_or(&0) <= len+neighbour.1 
                    let mut visited = vis.clone();
                    visited.insert(neighbour.0);
                    open_set.push(SearchState { len: len+neighbour.1, pos: neighbour.0, visited });
                    // longest.insert((pos, neighbour.0), len+neighbour.1);
                    // coming_from.insert(neighbour.0, pos);
                }
            }
        }
        longest_so_far
    }

    fn distances(&self) -> Vec<Vec<(usize, usize)>> {
        let mut res = Vec::new();
        let mut nodes = vec![self.start, self.dest];
        self.path.iter().filter(|n| self.neighbours(**n, false).len() > 2).for_each(|n| nodes.push(*n));
        self.slopes.iter().for_each(|s| nodes.push(s.0));

        nodes.iter().for_each(|from_node| {
            let mut this = Vec::new();
            self.neighbours(*from_node, false).iter().for_each(|n| {
                let mut prev = *from_node;
                let mut curr = n.0;
                let mut len = 1;
                loop {
                    if let Some(to_idx) = nodes.iter().position(|n| n == &curr) {
                        this.push((to_idx, len));
                        break;
                    }
                    if let Some(&(next, _)) = self.neighbours(curr, false).iter().find(|&&(next, _)| next != prev) {
                        len += 1;
                        prev = curr;
                        curr = next;
                    } else {
                        break;
                    }
                }
            });
            res.push(this);
        });
        res
    }


    fn neighbours(&self, pos: Coordinate, steep_slopes: bool) -> Vec<(Coordinate, usize)> {
        let mut res = Vec::new();
        if pos.0 > 0 && self.path.contains(&Coordinate(pos.0-1, pos.1)) { res.push((Coordinate(pos.0-1, pos.1), 1)); }
        if pos.1 > 0 && self.path.contains(&Coordinate(pos.0, pos.1-1)) { res.push((Coordinate(pos.0, pos.1-1), 1)); }
        if self.path.contains(&Coordinate(pos.0+1, pos.1)) { res.push((Coordinate(pos.0+1, pos.1), 1)); }
        if self.path.contains(&Coordinate(pos.0, pos.1+1)) { res.push((Coordinate(pos.0, pos.1+1), 1)); }

        if steep_slopes {
            if pos.0 > 0 && self.slopes.contains(&(Coordinate(pos.0-1, pos.1), Direction::East)) { res.push((Coordinate(pos.0-2, pos.1), 2)); }
            if pos.1 > 0 && self.slopes.contains(&(Coordinate(pos.0, pos.1-1), Direction::North)) { res.push((Coordinate(pos.0, pos.1-2), 2)); }
            if self.slopes.contains(&(Coordinate(pos.0+1, pos.1), Direction::West)) { res.push((Coordinate(pos.0+2, pos.1), 2)); }
            if self.slopes.contains(&(Coordinate(pos.0, pos.1+1), Direction::South)) { res.push((Coordinate(pos.0, pos.1+2), 2)); }
        } else {
            if pos.0 > 0 && self.slopes.iter().any(|(p, _)| p == &Coordinate(pos.0-1, pos.1)) { res.push((Coordinate(pos.0-1, pos.1), 1)); }
            if pos.1 > 0 && self.slopes.iter().any(|(p, _)| p == &Coordinate(pos.0, pos.1-1)) { res.push((Coordinate(pos.0, pos.1-1), 1)); }
            if self.slopes.iter().any(|(p, _)| p == &Coordinate(pos.0+1, pos.1)) { res.push((Coordinate(pos.0+1, pos.1), 1)); }
            if self.slopes.iter().any(|(p, _)| p == &Coordinate(pos.0, pos.1+1)) { res.push((Coordinate(pos.0, pos.1+1), 1)); }
        }

        res
    }
}

#[derive(PartialEq, Eq)]
struct SearchState {
    len: usize,
    pos: Coordinate,
    visited: HashSet<Coordinate>,
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.len.cmp(&other.len)
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let map = Map::try_from(input)?;
    let first = map.longest_route(true);
    let distances = map.distances();
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

fn path_contains(path: &HashMap<Coordinate, Coordinate>, dest: Coordinate, query: Coordinate) -> bool {
    if let Some(&prev) = path.get(&dest) {
        if prev == query {
            false
        } else {
            path_contains(path, prev, query)
        }
    } else {
        true
    }
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
