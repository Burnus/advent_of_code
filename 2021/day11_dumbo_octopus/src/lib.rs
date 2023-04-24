use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ParseIntError(char),
    LineMalformed(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseIntError(c) => write!(f, "Unable to parse {c} into integer"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

struct OctopusGrid {
    grid: Vec<Vec<u8>>,
    flashes: usize,
}

impl TryFrom<&str> for OctopusGrid {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self { 
            grid: value.lines()
                       .map(|line| line.chars().map(|c| c.to_digit(10).map(|d| d as u8).ok_or(Self::Error::ParseIntError(c))).collect::<Result<Vec<_>, _>>())
                       .collect::<Result<Vec<_>, _>>()?,
            flashes: 0,
        })
    }
}
impl OctopusGrid {
    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        (0..3).flat_map(|dx| (0..3).filter(|dy| (dx != 1 || *dy != 1) && (1..=self.grid.len()).contains(&(x+dx)) && (1..=self.grid[0].len()).contains(&(y+dy))).map(|dy| (x+dx-1, y+dy-1)).collect::<Vec<_>>()).collect()
    }

    fn increase(&mut self, (x, y): (usize, usize)) {
        self.grid[y][x] += 1;
        if self.grid[y][x] == 10 {
            self.flashes += 1;
            self.neighbours((x, y)).iter().for_each(|n| self.increase(*n));
        }
    }
    
    fn step(&mut self) {
        (0..self.grid.len()).for_each(|y| (0..self.grid[y].len()).for_each(|x| self.increase((x, y))));
        self.grid.iter_mut().for_each(|row| row.iter_mut().for_each(|energy_level| {
            if *energy_level > 9 {
                *energy_level = 0;
            }
        }));
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut grid = OctopusGrid::try_from(input)?;
    for _ in 1..=100 {
        grid.step();
    }
    let first = grid.flashes;
    let mut second = 0;
    let mut last_flashes = first;
    for step in 101.. {
        grid.step();
        if grid.flashes - last_flashes == 100 {
            second = step;
            break;
        }
        last_flashes = grid.flashes;
    }
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
        assert_eq!(run(&sample_input), Ok((1656, 195)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((1603, 222)));
    }
}
