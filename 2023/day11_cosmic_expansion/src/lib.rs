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

fn galaxies_from_image(input: &str, expansion: usize) -> Vec<(usize, usize)> {
    let mut row = 0;
    let mut galaxies = Vec::new();
    input.lines().for_each(|line| {
        let curr_line: Vec<_> = line.match_indices('#').collect();
        if curr_line.is_empty() {
            row += expansion;
        } else {
            curr_line.iter().for_each(|(col, _)| galaxies.push((row, *col)));
            row += 1;
        }
    });
    galaxies.sort_by(|a, b| b.1.cmp(&a.1));
    (0..galaxies.len()-1).for_each(|idx| {
        let diff = galaxies[idx].1 - galaxies[idx+1].1;
        if diff > 1 {
            galaxies.iter_mut().take(idx+1).for_each(|(_row, col)| *col += (diff-1)*expansion-1);
        }
    });
    galaxies
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let galaxies: Vec<_> = galaxies_from_image(input, 2);
    let first = galaxies.iter().enumerate().map(|(idx, (y1, x1))| galaxies.iter().skip(idx+1).map(|(y2, x2)| y1.abs_diff(*y2) + x1.abs_diff(*x2)).sum::<usize>()).sum();
    let galaxies: Vec<_> = galaxies_from_image(input, 1000000);
    let second = galaxies.iter().enumerate().map(|(idx, (y1, x1))| galaxies.iter().skip(idx+1).map(|(y2, x2)| y1.abs_diff(*y2) + x1.abs_diff(*x2)).sum::<usize>()).sum();
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
        assert_eq!(run(&sample_input), Ok((374, 82000210)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((10228230, 447073334102)));
    }
}
