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
    let mut lines = input.lines();
    let earliest_arrival = lines.next().expect("Input was empty").parse::<usize>()?;
    let busses: Vec<_> = lines.next().expect("Input only had 1 line instead of 2").split(',').enumerate().filter(|(_idx, bus)| bus != &"x").map(|(idx, bus)| match bus.parse::<usize>() { 
        Ok(id) => Ok((idx, id)),
        Err(e) => Err(e) } )
        .collect::<Result<Vec<_>, _>>()?;
    let first = if busses.iter().any(|(_idx, id)| earliest_arrival % id == 0) {
        0
    } else {
        busses.iter().min_by_key(|(_idx, id)| *id-(earliest_arrival % *id)).map(|(_idx, id)| *id * (*id-(earliest_arrival % *id))).unwrap()
    };
    let mut period = 1;
    let mut offset = 0;
    for (bus_offset, bus_id) in busses {
        for factor in 0.. {
            let this_offset = period*factor+offset;
            if (this_offset + bus_offset) % bus_id == 0 {
                period = lcm(period, bus_id);
                offset = this_offset;
                break;
            }
        }
    }
    Ok((first, offset))
}

fn lcm(lhs: usize, rhs: usize) -> usize {
    if rhs < 2 {
        lhs
    } else if lhs < 2 {
        rhs
    } else {
        let mut factor = rhs;
        while lhs % factor > 0 || rhs % factor > 0 {
            factor -= 1;
        }
        if factor < 2 {
            lhs * rhs
        } else {
            factor * lcm(lhs/factor, rhs/factor)
        }
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
        assert_eq!(run(&sample_input), Ok((295, 1068781)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((2406, 225850756401039)));
    }
}
