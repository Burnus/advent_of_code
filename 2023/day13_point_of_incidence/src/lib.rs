use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidChar(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChar(c) => write!(f, "Unable to parse {c} into a pixel"),
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let imgs_by_row: Vec<_> = input.split("\n\n").map(try_parse_image).collect::<Result<Vec<_>, _>>()?;
    let imgs_by_col: Vec<_> = imgs_by_row.iter().map(|img| transpose(img)).collect();
    let first = 100 * imgs_by_row.iter().map(|img| reflection_idx(img)).sum::<usize>() + imgs_by_col.iter().map(|img| reflection_idx(img)).sum::<usize>();
    let second = 100 * imgs_by_row.iter().map(|img| smudged_reflection_idx(img)).sum::<usize>() + imgs_by_col.iter().map(|img| smudged_reflection_idx(img)).sum::<usize>();
    Ok((first, second))
}

fn try_parse_image(value: &str) -> Result<Vec<u32>, ParseError> {
    value.lines()
        .map(try_parse_line)
        .collect::<Result<Vec<_>, _>>()
}

fn try_parse_line(line: &str) -> Result<u32, ParseError> {
    let mut res = 0;
    for c in line.chars() {
        match c {
            '.' => res <<= 1,
            '#' => res = (res<<1)+1,
            e => return Err(ParseError::InvalidChar(e)),
        }
    }
    Ok(res)
}

fn reflection_idx(image: &[u32]) -> usize {
    (1..image.len()).find(|lower| (0..=lower-1).all(|row| 2*lower - row > image.len() || image[row] == image[2*lower-row-1])).unwrap_or(0)
}

fn smudged_reflection_idx(image: &[u32]) -> usize {
    (1..image.len()).find(|lower| (0..=lower-1).filter(|row| 2*lower-row<=image.len()).map(|row| (image[row] ^ image[2*lower-row-1]).count_ones()).sum::<u32>() == 1).unwrap_or(0)
}

fn transpose(matrix: &[u32]) -> Vec<u32> {
    let elems = (0..=31).rev().find(|n| matrix.iter().any(|x| x & 1<<n > 0)).unwrap_or(0);

    (0..=elems).rev()
        .map(|col| matrix.iter()
                        .cloned()
                        .fold(0, |acc, y| (acc<<1)+((y>>col) & 1)))
        .collect()
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
        assert_eq!(run(&sample_input), Ok((405, 400)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((32723, 34536)));
    }
}
