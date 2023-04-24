use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    LineMalformed(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

#[derive(Default)]
struct Cavern {
    is_large: bool,
    neighbours: Vec<usize>,
}

struct Network {
    caverns: Vec<Cavern>,
}

impl TryFrom<&str> for Network {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut caverns = vec![Cavern::default(), Cavern::default()];
        let mut names = vec!["start", "end"];

        for line in value.lines() {
            if let Some((lhs, rhs)) = line.split_once('-') {
                let lhs = match names.iter().position(|name| *name == lhs) {
                    Some(idx) => idx,
                    None => {
                        let is_large = lhs.chars().next().unwrap().is_uppercase();
                        names.push(lhs);
                        caverns.push(Cavern { is_large, ..Default::default() });
                        caverns.len()-1
                    }
                };
                let rhs = match names.iter().position(|name| *name == rhs) {
                    Some(idx) => idx,
                    None => {
                        let is_large = rhs.chars().next().unwrap().is_uppercase();
                        names.push(rhs);
                        caverns.push(Cavern { is_large, ..Default::default() });
                        caverns.len()-1
                    }
                };
                caverns[lhs].neighbours.push(rhs);
                caverns[rhs].neighbours.push(lhs);
            } else {
                return Err(Self::Error::LineMalformed(line.to_string()));
            }
        }
        Ok(Self { caverns })
    }
}

impl Network {
    fn get_paths(&self, revisits: usize) -> Vec<Vec<usize>> {
        let mut open_set = vec![(vec![0], revisits)];
        let mut res = Vec::new();

        while let Some(path) = open_set.pop() {
            let current = path.0[path.0.len()-1];
            if current == 1 {
                res.push(path.0);
            } else {
                for neighbour in &self.caverns[current].neighbours {
                    if self.caverns[*neighbour].is_large || !path.0.contains(neighbour) {
                        let mut new_path = path.0.clone();
                        new_path.push(*neighbour);
                        open_set.push((new_path, path.1));
                    } else if path.1 > 0 && *neighbour > 0 {
                        let mut new_path = path.0.clone();
                        new_path.push(*neighbour);
                        open_set.push((new_path, path.1-1));
                    }
                }
            }
        }
        res
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let network = Network::try_from(input)?;
    let first = network.get_paths(0).len();
    let second = network.get_paths(1).len();
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
        assert_eq!(run(&sample_input), Ok((226, 3509)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((3708, 93858)));
    }
}
