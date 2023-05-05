use core::fmt::Display;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InputMalformed,
    InvalidChar(char),
    LineMalformed(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputMalformed => write!(f, "Input does not consist of two parts, separated by an empty line"),
            Self::InvalidChar(c) => write!(f, "Invalid Character {c} encountered"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

struct Image {
    bright_pixels: HashSet<(isize, isize)>,
    x_range: (isize, isize),
    y_range: (isize, isize),
    outer: bool,
}

impl From<&str> for Image {
    fn from(value: &str) -> Self {
        let bright_pixels: HashSet<(isize, isize)> = value.lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars()
                 .enumerate()
                 .filter(|(_x, c)| *c == '#')
                 .map(|(x, _c)| (x as isize, y as isize))
                 .collect::<Vec<_>>()
            ).collect();
        let x_max = *bright_pixels.iter().map(|(x, _y)| x).max().unwrap();
        let y_max = *bright_pixels.iter().map(|(_x, y)| y).max().unwrap();
        Self { 
            bright_pixels,
            x_range: (0, x_max),
            y_range: (0, y_max),
            outer: false,
        }
    }
}

impl Image {
    fn get(&self, (x, y): (isize, isize)) -> bool {
        if (self.x_range.0..=self.x_range.1).contains(&x) && (self.y_range.0..=self.y_range.1).contains(&y) {
            self.bright_pixels.contains(&(x, y))
        } else {
            self.outer
        }
    }

    fn enhance(&mut self, lookup_table: &[bool]) {
        let mut new_pixels = HashSet::new();
        (self.y_range.0-1..=self.y_range.1+1).for_each(|y|
            (self.x_range.0-1..=self.x_range.1+1).for_each(|x| {
                let mut idx = 0;
                (-1..=1).for_each(|dy| (-1..=1).for_each(|dx| {
                     idx *= 2;
                     if self.get((x+dx, y+dy)) {
                         idx += 1;
                     }
                }));
                if lookup_table[idx] {
                    new_pixels.insert((x, y));
                }
            }));
        std::mem::swap(&mut new_pixels, &mut self.bright_pixels);
        self.x_range = (self.x_range.0-1, self.x_range.1+1);
        self.y_range = (self.y_range.0-1, self.y_range.1+1);
        if self.outer && !lookup_table[511] {
            self.outer = false;
        } else if !self.outer && lookup_table[0] {
            self.outer = true;
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let (lookup_table, scan) = input.split_once("\n\n").ok_or(ParseError::InputMalformed)?;
    let lookup_table = try_to_bool_vec(lookup_table)?;
    let mut image = Image::from(scan);
    for _ in 0..2 {
        image.enhance(&lookup_table);
    }
    let first = image.bright_pixels.len();
    for _ in 2..50 {
        image.enhance(&lookup_table);
    }
    let second = image.bright_pixels.len();
    Ok((first, second))
}

fn try_to_bool_vec(input: &str) -> Result<Vec<bool>, ParseError> {
    if input.len() != 512 {
        return Err(ParseError::LineMalformed(input.to_string()));
    }
    input.bytes().map(|b| match b{
            b'.' => Ok(false),
            b'#' => Ok(true),
            o => Err(ParseError::InvalidChar(o as char)),
        }).collect::<Result<Vec<_>, _>>()
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
        assert_eq!(run(&sample_input), Ok((35, 3351)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((5819, 18516)));
    }
}
