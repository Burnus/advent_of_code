use core::fmt::Display;
use std::collections::{HashMap, BinaryHeap};

#[derive(Debug, PartialEq, Eq)]
pub enum MapError {
    NoPath,
    ParseIntError(char),
}

impl Display for MapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoPath => write!(f, "Unable to find any path from start to destination"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

#[derive(PartialEq, Eq)]
enum CrucibleType { Normal, Ultra, }

impl CrucibleType {
    fn min(&self) -> u8 {
        match self {
            Self::Normal => 1,
            Self::Ultra => 4,
        }
    }

    fn max(&self) -> u8 {
        match self {
            Self::Normal => 3,
            Self::Ultra => 10,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    East,
    South,
}

impl Direction {
    fn coming_from(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Self::North => (x, y-1),
            Self::West => (x-1, y),
            Self::East => (x+1, y),
            Self::South => (x, y+1),
        }
    }

    fn perpendicular(&self) -> [Self; 2] {
        match self {
            Self::North | Self::South => [Self::West, Self::East],
            Self::West | Self::East => [Self::North, Self::South],
        }
    }

    fn positions_with(&self, (x, y): (usize, usize), dist: u8, crucible_type: &CrucibleType) -> Vec<((usize, usize), Self, u8)> {
        let (min, max) = (crucible_type.min(), crucible_type.max());
        match (self, dist, x, y) {
            (Self::North, s, _, 0) if s < min => Vec::new(),
            (Self::West, s, 0, _) if s < min => Vec::new(),
            (d, s, x, y) if s < min => vec![(d.coming_from((x, y)), *d, s+1)],
            (d, m, x, y) if m == max => d.perpendicular()
                                            .iter()
                                            .filter(|dir| (x > 0 || dir != &&Self::West) && (y > 0 || dir != &&Self::North))
                                            .map(|dir| (dir.coming_from((x, y)), *dir, 1))
                                            .collect(),
            (d, n, x, y) => d.perpendicular()
                                            .iter()
                                            .map(|dir| (dir, 1))
                                            .chain(std::iter::once((d, n+1)))
                                            .filter(|(dir, _dist)| (x > 0 || dir != &&Self::West) && (y > 0 || dir != &&Self::North))
                                            .map(|(dir, dist)| (dir.coming_from((x, y)), *dir, dist))
                                            .collect(),

        }
    }
}

#[derive(PartialEq, Eq)]
struct State {
    estimated_costs: usize,
    pos: (usize, usize),
    dir: Direction,
    dist: u8,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.estimated_costs.cmp(&self.estimated_costs)
    }
}

pub fn run(input: &str) -> Result<(usize, usize), MapError> {
    let map = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).ok_or(MapError::ParseIntError(c)).map(|n|n as u8)).collect::<Result<Vec<u8>, _>>()).collect::<Result<Vec<_>, _>>()?;
    let first = cheapest_path(&map, &CrucibleType::Normal)?;
    let second = cheapest_path(&map, &CrucibleType::Ultra)?;
    Ok((first, second))
}

fn cheapest_path(map: &[Vec<u8>], crucible_type: &CrucibleType) -> Result<usize, MapError> {
    let dest = (map.last().unwrap().len()-1, map.len()-1);
    let mut costs = HashMap::from([(((0, 0), Direction::East, 0), 0), (((0, 0), Direction::South, 0), 0)]);
    let mut open_set = BinaryHeap::from([
                                      State{ estimated_costs: dest.0 + dest.1, pos: (0, 0), dir: Direction::East, dist: 0, }, 
                                      State{ estimated_costs: dest.0 + dest.1, pos: (0, 0), dir: Direction::South, dist: 0, },
                        ]);

    while let Some(state) = open_set.pop() {
        let (pos, dir, dist) = (state.pos, state.dir, state.dist);
        let old_costs = *costs.get(&(pos, dir, dist)).unwrap();
        if pos == dest {
            if dist >= crucible_type.min() {
                return Ok(old_costs);
            } else {
                continue;
            }
        }
        for (new_pos, new_dir, new_dist) in dir.positions_with(pos, dist, crucible_type) {
            if new_pos.1 <= dest.1 && new_pos.0 < map[new_pos.1].len() {
                let new_costs = old_costs + map[new_pos.1][new_pos.0] as usize;
                let new_est = new_costs + dest.0.abs_diff(new_pos.0) + dest.1.abs_diff(new_pos.1);
                let best_so_far = if new_dist > crucible_type.min() {
                    (crucible_type.min()..=new_dist).map(|i| *costs.get(&(new_pos, new_dir, i)).unwrap_or(&usize::MAX)).min().unwrap()
                } else {
                    *costs.get(&(new_pos, new_dir, new_dist)).unwrap_or(&usize::MAX)
                };
                if new_est < best_so_far {
                    costs.insert((new_pos, new_dir, new_dist), new_costs);
                    open_set.push(State{ estimated_costs: new_est, pos: new_pos, dir: new_dir, dist: new_dist, });
                }
            }
        }
    }
    Err(MapError::NoPath)
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
        assert_eq!(run(&sample_input), Ok((102, 94)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((855, 980)));
    }
}
