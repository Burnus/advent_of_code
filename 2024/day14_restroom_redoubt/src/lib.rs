use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    ParseIntError(std::num::ParseIntError),
    LineMalformed(&'a str),
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

type Coordinates = (isize, isize);
struct Robot {
    pos: Coordinates,
    dir: Coordinates,
}

impl<'a> TryFrom<&'a str> for Robot {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let componnents: Vec<_> = value.split(&['=', ',', ' ']).collect();
        if componnents.len() != 6 {
            return Err(Self::Error::LineMalformed(value));
        }
        let pos = (componnents[1].parse()?, componnents[2].parse()?);
        let dir = (componnents[4].parse()?, componnents[5].parse()?);

        Ok(Self{ pos, dir, })
    }
}

impl Robot {
    fn pos_after(&self, steps: isize, map_size: Coordinates) -> Coordinates {
        let new = ((self.pos.0 + steps * self.dir.0) % map_size.0, (self.pos.1 + steps * self.dir.1) % map_size.1);
        match (new.0.signum(), new.1.signum()) {
            (-1, -1) => (new.0 + map_size.0, new.1 + map_size.1),
            (-1, _) => (new.0 + map_size.0, new.1),
            (_, -1) => (new.0, new.1 + map_size.1),
            _ => new,
        }
    }

    fn quadrant_after(&self, steps: isize, map_size: Coordinates) -> usize {
        let pos = self.pos_after(steps, map_size);
        let mid = (map_size.0 / 2, map_size.1 / 2);
        match ((pos.0-mid.0).signum(), (pos.1-mid.1).signum()) {
            (0, _) | (_, 0) => 0,
            (-1, -1) => 1,
            (1, -1)  => 2,
            (-1, 1)  => 3,
            (1, 1)   => 4,
            _ => unreachable!()
        }
    }

    fn step(&mut self, map_size: Coordinates) {
        self.pos = self.pos_after(1, map_size);
    }
}

fn quadrant_robot_counts_after(robots: &[Robot], steps: isize, map_size: Coordinates) -> [usize; 5] {
    let mut res = [0; 5];
    robots.iter().for_each(|r| res[r.quadrant_after(steps, map_size)] += 1);
    res
}

// Assumes the first configuration without overlapping robots is the desired result
fn find_christmas_tree(robots: &mut [Robot], map_size: Coordinates) -> usize {
    for step in 0.. {
        if robots.iter().enumerate().all(|(idx, r1)| !robots.iter().skip(idx+1).any(|r2| r1.pos == r2.pos)) {
            // Print the whole map to visually confirm we indeed found a christmas tree
            //
            // let mut tiles = vec![vec![false; map_size.0 as usize]; map_size.1 as usize];
            // robots.iter().for_each(|r| tiles[r.pos.1 as usize][r.pos.0 as usize] = true);
            // tiles.iter().for_each(|row|{
            //     row.iter().for_each(|&t|
            //         print!("{}", if t { '*' } else { '.' } )
            //     );
            //     println!();
            // });
            return step;
        }
        robots.iter_mut().for_each(|r| r.step(map_size));
    }
    unreachable!()
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut robots: Vec<_> = input.lines().map(Robot::try_from).collect::<Result<Vec<_>, _>>()?;
    let map_size = if robots.iter().map(|r| r.pos.0).max().unwrap_or(0) < 11 { 
        (11, 7)
    } else { 
        (101, 103) 
    };
    let first = quadrant_robot_counts_after(&robots, 100, map_size).iter().skip(1).product();
    let second = find_christmas_tree(&mut robots, map_size);
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
        // The result for part 2 doesn't actually form a christmas tree. I doubt it ever will.
        assert_eq!(run(&sample_input), Ok((12, 1)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((229069152, 7383)));
    }
}
