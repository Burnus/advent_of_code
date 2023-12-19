use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    InvalidDirection(&'a str),
    LineMalformed(&'a str),
    ParseDirError(usize),
    ParseIntError(std::num::ParseIntError),
}

struct ParseDirError(usize);

impl From<ParseDirError> for ParseError<'_> {
    fn from(value: ParseDirError) -> Self {
        Self::ParseDirError(value.0)
    }
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDirection(d) => write!(f, "Unable to parse \"{d}\" into a direction. Value needs to be U, D, L, or R."),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseDirError(d) => write!(f, "Unable to parse {d} into a direction. Value needs to be within [0..=3]."),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl TryFrom<usize> for Direction {
    type Error = ParseDirError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::Right),
            1 => Ok(Direction::Down),
            2 => Ok(Direction::Left),
            3 => Ok(Direction::Up),
            e => Err(ParseDirError(e)),
        }
    }
}

struct Trench {
	dir_v1: Direction,
	len_v1: usize,
	dir_v2: Direction,
	len_v2: usize,
}

impl<'a> TryFrom<&'a str> for Trench {
	type Error = ParseError<'a>;
	
	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		let components: Vec<_> = value.split_whitespace().collect();
		if components.len() != 3 {
			return Err(Self::Error::LineMalformed(value));
		}
		let dir_v1 = match components[0] {
			"U" => Ok(Direction::Up),
			"D" => Ok(Direction::Down),
			"L" => Ok(Direction::Left),
			"R" => Ok(Direction::Right),
			e => Err(Self::Error::InvalidDirection(e))
		}?;
		let len_v1 = components[1].parse()?;
		let colour = usize::from_str_radix(&components[2][2..components[2].len()-1], 16)?;
        let len_v2 = colour/16;
        let dir_v2 = Direction::try_from(colour%4)?;
		Ok(Self{ dir_v1, len_v1, dir_v2, len_v2, })
	}
}

fn lagoon_size(trenches: &[Trench], v2: bool) -> usize {
	let mut curr = (0, 0);
    let mut corners = Vec::from([curr]);
	trenches.iter().for_each(|trench| {
        let (dir, len) = if v2 { (trench.dir_v2, trench.len_v2) } else { (trench.dir_v1, trench.len_v1) };
        curr = match dir {
            Direction::Up => (curr.0, curr.1 - len as isize),
			Direction::Down => (curr.0, curr.1 + len as isize),
			Direction::Left => (curr.0 - len as isize, curr.1),
			Direction::Right => (curr.0 + len as isize, curr.1),
		};
		corners.push(curr);
    });
	segment_size(&corners)
}

fn segment_size(corners: &[(isize, isize)]) -> usize {
	let (mut min_x, mut max_x, mut min_y, mut max_y) = (isize::MAX, isize::MIN, isize::MAX, isize::MIN);
	corners.iter().for_each(|(x, y)| {
		min_x = min_x.min(*x);
		max_x = max_x.max(*x);
		min_y = min_y.min(*y);
		max_y = max_y.max(*y);
	});
    if min_x == max_x || min_y == max_y {
        return 0;
    }
	match corners.len() {
		f if f < 3 => 0,
		3 => ((max_x-min_x)*(max_y-min_y)) as usize,
		4 if corners[0].0 == corners[3].0 => ((max_x-min_x)*(max_y-min_y-1)) as usize,
		4 => ((max_x-min_x-1)*(max_y-min_y)) as usize,
		n => {
				// find all trench corners on the edges of this segment
				let mut outside_corner_indexes : Vec<_> = corners.iter().enumerate().filter(|(_idx, (x, y))| [min_x, max_x].contains(x) || [min_y, max_y].contains(y)).map(|(idx, _corner)| idx).collect();
				let mut res = if corners[0] == corners[n-1] {
				    // push the first one to the end to find segments that wrap around the end of our slice
				    outside_corner_indexes.push(outside_corner_indexes[0]);
					// We are looking at the lagoon itself, which is the only case of a closed segment. Consider anything including the trench.
					((max_x+1-min_x)*(max_y+1-min_y)) as usize
                } else {
					// If we have an even number of corners, the segment is in the middle of the edge. We need to consider the whole area and deduct the trench afterwards.
					((max_x+1-min_x)*(max_y+1-min_y)) as usize - corners.windows(2).map(|w| w[0].0.abs_diff(w[1].0)+ w[0].1.abs_diff(w[1].1)).sum::<usize>() - 1
				};
				
				// deduct the size of each segment on our edge
				for w in outside_corner_indexes.windows(2) {
					if w[1] > w[0] {
						res -= segment_size(&corners[w[0]..=w[1]]);
					} else {
						res -= segment_size(&[&corners[w[0]..], &corners[..=w[1]]].concat());
					}
				}
				
				res
			},
	}
} 

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let trenches: Vec<_> = input.lines().map(Trench::try_from).collect::<Result<Vec<_>, _>>()?;
    let first = lagoon_size(&trenches, false);
    let second = lagoon_size(&trenches, true);
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
    fn segment_size_test() {
        let data = [
			vec![(0, 0), (3, 0), (3, 4), (0, 4), (0, 0)],
			vec![(0, 0), (2, 0), (2, 2), (4, 2), (4, 4), (0, 4), (0, 0)],
			vec![(0, 0), (2, 0), (2, 1), (3, 1), (3, 2), (4, 2), (4, 4), (0, 4), (0, 0)],
			vec![(0, 0), (2, 0), (2, 1), (1, 1), (1, 4), (6, 4), (6, 2), (5, 2), (5, 0), (7, 0), (7, 5), (0, 5), (0, 0)],
			vec![(0, 3), (5, 3), (5, 0)],
			vec![(0, 3), (3, 3), (3, 2), (5, 2), (5, 0)],
			vec![(2, 0), (2, 1), (1, 1), (1, 3), (5, 3), (5, 0)],
		];
		
		let expected = [
			20,
			21,
			22,
			37,
			15,
			13,
			7,
		];
		
		for (idx, corners) in data.iter().enumerate() {
			assert_eq!(segment_size(corners), expected[idx]);
		}
    }
	
    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), Ok((62, 952408144115)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((34329, 42617947302920)));
    }
}

