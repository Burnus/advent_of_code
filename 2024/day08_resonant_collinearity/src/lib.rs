use core::fmt::Display;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyMap,
    NonRectangular,
    ParseCharError(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyMap => write!(f, "Input can't be empty"),
            Self::NonRectangular => write!(f, "Input lines must be of equal length"),
            Self::ParseCharError(e) => write!(f, "Unable to parse {e} into an antenna. Must be an ASCII alphanumeric character."),
        }
    }
}

struct Map {
    antennae: Vec<Vec<(usize, usize)>>,
    height: usize,
    width: usize,
}

impl TryFrom<&str> for Map {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let height = value.lines().count();
        if height == 0 {
            return Err(Self::Error::EmptyMap);
        }
        let width = value.lines().next().unwrap().len();
        let mut antennae: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut indeces: HashMap<char, usize> = HashMap::new();

        for (y, line) in value.lines().enumerate() {
            if line.len() != width {
                return Err(Self::Error::NonRectangular);
            }
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => (),
                    a if a.is_ascii_alphanumeric() => {
                        if let Some(idx) = indeces.get(&a) {
                            antennae[*idx].push((x, y));
                        } else {
                            indeces.insert(a, antennae.len());
                            antennae.push(Vec::from([(x, y)]));
                        }
                    },
                    e => return Err(Self::Error::ParseCharError(e)),
                }
            }
        }
        Ok(Self { antennae, height, width, })
    }
}

impl Map {
    fn antinodes_of(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        if let (Some(x), Some(y)) = ((2 * x1).checked_sub(x2), (2 * y1).checked_sub(y2)) {
            if x < self.width && y < self.height {
                res.push((x, y));
            }
        }
        if let (Some(x), Some(y)) = ((2 * x2).checked_sub(x1), (2 * y2).checked_sub(y1)) {
            if x < self.width && y < self.height {
                res.push((x, y));
            }
        }
        res
    }

    fn simple_antinodes(&self) -> HashSet<(usize, usize)> {
        let mut antinodes = HashSet::new();

        self.antennae.iter().for_each(|ants| 
            ants.iter().enumerate().for_each(|(idx1, &a1)|
                ants.iter().enumerate().filter(|(idx2, _a2)| *idx2 > idx1).for_each(|(_idx2, &a2)| 
                    self.antinodes_of(a1, a2).iter().for_each(|antinode| { antinodes.insert(*antinode); })
                )));

        antinodes
    }

    fn harmonic_antinodes_of(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for i in 1.. {
            if let (Some(x), Some(y)) = ((i*x1).checked_sub((i-1)*x2), (i*y1).checked_sub((i-1)*y2)) {
                if x < self.width && y < self.height {
                    res.push((x, y));
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        for i in 1.. {
            if let (Some(x), Some(y)) = ((i*x2).checked_sub((i-1)*x1), (i*y2).checked_sub((i-1)*y1)) {
                if x < self.width && y < self.height {
                    res.push((x, y));
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        res
    }

    fn antinodes_with_harmonics(&self) -> HashSet<(usize, usize)> {
        let mut antinodes = HashSet::new();

        self.antennae.iter().for_each(|ants| 
            ants.iter().enumerate().for_each(|(idx1, &a1)|
                ants.iter().enumerate().filter(|(idx2, _a2)| *idx2 > idx1).for_each(|(_idx2, &a2)| 
                    self.harmonic_antinodes_of(a1, a2).iter().for_each(|antinode| { antinodes.insert(*antinode); })
                )));

        antinodes
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let map = Map::try_from(input)?;
    let first = map.simple_antinodes().len();
    let second = map.antinodes_with_harmonics().len();
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
        assert_eq!(run(&sample_input), Ok((14, 34)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((354, 1263)));
    }
}
