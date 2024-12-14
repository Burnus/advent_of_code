use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    ParseIntError(std::num::ParseIntError),
    InputMalformed(&'a str),
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
            Self::InputMalformed(v) => write!(f, "Machine is malformed: {v}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

type Coordinates = (usize, usize);

struct Machine {
    btn_a: Coordinates,
    btn_b: Coordinates,
    prize: Coordinates,
}

impl<'a> TryFrom<&'a str> for Machine {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let lines: Vec<_> = value.lines().collect();
        if lines.len() != 3 {
            return Err(Self::Error::InputMalformed(value));
        }
        let a: Vec<_> = lines[0].split(&['+', ',']).collect();
        if a.len() != 4 {
            return Err(Self::Error::LineMalformed(lines[0]));
        }
        let btn_a = (a[1].parse()?, a[3].parse()?);

        let b: Vec<_> = lines[1].split(&['+', ',']).collect();
        if b.len() != 4 {
            return Err(Self::Error::LineMalformed(lines[1]));
        }
        let btn_b = (b[1].parse()?, b[3].parse()?);

        let p: Vec<_> = lines[2].split(&['=', ',']).collect();
        if p.len() != 4 {
            return Err(Self::Error::LineMalformed(lines[2]));
        }
        let prize = (p[1].parse()?, p[3].parse()?);

        Ok(Self {
            btn_a,
            btn_b,
            prize,
        })
    }
}

impl Machine {
    fn prize_cost(&self) -> Option<usize> {
        // Determine the winning combination using matrix inversion. This method delivers the only
        // combination that will work (provided the button vectors are lenearly independent), but
        // the result may be non-integer, in which case there is no valid solution.
        let inverse_determinant = 1.0 / ((self.btn_a.0*self.btn_b.1) as f64 - (self.btn_a.1*self.btn_b.0) as f64);
        let a_presses = (inverse_determinant * (
            (self.btn_b.1 * self.prize.0) as f64 - (self.btn_b.0 * self.prize.1) as f64)
        ).round() as isize;
        let b_presses = (inverse_determinant * (
            (self.btn_a.0 * self.prize.1) as f64 - (self.btn_a.1 * self.prize.0) as f64)
        ).round() as isize;

        // Try all neighbouring combinations to account for rounding errors
        for delta_a in (-1..=1).filter(|da| a_presses + da >= 0) {
            let a = (a_presses + delta_a) as usize;
            for delta_b in (-1..=1).filter(|db| b_presses + db >= 0) {
                let b = (b_presses + delta_b) as usize;
                    if a * self.btn_a.0 + b * self.btn_b.0 == self.prize.0 && 
                        a * self.btn_a.1 + b * self.btn_b.1 == self.prize.1 {
                            return Some(3 * a + b)
                }
            }
        }
        None
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut machines: Vec<_> = input.split("\n\n").map(Machine::try_from).collect::<Result<Vec<_>, _>>()?;
    let first = machines.iter().filter_map(|m| m.prize_cost()).sum();
    machines.iter_mut().for_each(|m| {
        m.prize.0 += 10000000000000;
        m.prize.1 += 10000000000000;
    });
    let second = machines.iter().filter_map(|m| m.prize_cost()).sum();
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
        // The result for part 2 is not given in the challenge, but is what my solution finds.
        // Thus, it may be incorrect (although I doubt it since the other 3 results are correct).
        assert_eq!(run(&sample_input), Ok((480, 875318608908)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((29598, 93217456941970)));
    }
}
