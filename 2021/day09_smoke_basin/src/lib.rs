use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ParseIntError(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseIntError(c) => write!(f, "Unable to parse {c} into integer"),
        }
    }
}

struct Map {
    heights: Vec<Vec<usize>>,
}

impl TryFrom<&str> for Map {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            heights: value.lines().map(|line| line.chars().map(|c| c.to_digit(10).ok_or(Self::Error::ParseIntError(c)).map(|i| i as usize)).collect::<Result<Vec<_>, _>>()).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Map {
    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<((usize, usize), usize)> {
        [(1, 0), (0, 1), (2, 1), (1, 2)].iter().filter(|(dx, dy)| (1..=self.heights[0].len()).contains(&(x+dx)) && (1..=self.heights.len()).contains(&(y+dy))).map(|(dx, dy)| ((x+dx-1, y+dy-1), self.heights[y+dy-1][x+dx-1])).collect()
    }

    fn is_local_low_point(&self, (x, y): (usize, usize)) -> bool {
        let this_height = self.heights[y][x];

        self.neighbours((x, y)).iter().all(|(_coords, height)| height > &this_height)
    }

    fn get_local_low_points(&self) -> Vec<usize> {
        self.heights.iter()
                    .enumerate()
                    .flat_map(|(y, row)| row.iter()
                                            .enumerate()
                                            .filter(|(x, _height)| self.is_local_low_point((*x, y)))
                                            .map(|(_x, height)| *height)
                                            .collect::<Vec<_>>())
                    .collect()
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut map = Map::try_from(input)?;
    let first = map.get_local_low_points().iter().map(|height| height+1).sum();
    let second = get_largest_basin(&mut map);
    Ok((first, second))
}

fn get_largest_basin(map: &mut Map) -> usize {
    let height = map.heights.len();
    let width = map.heights[0].len();

    let mut basins = Vec::new();

    // While it's tempting to only iterate over the local lows from step 1, we cannot garranty that
    // every basin contains a local low point (there may be multiple consecutive points forming a
    // valley of equal height). Therefore we iterate over the entire array.
    (0..height).for_each(|y| {
        (0..width).for_each(|x| {
            let mut this_basin = 0;

            let mut open_set = vec![(x, y)];
            while let Some((x, y)) = open_set.pop() {
                if map.heights[y][x] == 9 {
                    continue;
                }
                this_basin += 1;
                map.heights[y][x] = 9;

                for neighbour in map.neighbours((x, y)) {
                    open_set.push(neighbour.0);
                }
            }
            basins.push(this_basin);
        });
    });

    basins.sort_by(|a, b| b.cmp(&a));
    basins.iter().take(3).product()
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
        assert_eq!(run(&sample_input), Ok((15, 1134)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((524, 1235430)));
    }
}
