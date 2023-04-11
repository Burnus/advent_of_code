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

struct Requirement {
    is_departure: bool,
    left_range: (usize, usize),
    right_range: (usize, usize),
}

impl TryFrom<&str> for Requirement {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let components: Vec<_> = value.split(&[' ', '-']).collect();
        if !(6..=7).contains(&components.len()) {
            return Err(Self::Error::LineMalformed(value.to_string()));
        }

        Ok(Self { 
            is_departure: components[0] == "departure",
            left_range: (components[components.len()-5].parse()?, components[components.len()-4].parse()?), 
            right_range: (components[components.len()-2].parse()?, components[components.len()-1].parse()?), 
        })
    }
}

impl Requirement {
    fn is_valid(&self, value: usize) -> bool {
        (self.left_range.0..=self.left_range.1).contains(&value) || (self.right_range.0..=self.right_range.1).contains(&value)
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let parts: Vec<_> = input.split("\n\n").collect();
    let mut requirements: Vec<_> = parts[0].lines().map(Requirement::try_from).collect::<Result<Vec<_>, _>>()?;
    let mine: Vec<_> = parts[1].lines().nth(1).unwrap().split(',').map(|n| n.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
    let nearby: Vec<_> = parts[2].lines().skip(1).map(|line| line.split(',').map(|n| n.parse::<usize>()).collect::<Result<Vec<_>, _>>()).collect::<Result<Vec<_>, _>>()?;
    let first = nearby.iter().map(|ticket| ticket.iter().filter(|value| !requirements.iter().any(|r| r.is_valid(**value))).sum::<usize>()).sum();
    let valid: Vec<_> = nearby.iter().filter(|ticket| ticket.iter().all(|value| requirements.iter().any(|r| r.is_valid(*value)))).collect();
    let mut undecided_fields: Vec<_> = (0..requirements.len()).collect();
    let mut departure_fields = Vec::new();
    while requirements.iter().any(|req| req.is_departure) {
        let mut possible = Vec::new();
        undecided_fields.iter().for_each(|field_idx| {
            possible.push((*field_idx, requirements.iter().enumerate().filter(|(_req_idx, req)| req.is_valid(mine[*field_idx]) && valid.iter().all(|ticket| req.is_valid(ticket[*field_idx]))).map(|(req_idx, _req)| req_idx).collect::<Vec<_>>()));
        });
        let pos_idx = if let Some(pos) = possible.iter().position(|(_field_idx, reqs)| reqs.len() == 1) {
            pos
        } else if let Some(req_id) = (0..requirements.len()).find(|req_idx| possible.iter().filter(|(_field_idx, reqs)| reqs.contains(&req_idx)).count() == 1) {
            possible.iter().position(|(_field_id, reqs)| reqs.contains(&req_id)).unwrap()
        } else {
            panic!("Unable to discard any possibilities");
        };

        let field_idx = possible[pos_idx].0;
        let req_idx = possible[pos_idx].1[0];
        if requirements[req_idx].is_departure {
            departure_fields.push(field_idx);
        }
        undecided_fields.remove(undecided_fields.binary_search(&field_idx).unwrap());
        requirements.remove(req_idx);
    }
    let second = mine.iter().enumerate().filter(|(idx, _val)| departure_fields.contains(&&idx)).map(|(_idx, val)| val).product();
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
        assert_eq!(run(&sample_input), Ok((71, 132)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((23954, 453459307723)));
    }
}
