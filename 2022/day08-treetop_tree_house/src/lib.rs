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

fn get_visibility(forest: &[Vec<u8>]) -> usize {
    let rows = forest.len();
    let cols = forest[0].len();

    let mut visible = (rows+cols-2)*2; // all border trees are already visible and have a scenic score of 0.
    for row in 1..forest.len()-1 {
        for col in 1..forest[0].len()-1 {
            let tree_height = forest[row][col];

            if forest[row].iter()
                    .take(col)
                    .max()
                    .unwrap() < &tree_height ||
                forest[row].iter()
                    .skip(col+1)
                    .max()
                    .unwrap() < &tree_height ||
                forest.iter()
                    .take(row)
                    .map(|row| row[col])
                    .max()
                    .unwrap() < tree_height ||
                forest.iter()
                    .skip(row+1)
                    .map(|row| row[col])
                    .max()
                    .unwrap() < tree_height {
                        visible += 1;
                    }
        }
    }
    visible
}

fn get_scenic_score(forest: &[Vec<u8>]) -> usize {
    let rows = forest.len();
    let cols = forest[0].len();

    let mut highest_scenic_score = 0;
    for row in 1..forest.len()-1 {
        for col in 1..forest[0].len()-1 {
            let tree_height = forest[row][col];

            let mut scenic_score = 1;
            let mut this_factor = 0;
            for this_col in (0..col).rev() {
                this_factor += 1;
                if forest[row][this_col] >= tree_height { break; }
            }
            scenic_score *= this_factor;
            this_factor = 0;

            for this_col in col+1..cols {
                this_factor += 1;
                if forest[row][this_col] >= tree_height { break; }
            }
            scenic_score *= this_factor;
            this_factor = 0;

            for this_row in (0..row).rev() {
                this_factor += 1;
                if forest[this_row][col] >= tree_height { break; }
            }
            scenic_score *= this_factor;
            this_factor = 0;

            for this_row in row+1..rows {
                this_factor += 1;
                if forest[this_row][col] >= tree_height { break; }
            }
            scenic_score *= this_factor;

            highest_scenic_score = highest_scenic_score.max(scenic_score);
        }
    }
    highest_scenic_score
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let forest = input.lines().map(|row| row.chars().map(|t| t.to_digit(10).map(|d| d as u8).ok_or(ParseError::ParseIntError(t))).collect()).collect::<Result<Vec<Vec<_>>, _>>()?;
    let first = get_visibility(&forest);
    let second = get_scenic_score(&forest);
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
        assert_eq!(run(&sample_input), Ok((21, 8)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((1695, 287040)));
    }
}
