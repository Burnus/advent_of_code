use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InputMalformed(String),
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
            Self::InputMalformed(v) => write!(f, "Input is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

struct BingoBoard {
    rows: [[usize; 5]; 5],
    ticked: Vec<(usize, usize)>,
    done: bool,
}

impl TryFrom<&str> for BingoBoard {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut rows = [[0; 5]; 5];
        let lines: Vec<_> = value.lines().collect();
        if lines.len() != 5 {
            return Err(Self::Error::InputMalformed(value.to_string()));
        }
        for (y, line) in lines.iter().enumerate() {
            let numbers: Vec<_> = line.split_whitespace().collect();
            if numbers.len() != 5 {
                return Err(Self::Error::LineMalformed(line.to_string()));
            }
            for (x, n) in numbers.iter().enumerate() {
                rows[y][x] = n.parse()?;
            }
        }

        Ok(Self{ rows, ticked: Vec::new(), done: false })
    }
}

impl BingoBoard {
    fn tick(&mut self, number: usize) -> Option<usize> {
        if !self.done {
            if let Some((x, y)) = self.rows.iter().enumerate().find(|(_y, row)| row.contains(&number)).map(|(y, row)| (row.iter().position(|n| *n == number).unwrap(), y)) {
                self.ticked.push((x, y));
                if self.ticked.iter().filter(|(col, _row)| *col == x).count() == 5 ||
                    self.ticked.iter().filter(|(_col, row)| *row == y).count() == 5 {
                        self.done = true;
                        return Some(number);
                    }
            }
        }
        None
    }

    fn sum_unmarked(&self) -> usize {
        self.rows.iter().enumerate().map(|(y, row)| row.iter().enumerate().filter(|(x, _n)| !self.ticked.contains(&(*x, y))).map(|(_x, n)| n).sum::<usize>()).sum()
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let items: Vec<_> = input.split("\n\n").collect();
    let numbers: Vec<_> = items[0].split(',').map(|i| i.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
    let mut boards: Vec<_> = items[1..].iter().map(|b| BingoBoard::try_from(*b)).collect::<Result<Vec<_>, _>>()?;
    let mut todo = boards.len();
    let mut first = 0;
    let mut second = 0;
    'outer: for number in numbers {
        for board in boards.iter_mut() {
            if let Some(last) = board.tick(number) {
                if first == 0 {
                    first = last * board.sum_unmarked();
                } else if todo == 1 {
                    second = last * board.sum_unmarked();
                    break 'outer;
                }
                todo -= 1;
            }
        }
    };
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
        assert_eq!(run(&sample_input), Ok((4512, 1924)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((45031, 2568)));
    }
}
