use core::fmt;
use std::{iter::Sum, ops::AddAssign, num::ParseIntError};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    x: isize, 
    y: isize, 
    z: isize,
}

impl Sum for Coordinate {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        iter.for_each(|coord| {
            x += coord.x;
            y += coord.y;
            z += coord.z;
        });

        Self { x, y, z }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Coordinate {
    fn energy(&self) -> usize {
        self.x.unsigned_abs() + self.y.unsigned_abs() + self.z.unsigned_abs()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseMoonError {
    ParseIntError(std::num::ParseIntError),
    InvalidCoordinates(usize),
}

impl From<ParseIntError> for ParseMoonError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl fmt::Display for ParseMoonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseIntError(v) => write!(f, "Error parsing coordinates: {v}"),
            Self::InvalidCoordinates(n) => write!(f, "Error reading the coordinates list. It contains {n} components instead of 7."),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Moon {
    position: Coordinate,
    velocity: Coordinate,
}

impl TryFrom<&str> for Moon {
    type Error = ParseMoonError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let components: Vec<_> = value.split(&['=', ',', '>']).collect();
        if components.len() != 7 {
            return Err(ParseMoonError::InvalidCoordinates(components.len()));
        }
        let x = components[1].parse()?;
        let y = components[3].parse()?;
        let z = components[5].parse()?;
        Ok(Self {
            position: Coordinate { x, y, z, },
            velocity: Coordinate { x: 0, y: 0, z: 0, },
        })
    }
}

impl Moon {
    fn get_energy(&self) -> usize {
        self.position.energy() * self.velocity.energy()
    }
}

struct LunarSystem {
    moons: Vec<Moon>,
}

impl LunarSystem {
    fn step_motion(&mut self) {
        let positions: Vec<_> = self.moons.iter().map(|moon| moon.position).collect();
        for moon in self.moons.iter_mut() {
            let delta_v: Coordinate = positions.iter().map(|other| Coordinate { x: (other.x - moon.position.x).signum(), y: (other.y - moon.position.y).signum(), z: (other.z - moon.position.z).signum() }).sum();
            moon.velocity += delta_v;
        }
        for moon in self.moons.iter_mut() {
            moon.position += moon.velocity;
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseMoonError> {
    let moons: Vec<Moon> = input.lines().map(Moon::try_from).collect::<Result<Vec<Moon>, _>>()?;
    //.unwrap_or_else(|err| panic!("Error parsing input into moons: {err}"));
    let mut system = LunarSystem { moons: moons.to_vec(), };
    for _ in 0..1000 {
        system.step_motion();
    }
    let first = system.moons.iter().map(|moon| moon.get_energy()).sum();

    system.moons = moons.to_vec();
    let mut periods = [0; 3];
    for step in 1.. {
        system.step_motion();
        if system.moons.iter().enumerate().all(|(idx, moon)| periods[0] == 0 && moon.velocity.x == 0 && moon.position.x == moons[idx].position.x) {
            periods[0] = step;
            if periods[1] > 0 && periods[2] > 0 {
                break;
            }
        }
        if system.moons.iter().enumerate().all(|(idx, moon)| periods[1] == 0 && moon.velocity.y == 0 && moon.position.y == moons[idx].position.y) {
            periods[1] = step;
            if periods[0] > 0 && periods[2] > 0 {
                break;
            }
        }
        if system.moons.iter().enumerate().all(|(idx, moon)| periods[2] == 0 && moon.velocity.z == 0 && moon.position.z == moons[idx].position.z) {
            periods[2] = step;
            if periods[0] > 0 && periods[1] > 0 {
                break;
            }
        }
    }
    let second = scm(periods[0], scm(periods[1], periods[2]));
    Ok((first, second))
}

fn scm(lhs: usize, rhs: usize) -> usize {
    let l = lhs.max(rhs);
    let s = lhs.min(rhs);

    for i in 1.. {
        if (i*l) % s == 0 {
            return i*l;
        }
    }
    unreachable!("The loop always runs");
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
        assert_eq!(run(&sample_input), Ok((14645, 4686774924)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((7687, 334945516288044)));
    }
}
