use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidChar(char)
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChar(c) => write!(f, "Invalid Character encountered: {c}"),
        }
    }
}

struct Grid {
    trees: Vec<Vec<bool>>,
}

impl TryFrom<&str> for Grid {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            trees: value.lines().map(|line| line.chars().map(|c| match c {
                '.' => Ok(false),
                '#' => Ok(true),
                _ => Err(ParseError::InvalidChar(c)),
            }).collect::<Result<Vec<_>, _>>()).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Grid {
    fn trees_hit_by_going(&self, (right, down): (usize, usize)) -> usize {
            self.trees.iter().enumerate().step_by(down).filter(|(idx, row)| row[(right*idx/down)%row.len()]).count()
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let grid = Grid::try_from(input)?;
    let first = grid.trees_hit_by_going((3, 1));
    let slopes = [(1, 1), (5, 1), (7, 1), (1, 2)];
    let second = slopes.iter().map(|slope| grid.trees_hit_by_going(*slope)).product::<usize>() * first;
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
        assert_eq!(run(&sample_input), Ok((7, 336)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((272, 3898725600)));
    }
}
