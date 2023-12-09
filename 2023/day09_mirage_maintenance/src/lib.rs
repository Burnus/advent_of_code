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

pub fn run(input: &str) -> Result<(isize, isize), ParseError> {
    let datasets: Vec<_> = input.lines().map(|line| line.split_whitespace().map(|d| d.parse()).collect::<Result<Vec<isize>, ParseIntError>>()).collect::<Result<Vec<_>, _>>()?;
    let predictions: Vec<_> = datasets.iter().map(|d| predict(d)).collect();
    let first = predictions.iter().map(|(next, _prev)| next).sum();
    let second = predictions.iter().map(|(_next, prev)| prev).sum();
    Ok((first, second))
}

fn predict(dataset: &[isize]) -> (isize, isize) {
    let mut diffs = vec![dataset.to_vec()];
    loop {
        let curr = diffs.last().unwrap();
        if curr.iter().all(|v| v == &0) {
            break;
        }
        // reserve space for one more element (the series is 1 shorter now, but we'll end 2 for the
        // predictions in the end).
        let mut next = Vec::with_capacity(curr.len()+1);
        curr.windows(2).for_each(|pair| next.push(pair[1]-pair[0]));
        diffs.push(next);
    }
    (0..diffs.len()-1).rev().for_each(|idx| {
        let &&last = &diffs[idx].last().unwrap_or(&0);
        let &&diff = &diffs[idx+1].last().unwrap_or(&0);
        diffs[idx].push(last+diff);
    });
    let next = *diffs[0].last().unwrap();

    (0..diffs.len()-1).rev().for_each(|idx| {
        let &&last = &diffs[idx].first().unwrap_or(&0);
        let &&diff = &diffs[idx+1].last().unwrap_or(&0);
        diffs[idx].push(last-diff);
    });
    let prev = *diffs[0].last().unwrap();

    (next, prev)
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
        assert_eq!(run(&sample_input), Ok((114, 2)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((1861775706, 1082)));
    }
}
