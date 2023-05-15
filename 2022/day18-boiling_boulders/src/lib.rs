use core::fmt::Display;
use std::{num::ParseIntError, collections::BTreeSet};

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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Voxel {
    x: i8,
    y: i8,
    z: i8,
}

impl<'a> TryFrom<&'a str> for Voxel {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let components = value.split(',').collect::<Vec<&str>>();
        if components.len() != 3 {
            Err(Self::Error::LineMalformed(value))
        } else {
            Ok(Self {
                x: components[0].parse()?,
                y: components[1].parse()?,
                z: components[2].parse()?,
            })
        }
    }
}

impl Voxel {
    fn neighbours(&self) -> [Self; 6] {
        [
            Self { x: self.x-1, y: self.y, z: self.z },
            Self { x: self.x+1, y: self.y, z: self.z },
            Self { x: self.x, y: self.y-1, z: self.z },
            Self { x: self.x, y: self.y+1, z: self.z },
            Self { x: self.x, y: self.y, z: self.z-1 },
            Self { x: self.x, y: self.y, z: self.z+1 },
        ]
    }
}

fn find_total_surface_area(voxels: &BTreeSet<Voxel>) -> usize {
    voxels.iter()
        .map(|v| 6 - v.neighbours().iter().filter(|n| voxels.contains(n)).count())
        .sum()
}

fn find_area_reachable_from_origin(voxels: &BTreeSet<Voxel>) -> usize {
    let max_x = voxels.last().unwrap().x + 1;
    let max_y = voxels.iter().map(|v| v.y).max().unwrap() + 1;
    let max_z = voxels.iter().map(|v| v.z).max().unwrap() + 1;
    
    let mut water = BTreeSet::from([Voxel { x: 0, y: 0, z: 0 }]);
    let mut water_last_step = water.clone();
    loop {
        let mut water_this_step = BTreeSet::new();
        for droplet in &water_last_step {
            for neighbour in droplet.neighbours() {
                if !water.contains(&neighbour) && !voxels.contains(&neighbour) && (-1..=max_x).contains(&neighbour.x) && (-1..=max_y).contains(&neighbour.y) && (-1..=max_z).contains(&neighbour.z) {
                    water_this_step.insert(neighbour);
                    water.insert(neighbour);
                }
            }
        }
        if water_this_step.is_empty() {
            break;
        }
        std::mem::swap(&mut water_this_step, &mut water_last_step);
    }

    voxels.iter()
        .map(|v| v.neighbours().iter().filter(|n| water.contains(n)).count())
        .sum()
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let voxels: BTreeSet<_> = input.lines().map(Voxel::try_from).collect::<Result<BTreeSet<_>, _>>()?;
    let first = find_total_surface_area(&voxels);
    let second = find_area_reachable_from_origin(&voxels);
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
        assert_eq!(run(&sample_input), Ok((64, 58)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((4320, 2456)));
    }
}
