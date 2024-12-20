use core::fmt::Display;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidTile(char),
    NoDest,
    NoStart,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTile(e) => write!(f, "Unable to convert {e} into a map tile. Only '.', '#', 'E', and 'S' are allowed"),
            Self::NoDest => write!(f, "No end position ('E') found in input"),
            Self::NoStart => write!(f, "No starting position ('S') found in input"),
        }
    }
}

type Coordinates = (isize, isize);

struct Map {
    walkable: HashSet<Coordinates>,
    start: Coordinates,
    dest: Coordinates,
}

impl TryFrom<&str> for Map {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut start = None;
        let mut dest = None;
        let mut walkable = HashSet::new();

        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coords = (x as isize, y as isize);
                match c {
                    '#' => (),
                    '.' => _ = walkable.insert(coords),
                    'S' => start = Some(coords),        // walkable, but why would we ever want to return?
                    'E' => {
                        dest = Some(coords);
                        walkable.insert(coords);
                    },
                    e => return Err(Self::Error::InvalidTile(e)),
                }
            }
        }

        if let Some(start) = start {
            if let Some(dest) = dest {
                Ok(Self {
                    walkable,
                    start,
                    dest,
                })
            } else {
                Err(Self::Error::NoDest)
            }
        } else {
            Err(Self::Error::NoStart)
        }

    }
}

impl Map {
    /// Returns the number of cheats of length `cheat_len` or shorter that result in at least `threshold` fewer steps.
    fn cheats(&self, cheat_len: isize, threshold: usize) -> usize {
        let mut cheats = 0;
        let dists_without_cheating = self.dists();
        dists_without_cheating.iter().map(|(&pos, &dist)| (pos, dist)).for_each(|((x, y), dist)| {
            (0..=cheat_len).for_each(|dx| {
                (0..=cheat_len-dx).for_each(|dy| {
                    [(x-dx, y-dy), (x+dx, y-dy), (x-dx, y+dy), (x+dx, y+dy)]
                        .iter()
                        .enumerate()
                        .filter(|&(idx, _pos)| (dx > 0 || idx & 1 == 0) && (dy > 0 || idx < 2))
                        .for_each(|(_idx, pos)| {
                            if let Some(old_dist) = dists_without_cheating.get(pos) {
                                if *old_dist >= dist + (dx + dy) as usize + threshold {
                                    cheats += 1
                                }
                            }
                        });

                });
            })
        });
        cheats
    }

    /// Calculates the distances needed to reach each position from the start without cheats. This
    /// doesn't rely on the fact that there is only one way through the maze.
    fn dists(&self) -> HashMap<Coordinates, usize> {
        let mut dists = HashMap::from([(self.start, 0)]);
        let mut open_set = VecDeque::from([(self.start, 0)]);
        while let Some(((x, y), dist)) = open_set.pop_front() {
            dists.insert((x, y), dist);
            if (x, y) == self.dest {
                break;
            }
            [(x-1, y), (x+1, y), (x, y-1), (x, y+1)]
                .into_iter()
                .filter(|pos| self.walkable.contains(pos))
                .for_each(|pos| {
                    if !dists.contains_key(&pos) {
                        open_set.push_back((pos, dist+1));
                    }
                });
        }
        dists
    }
}

/// Generalized run function to accomodate the different thresholds for the samle input
fn run_challenge(input: &str, thresholds: (usize, usize)) -> Result<(usize, usize), ParseError> {
    let map = Map::try_from(input)?;
    let first = map.cheats(2, thresholds.0);
    let second = map.cheats(20, thresholds.1);
    Ok((first, second))
}

pub fn run_sample(input: &str) -> Result<(usize, usize), ParseError> {
    run_challenge(input, (1, 50))
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    run_challenge(input, (100, 100))
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
        assert_eq!(run_sample(&sample_input), Ok((44, 285)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((1343, 982891)));
    }
}
