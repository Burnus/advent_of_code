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

struct Area {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl TryFrom<&str> for Area {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let components: Vec<_> = value.split(&[' ', ',']).collect();
        if components.len() == 5 {
            let x: Vec<_> = components[2].split(&['.', '=']).collect();
            let y: Vec<_> = components[4].split(&['.', '=']).collect();
            if x.len() == 4 && y.len() == 4 {
                Ok(Self {
                    x_min: x[1].parse()?, 
                    x_max: x[3].parse()?,
                    y_min: y[1].parse()?,
                    y_max: y[3].parse()?,
                })
            } else {
                Err(Self::Error::LineMalformed(value.to_string()))
            }
        } else {
            Err(Self::Error::LineMalformed(value.to_string()))
        }
    }
}

pub fn run(input: &str) -> Result<(isize, usize), ParseError> {
    let target = Area::try_from(input)?;
    let mut max_y = 0;
    let mut hits = 0;
    for y in target.y_min..-target.y_min {
        for x in 1..=target.x_max {
            let attempt = launch((0, 0), (x, y), &target, 0);
            if attempt.0 {
                max_y = max_y.max(attempt.1);
                hits += 1;
            }
        }
    }
    let first = max_y;
    let second = hits;
    Ok((first, second))
}

fn launch((x, y): (isize, isize), (x_vel, y_vel): (isize, isize), target: &Area, max_y: isize) -> (bool, isize) {
    if (target.x_min..=target.x_max).contains(&x) && (target.y_min..=target.y_max).contains(&y) {
        (true, max_y)
    } else if x_vel >= 0 && target.x_max < x ||
              x_vel <= 0 && target.x_min > x ||
              y_vel <= 0 && target.y_min > y {
        (false, 0)
    } else if x_vel > 0 {
        launch((x+x_vel, y+y_vel), (x_vel-1, y_vel-1), target, max_y.max(y+y_vel))
    } else if x_vel < 0 {
        launch((x+x_vel, y+y_vel), (x_vel+1, y_vel-1), target, max_y.max(y+y_vel))
    } else {
        launch((x+x_vel, y+y_vel), (0, y_vel-1), target, max_y.max(y+y_vel))
    }
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
        let target = Area::try_from(&sample_input[..]).unwrap();
        let should_hit = [
            (7, 2),
            (6, 3),
            (9, 0),
        ];
        let should_miss = [ (17, -4) ];
        for vel in should_hit {
            assert!(launch((0, 0), vel, &target, 0).0);
        }
        for vel in should_miss {
            assert!(!launch((0, 0), vel, &target, 0).0);
        }
        assert_eq!(run(&sample_input), Ok((45, 112)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((6903, 2351)));
    }
}
