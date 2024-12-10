use core::fmt::Display;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyMap,
    InvalidHeight(char),
    NonRectangular,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyMap => write!(f, "Input must not be empty."),
            Self::InvalidHeight(e) => write!(f, "Unable to parse {e} into a height"),
            Self::NonRectangular => write!(f, "All input lines must have equal length"),
        }
    }
}

type Coordinates = (isize, isize);

struct Map {
    tiles: Vec<Vec<u8>>,
    trailheads: Vec<(Coordinates, usize, usize)>,
    width: isize,
    height: isize,
}

impl TryFrom<&str> for Map {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let width = value.lines().next().ok_or(Self::Error::EmptyMap).map(|line| line.len())?;
        let mut trailheads = Vec::new();
        let tiles = value.lines().enumerate().map(|(y, line)| {
            if line.len() != width {
                return Err(Self::Error::NonRectangular);
            }
            line.chars().enumerate().map(|(x, c)| {
                match c {
                    '0' => {
                        trailheads.push(((x as isize, y as isize), 0, 0));
                        Ok(0)
                    },
                    d if d.is_ascii_digit() => Ok(d as u8 - b'0'),
                    e => Err(Self::Error::InvalidHeight(e)),
                }

            }).collect::<Result<_, _>>()
        }).collect::<Result<Vec<_>, _>>()?;

        let mut res = Self {
            height: tiles.len() as isize,
            tiles,
            trailheads,
            width: width as isize,
        };
        (0..res.trailheads.len()).for_each(|idx| res.rate_trailhead(idx));
        
        Ok(res)
    }
}

impl Map {
    /// Returns the elevation at valid map `Coordinates`, or `u8::MAX`, if they are out of bounds.
    fn elevation(&self, at: Coordinates) -> u8 {
        if at.0 >= 0 && at.1 >= 0 && at.0 < self.width && at.1 < self.height {
            self.tiles[at.1 as usize][at.0 as usize]
        } else {
            u8::MAX
        }
    }

    /// Assigns the score and rating to the trailhead at the given index.
    fn rate_trailhead(&mut self, trailhead_idx: usize) {
        let (score, rating) = if let Some (trailhead) = self.trailheads.get(trailhead_idx) {
            let start = trailhead.0;
            let mut open_set = Vec::from([(start, 0)]);
            let mut destinations = HashSet::new();
            let mut trails = 0;
            while let Some((pos, elevation)) = open_set.pop() {
                if elevation == 9 {
                    trails += 1;
                    destinations.insert(pos);
                    continue;
                }
                [(-1,0), (1,0), (0,-1), (0,1)]
                    .iter()
                    .filter(|(dx, dy)| self.elevation((pos.0+dx, pos.1+dy)) == elevation + 1)
                    .for_each(|(dx, dy)| {
                        let next = (pos.0+dx, pos.1+dy);
                        open_set.push((next, elevation+1));
                    });
            }
            (destinations.len(), trails)
        } else {
            return
        };
        self.trailheads[trailhead_idx].1 = score;
        self.trailheads[trailhead_idx].2 = rating;
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let map = Map::try_from(input)?;
    let first = map.trailheads.iter().map(|&t| t.1).sum();
    let second = map.trailheads.iter().map(|&t| t.2).sum();
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
        assert_eq!(run(&sample_input), Ok((36, 81)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((535, 1186)));
    }
}
