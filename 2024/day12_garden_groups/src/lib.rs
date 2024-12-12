use core::fmt::Display;
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyMap,
    NonRectangular,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyMap => write!(f, "Input must not be empty"),
            Self::NonRectangular => write!(f, "All input lines must be of equal length"),
        }
    }
}

type Plant = u8;

struct Map {
    tiles: Vec<Plant>,
    height: isize,
    width: isize,
}

impl TryFrom<&str> for Map {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let width = value.lines().next().map(|line| line.len()).ok_or(Self::Error::EmptyMap)?;
        let tiles: Vec<u8> = value.lines()
            .map(|line| if line.len() == width {
                    Ok(line.as_bytes())
                } else {
                    Err(Self::Error::NonRectangular)
                })
            .collect::<Result<Vec<_>, _>>()
            .map(|v| v.iter().flat_map(|row| row.to_vec()).collect())?;
        let height = (tiles.len() / width) as isize;
        Ok(Self {
            tiles,
            height,
            width: width as isize,
        })
    }
}

impl Map {
    fn at(&self, (x, y): (isize, isize)) -> Option<Plant> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            None
        } else {
            // Some(self.tiles[y as usize][x as usize])
            Some(self.tiles[(y * self.width + x) as usize])
        }
    }

    fn regions(&self) -> Vec<(usize, usize, usize)> {
        let mut outside = BTreeSet::from([(0, 0)]);
        let mut visited = HashSet::with_capacity((self.width * self.height) as usize);
        let mut regions = Vec::new();

        // pre-allocate some per-region datastructures for performance reasons
        let mut open_set = Vec::new();
        let mut visited_this_region = HashSet::new();
        while let Some(start) = outside.pop_first() {
            if visited.contains(&start) {
                continue;
            }
            // safe to unwrap because we made sure this tile exists (for (0, 0) in the constructor,
            // and for every other tile because it wouldn't have been pushed onto `outside` otherwise)
            let this_plant = self.at(start).unwrap();
            visited.insert(start);
            open_set.clear();
            visited_this_region.clear();
            open_set.push(start);
            visited_this_region.insert(start);
            let mut area = 1;
            let mut perimeter = 0;
            let mut corners = 0;
            while let Some((x, y)) = open_set.pop() {
                [(-1,0), (1,0), (0,-1), (0,1)]
                    .iter()
                    .for_each(|(dx, dy)| {
                        let next = (x+dx, y+dy);
                        if !visited_this_region.contains(&next) {
                            match self.at(next) {
                                Some(plant) if plant == this_plant => {
                                    visited.insert(next);
                                    visited_this_region.insert(next);
                                    open_set.push(next);
                                    area += 1;
                                },
                                Some(_) => {
                                    outside.insert(next);
                                    perimeter += 1;

                                    // To count the number of corners we found here, consider this
                                    // situation:
                                    //
                                    // ......
                                    // ..AC..
                                    // ..XY..
                                    // ..BD..
                                    // ......
                                    //
                                    // Say, we are at X and looking at Y, which is a different plant.
                                    // A - D are yet unknown, other tiles are irrelevant for now. We found:
                                    // - no corner, if A and B are the same plant as X, but C and D are different;
                                    // - 2 corners, if either all or none of A - D are plant X, and
                                    // - 1 corner otherwise.
                                    //
                                    // Therefore, the  number of new corners can be expressed as:
                                    // ´2 - number of direct neighbours which are the same, but
                                    //      their corresponding diagonal is different from this plant´.
                                    corners += 2 - [((x+dy), (y+dx)), ((x-dy), (y-dx))]
                                        .into_iter()
                                        .filter(|&n| self.at(n) == Some(this_plant) && 
                                            self.at((n.0+dx, n.1+dy)) != Some(this_plant)
                                        ).count();
                                },
                                None => {
                                    perimeter += 1;

                                    // Same situation as in the Some(_) case, but we are at an
                                    // edge, so C, D, and Y are automatically different to X
                                    // (namely None), which means we always have 
                                    // ´2 - number of direct neighbours whith the same plant´
                                    // new corners.
                                    corners += 2 - [((x+dy), (y+dx)), ((x-dy), (y-dx))]
                                        .iter()
                                        .filter(|&n| self.at(*n) == Some(this_plant))
                                        .count();
                                },
                            }
                        }
                    });
            }
            // In every 2D shape, the number of sides is equal to the number of corners. However, we
            // always double-counted them, since we looked at them from two directions. Consider
            // the corner in the middle of:
            //
            // XY
            // YY
            //
            // For X, we counted it looking in rightward and downward direction, for Y in leftward and
            // upward direction (both from the bottom right tile). In any case, we counted it twice
            // for each region. Since for the X case it is irrelevant whether all the Ys are the
            // same plant (or any plant at all), and the example is unchanged by rotation, we find,
            // that this example indeed represents all possible ways that a corner could ever appear.
            // Hence, our observation must hold for every corner on the map.
            regions.push((area, perimeter, corners/2));
        }
        regions
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let map = Map::try_from(input)?;
    let regions = map.regions();
    let first = regions.iter().map(|(area, perimeter, _sides)| area * perimeter).sum();
    let second = regions.iter().map(|(area, _perimeter, sides)| area * sides).sum();
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
    fn test_samples() {
        let expected = [
            (1930, 1206),
            (140, 80),
            (772, 436),
            (692, 236),
            (1184, 368),
        ];
        for (idx, expected) in expected.into_iter().enumerate() {
            let sample_input = read_file(&format!("tests/sample_input{idx}"));
            assert_eq!(run(&sample_input), Ok(expected));
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((1400386, 851994)));
    }
}
