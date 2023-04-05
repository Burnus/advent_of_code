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

pub fn run(input: &str) -> Result<(usize, usize), ParseIntError> {
    let mut adapters: Vec<_> = input.lines().map(|n| n.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
    adapters.push(0);
    adapters.sort();
    let first = get_multiplied_differences(&adapters);
    let groups = split_by_threes(&adapters);
    let second = groups.iter().map(|group| count_combinations(group)).product();
    Ok((first, second))
}

fn get_multiplied_differences(list: &[usize]) -> usize {
    let mut ones = 0;
    let mut threes = 1;
    list.windows(2).for_each(|w| {
        match w[1]-w[0] {
            1 => ones += 1,
            3 => threes += 1,
            0 | 2 => (),
            _ => panic!("Invalid chain! Difference between adapters of ratings {} and {} is larger than 3.", w[0], w[1]),
        }
    });
    ones * threes
}

fn split_by_threes(list: &[usize]) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    let mut last_partition = 0;

    list.windows(2).enumerate().for_each(|(lower_idx, w)| {
        if w[1]-w[0] == 3 {
            res.push(list[last_partition..=lower_idx].to_vec());
            last_partition = lower_idx+1;
        }
    });
    res.push(list[last_partition..].to_vec());
    res
}

fn count_combinations(list: &[usize]) -> usize {
    match list.len() {
        t if t < 3 => 1,
        3 => if list[2]-list[0] <= 3 { 2 } else { 1 },
        _ => {
            if list[3]-list[0] <= 3 {
                count_combinations(&list[1..]) + count_combinations(&list[2..]) + count_combinations(&list[3..])
            } else if list[2]-list[0] <= 3 {
                count_combinations(&list[1..]) + count_combinations(&list[2..])
            } else {
                count_combinations(&list[1..])
            }
        },
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
        assert_eq!(run(&sample_input), Ok((220, 19208)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((1856, 2314037239808)));
    }
}
