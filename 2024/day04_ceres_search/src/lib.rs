use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    NonRectangular,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonRectangular => write!(f, "All lines of the input must have equal length"),
        }
    }
}

struct Grid<'a> {
    chars: Vec<&'a [u8]>,
    width: usize,
    height: usize,
}

impl<'a> TryFrom<&'a str> for Grid<'a> {
    type Error = ParseError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let chars: Vec<_> = value.lines().map(|l| l.as_bytes()).collect();
        let height = chars.len();
        let width = chars[0].len();
        if chars.iter().any(|l| l.len() != width) {
            Err(Self::Error::NonRectangular)
        } else {
            Ok(Self { chars, width, height })
        }
    }
}

impl Grid<'_> {
    fn count_xmas(&self) -> usize {
        (0..self.height as isize).map(|y| 
            (0..self.width as isize).filter(|x| self.chars[y as usize][*x as usize] == b'X').map(|x|
                [(1, 0), (-1, 0), (0, 1), (0, -1), (-1, -1), (-1, 1), (1, -1), (1, 1)]
                    .into_iter()
                    .filter(|&(dx, dy)| (0..self.width as isize).contains(&(x + 3 * dx)) &&
                        (0..self.height as isize).contains(&(y + 3 * dy)) &&
                        self.chars[(y + dy) as usize][(x + dx) as usize] == b'M' &&
                        self.chars[(y + 2 * dy) as usize][(x + 2 * dx) as usize] == b'A' &&
                        self.chars[(y + 3 * dy) as usize][(x + 3 * dx) as usize] == b'S'
                    ).count()
            ).sum::<usize>()
        ).sum::<usize>()
    }

    fn count_mas_crosses(&self) -> usize {
        (1..self.height as isize - 1).map(|y| 
            (1..self.width as isize - 1).filter(|x| self.chars[y as usize][*x as usize] == b'A').map(|x|
                [(-1, -1), (-1, 1), (1, -1), (1, 1)]
                    .into_iter()
                    .filter(|&(dx, dy)| 
                            self.chars[(y + dy) as usize][(x + dx) as usize] == b'M' && 
                            self.chars[(y - dy) as usize][(x - dx) as usize] == b'S')
                    .count() / 2
            ).sum::<usize>()
        ).sum::<usize>()
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let grid = Grid::try_from(input)?;
    let first = grid.count_xmas();
    let second = grid.count_mas_crosses();
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
        assert_eq!(run(&sample_input), Ok((18, 9)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((2530, 1921)));
    }
}
