use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ParseIntError(std::num::ParseIntError),
    LineMalformed(String),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

struct Grid3D {
    cubes_active: Vec<Vec<Vec<bool>>>,
    max: (usize, usize, usize),
}

impl TryFrom<&str> for Grid3D {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let y_count = value.lines().count() + 14;
        let x_count = value.lines().next().unwrap().len() + 14;
        let z_count = 15;
        let empty_plane = vec![vec![false; x_count]; y_count];

        let mut init_plane = vec![vec![false; x_count]; 7];
        for line in value.lines() {
            let mut row = vec![false; 7];
            for c in line.chars() {
                match c {
                    '#' => row.push(true),
                    '.' => row.push(false),
                    _ => return Err(Self::Error::LineMalformed(line.to_string())),
                }
            }
            row.append(&mut vec![false; 7]);
            init_plane.push(row);
        }
        init_plane.append(&mut vec![vec![false; x_count]; 7]);
        let mut cubes_active = vec![empty_plane.to_vec(); 7];
        cubes_active.push(init_plane);
        cubes_active.append(&mut vec![empty_plane; 7]);

        Ok(Self { cubes_active, max: (x_count-1, y_count-1, z_count-1) })
    }
}

impl Grid3D {
    fn active_neighbours(&self, (x, y, z): (usize, usize, usize)) -> usize {
        if x == 0 || y == 0 || z == 0 || x == self.max.0 || y == self.max.1 || z == self.max.2 {
            0
        } else {
            (0..3).map(|dx| (0..3).map(|dy| (0..3).map(|dz| 
                match (dx, dy, dz, self.cubes_active[z+dz-1][y+dy-1][x+dx-1]) {
                    (1, 1, 1, _) => 0,
                    (_, _, _, false) => 0,
                    _ => 1,
                }).sum::<usize>()).sum::<usize>()).sum()
        }
    }

    fn step(&mut self) {
        let mut next = self.cubes_active.clone();
        next.iter_mut().enumerate().for_each(|(z, plane)| {
            plane.iter_mut().enumerate().for_each(|(y, row)| {
                row.iter_mut().enumerate().for_each(|(x, cube)| {
                    match (self.cubes_active[z][y][x], self.active_neighbours((x, y, z))) {
                        (true, 2) | (_, 3) => *cube = true,
                        _ => *cube = false,
                    }
                });
            });
        });
        std::mem::swap(&mut next, &mut self.cubes_active);
    }

    fn active_cubes(&self) -> usize {
        self.cubes_active.iter().map(|plane| plane.iter().map(|row| row.iter().filter(|cube| **cube).count()).sum::<usize>()).sum()
    }
}

struct Grid4D {
    cubes_active: Vec<Vec<Vec<Vec<bool>>>>,
    max: (usize, usize, usize, usize),
}

impl From<&Grid3D> for Grid4D {
    fn from(value: &Grid3D) -> Self {
        let empty_cube = vec![vec![vec![false; value.max.0+1]; value.max.1+1]; value.max.2+1];
        let mut cubes_active = vec![empty_cube.clone(); 7];
        cubes_active.push(value.cubes_active.clone());
        cubes_active.append(&mut vec![empty_cube.clone(); 7]);

        Self {
            cubes_active,
            max: (value.max.0, value.max.1, value.max.2, 14),
        }
    }
}

impl Grid4D {
    fn active_neighbours(&self, (x, y, z, w): (usize, usize, usize, usize)) -> usize {
        if x == 0 || y == 0 || z == 0 || w == 0 || x == self.max.0 || y == self.max.1 || z == self.max.2 || w == self.max.3 {
            0
        } else {
            (0..3).map(|dx| (0..3).map(|dy| (0..3).map(|dz| (0..3).map(|dw| 
                match (dx, dy, dz, dw, self.cubes_active[w+dw-1][z+dz-1][y+dy-1][x+dx-1]) {
                    (1, 1, 1, 1, _) => 0,
                    (_, _, _, _, false) => 0,
                    _ => 1,
                }).sum::<usize>()).sum::<usize>()).sum::<usize>()).sum()
        }
    }

    fn step(&mut self) {
        let mut next = self.cubes_active.clone();
        next.iter_mut().enumerate().for_each(|(w, dim)| {
            dim.iter_mut().enumerate().for_each(|(z, plane)| {
                plane.iter_mut().enumerate().for_each(|(y, row)| {
                    row.iter_mut().enumerate().for_each(|(x, cube)| {
                        match (self.cubes_active[w][z][y][x], self.active_neighbours((x, y, z, w))) {
                            (true, 2) | (_, 3) => *cube = true,
                            _ => *cube = false,
                        }
                    });
                });
            });
        });
        std::mem::swap(&mut next, &mut self.cubes_active);
    }

    fn active_cubes(&self) -> usize {
        self.cubes_active.iter().map(|dim| dim.iter().map(|plane| plane.iter().map(|row| row.iter().filter(|cube| **cube).count()).sum::<usize>()).sum::<usize>()).sum()
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut grid_3d = Grid3D::try_from(input)?;
    let mut grid_4d = Grid4D::from(&grid_3d);
    for _round in 0..6 {
        grid_3d.step();
        grid_4d.step();
    }
    let first = grid_3d.active_cubes();
    let second = grid_4d.active_cubes();
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
        assert_eq!(run(&sample_input), Ok((112, 848)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((304, 1868)));
    }
}
