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

#[derive(Clone)]
struct Brick {
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
    rests_on: Vec<usize>,
}

impl<'a> TryFrom<&'a str> for Brick {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let components: Vec<_> = value.split([',', '~']).collect();
        if components.len() != 6 {
            dbg!(&components, components.len());
            return Err(Self::Error::LineMalformed(value));
        }
        let x = (components[0].parse()?, components[3].parse()?);
        let y = (components[1].parse()?, components[4].parse()?);
        let z = (components[2].parse()?, components[5].parse()?);

        Ok(Self { x, y, z, rests_on: Vec::new(), })
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut bricks: Vec<_> = input.lines().map(Brick::try_from).collect::<Result<Vec<_>, _>>()?;
    let dependents = depedencies(&mut bricks);
    let first = dependents.iter().filter(|d| d == &&0).count();
    let second = dependents.iter().sum();
    Ok((first, second))
}

fn depedencies(bricks: &mut [Brick]) -> Vec<usize> {
    bricks.sort_by_key(|b| b.z.0);

    (0..bricks.len()).for_each(|idx| {
        let mut curr = bricks[idx].clone();
        while curr.z.0 > 1 {
            let rests_on: Vec<_> = bricks.iter().enumerate().take(idx).filter(|(_idx, other)| rests_on(&curr, other)).map(|(idx, _b)| idx).collect();
            if !rests_on.is_empty() {
                bricks[idx].rests_on = rests_on;
                break;
            }
            curr.z.0 -= 1;
            curr.z.1 -= 1;
        }
        bricks[idx].z.0 = curr.z.0;
        bricks[idx].z.1 = curr.z.1;
    });
    let mut dependent = vec![vec![]; bricks.len()];
    bricks.iter().enumerate().for_each(|(idx, brick)| {
        if brick.rests_on.len() == 1 {
            dependent[brick.rests_on[0]].push(idx);
        }
    });
    loop {
        let mut updated = false;

        bricks.iter().enumerate().for_each(|(idx, brick)| {
            let other: Vec<_> = dependent.iter()
                .take(idx)
                .enumerate()
                .filter(|(_other_idx, other)| !other.contains(&idx) && !brick.rests_on.is_empty() && brick.rests_on.iter().all(|resting| other.contains(resting)))
                .map(|(idx, _deps)| idx)
                .collect();
            if !other.is_empty() {
                updated = true;
                other.iter().for_each(|other_idx| dependent[*other_idx].push(idx));
            }
        });

        if !updated {
            break;
        }
    }
    dependent.iter().map(|d| d.len()).collect()
}

fn overlaps(lhs: (usize, usize), rhs: (usize, usize)) -> bool {
    (lhs.0..=lhs.1).contains(&rhs.0) ||
    (lhs.0..=lhs.1).contains(&rhs.1) ||
    (rhs.0..=rhs.1).contains(&lhs.0)
}

fn rests_on(upper: &Brick, lower: &Brick) -> bool {
    upper.z.0 == lower.z.1 + 1 &&
        overlaps(upper.x, lower.x) &&
        overlaps(upper.y, lower.y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    fn test_overlaps() {
        let test = [
            ((0, 2), (1, 3), true),
            ((1, 3), (2, 0), true),
            ((0, 2), (1, 1), true),
            ((2, 2), (1, 3), true),
            ((0, 2), (2, 2), true),
            ((0, 2), (2, 3), true),
            ((0, 2), (3, 4), false),
        ];
        for (lhs, rhs, expected) in test {
            assert_eq!(overlaps(lhs, rhs), expected);
        }
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), Ok((5, 7)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((471, 68525)));
    }
}
