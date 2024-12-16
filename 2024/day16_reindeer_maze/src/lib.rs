use core::fmt::Display;
use std::{cmp::Ordering, collections::{BTreeSet, HashMap, HashSet}};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyMap,
    InvalidTile(char),
    NoEnd,
    NoStart,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyMap => write!(f, "Map cannot be empty"),
            Self::InvalidTile(e) => write!(f, "Unable to parse {e} into a map tile. Only '#', '.', 'S', and 'E' are allowed."),
            Self::NoEnd => write!(f, "No end point ('E') found in input"),
            Self::NoStart => write!(f, "No starting point ('E') found in input"),
        }
    }
}

type Coordinates = (isize, isize);

struct Map {
    walls: HashSet<Coordinates>,
    start: Coordinates,
    end: Coordinates,
}

impl TryFrom<&str> for Map {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut walls = HashSet::new();
        let mut start = None;
        let mut end = None;

        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => _ = walls.insert((x as isize, y as isize)),
                    'S' => start = Some((x as isize, y as isize)),
                    'E' => end = Some((x as isize, y as isize)),
                    '.' => (),
                    e => return Err(Self::Error::InvalidTile(e)),
                }
            }
        }
        if let Some(start) = start {
            if let Some(end) = end {
                Ok(Self { walls, start, end, })
            } else {
                Err(Self::Error::NoEnd)
            }
        } else {
            Err(Self::Error::NoStart)
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SearchState {
    costs: usize,
    pos: Coordinates,
    facing: Coordinates,
}

impl Map {
    fn solve(&self) -> (usize, Vec<Coordinates>) {
        let start = SearchState {
            costs: 0,
            pos: self.start,
            facing: (1, 0),
        };
        let mut open_set = BTreeSet::from([start]);
        let mut visited = HashSet::new();
        let mut came_from: HashMap<(Coordinates, Coordinates), (Vec<Coordinates>, usize)> = HashMap::new();
        let mut best_costs = usize::MAX;
        while let Some(curr) = open_set.pop_first() {
            if curr.costs > best_costs {
                break;
            }
            if curr.pos == self.end {
                best_costs = curr.costs
            }
            if visited.contains(&(curr.pos, curr.facing)) {
                continue;
            }
            visited.insert((curr.pos, curr.facing));
            neighbours(curr.pos, curr.facing)
                .iter()
                .filter(|(pos, _facing, _costs)| !self.walls.contains(pos))
                .for_each(|&(pos, facing, addidtional_costs)| {
                    let costs = curr.costs + addidtional_costs;
                    open_set.insert(SearchState { pos, facing, costs, });
                    came_from.entry((pos, facing))
                        .and_modify(|(was_facing, best_costs)| {
                            match costs.cmp(best_costs) {
                                Ordering::Less => *was_facing = Vec::from([curr.facing]),
                                Ordering::Equal => was_facing.push(curr.facing),
                                Ordering::Greater => (),
                            }
                        }).or_insert((Vec::from([curr.facing]), costs));
                });
        }
        let mut path = Vec::from([self.end]);
        let mut open = came_from.keys().filter(|(pos, _facing)| *pos == self.end).copied().collect::<Vec<_>>();
        while let Some(this) = open.pop() {
            came_from.get(&this)
                .unwrap()
                .0
                .iter()
                .for_each(|prev_facing| {
                    let prev_pos = (this.0.0 - prev_facing.0, this.0.1 - prev_facing.1);
                    if !path.contains(&prev_pos) && this.0 != self.start {
                        path.push(prev_pos);
                        open.push((prev_pos, *prev_facing));
                    }
                });
        }
        

        (best_costs, path)
    }
}

fn neighbours(pos: Coordinates, facing: Coordinates) -> [(Coordinates, Coordinates, usize); 4] {
    [
        ((pos.0+facing.0, pos.1+facing.1), facing, 1),  // go ahead
        (pos, (facing.1, facing.0), 1000),              // turn one way
        (pos, (-facing.1, -facing.0), 1000),            // turn other way
        (pos, (-facing.0, -facing.1), 2000),            // turn around
    ]
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let map = Map::try_from(input)?;
    let path = map.solve();
    let first = path.0;
    let second = path.1.len();
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
        assert_eq!(run(&sample_input), Ok((7036, 45)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((101492, 543)));
    }
}
