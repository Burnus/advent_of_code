use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InputMalformed(String),
    LineMalformed(String),
    ParseIntError(std::num::ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputMalformed(v) => write!(f, "Input is malformed: {v}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

enum Fold {
    X(usize),
    Y(usize),
}

impl TryFrom<&str> for Fold {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(value) = value.split_whitespace().nth(2) {
            if let Some((axis, index)) = value.split_once('=') {
                match axis {
                    "x" => Ok(Self::X(index.parse()?)),
                    "y" => Ok(Self::Y(index.parse()?)),
                    _ => Err(Self::Error::LineMalformed(value.to_string())),
                }
            } else {
                Err(Self::Error::LineMalformed(value.to_string()))
            }
        } else {
            Err(Self::Error::LineMalformed(value.to_string()))
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl TryFrom<&str> for Point {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((x, y)) = value.split_once(',') {
            Ok(Self { 
                x: x.parse()?,
                y: y.parse()?,
            })
        } else {
            Err(Self::Error::LineMalformed(value.to_string()))
        }
    }
}

impl Point {
    fn fold(&mut self, fold: &Fold) {
        match fold {
            Fold::X(at) => {
                    if self.x > *at {
                        self.x = 2 * *at - self.x;
                    }
                },
            Fold::Y(at) => {
                    if self.y > *at {
                        self.y = 2 * *at - self.y;
                    }
                },
        }
    }
}

pub fn run(input: &str) -> Result<(usize, String), ParseError> {
    if let Some((dots, folds)) = input.split_once("\n\n") {
        let mut dots: Vec<_> = dots.lines().map(Point::try_from).collect::<Result<Vec<_>, _>>()?;
        let folds: Vec<_> = folds.lines().map(Fold::try_from).collect::<Result<Vec<_>, _>>()?;
        dots.iter_mut().for_each(|dot| dot.fold(&folds[0]));
        dots.sort();
        dots.dedup();
        let first = dots.len();
        for fold in folds.iter().skip(1) {
            dots.iter_mut().for_each(|dot| dot.fold(fold));
            dots.sort();
            dots.dedup();
        }
        let second = print(&dots);
        Ok((first, second))
    } else {
        Err(ParseError::InputMalformed(String::from("No empty line found")))
    }
}

fn print(points: &[Point]) -> String {
    let x_max = points.iter().map(|p| p.x).max();
    let y_max = points.iter().map(|p| p.y).max();
    if let Some(x_max) = x_max {
        if let Some(y_max) = y_max {
            (0..=y_max).map(|y| (0..=x_max).map(|x| {
                let this = Point{ x, y };
                if points.contains(&this) {
                    '#'
                } else {
                    ' '
                }
            }).chain(['\n'].into_iter()).collect::<String>()).collect()
        } else {
            String::new()
        }
    } else {
        String::new()
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
        let expected = "
#####
#   #
#   #
#   #
#####
";
        assert_eq!(run(&sample_input), Ok((17, expected[1..].to_string())));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        let expected = "
#    #  # ###  #### ###  ###  ###  #  #
#    # #  #  # #    #  # #  # #  # # # 
#    ##   #  # ###  ###  #  # #  # ##  
#    # #  ###  #    #  # ###  ###  # # 
#    # #  # #  #    #  # #    # #  # # 
#### #  # #  # #### ###  #    #  # #  #
";
        assert_eq!(run(&challenge_input), Ok((653, expected[1..].to_string())));
    }
}
