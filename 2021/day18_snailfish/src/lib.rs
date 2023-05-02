use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidToken(char),
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
            Self::InvalidToken(c) => write!(f, "Unexpected Character encountered: {c}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

#[derive(Clone, Debug)]
struct NumberPart {
	level: u8,
	value: usize,
}

#[derive(Clone)]
struct SnailNumber {
	parts: Vec<NumberPart>
}

impl TryFrom<&str> for SnailNumber {
	type Error = ParseError;

	fn try_from(input: &str) -> Result<Self, Self::Error> {
		let mut parts = Vec::new();
		let mut level = 0;
		let mut value = 0;
		let mut assembling_number = false;

		for c in input.chars() {
			match c {
				'[' => {
						if assembling_number {
							parts.push(NumberPart{ level, value });
							value = 0;
							assembling_number = false;
						}
						level += 1;
					},
				']' => {
						if assembling_number {
							parts.push(NumberPart{ level, value });
							value = 0;
							assembling_number = false;
						}
						level -= 1;
					},
				',' => {
						if assembling_number {
							parts.push(NumberPart{ level, value });
							assembling_number = false;
						}
						value = 0;
					},
				n if n.is_numeric() => {
						value = value*10 + n.to_digit(10).unwrap() as usize;
						assembling_number = true;
					},
				t => return Err(Self::Error::InvalidToken(t)),
			}
		}
		Ok(Self{ parts, })
	}
}

impl SnailNumber {
    fn add(&mut self, rhs: &mut Self) {
        self.parts.append(&mut rhs.parts);
        self.parts.iter_mut().for_each(|part| part.level += 1);
    }

	fn reduce(&mut self) {
        while self.explode() || self.split() {}
	}

	fn explode(&mut self) -> bool {
		if let Some(idx) = self.parts.iter().position(|p| p.level > 4) {
			if idx > 0 {
				self.parts[idx-1].value += self.parts[idx].value;
            }
			if idx < self.parts.len()-2 {
				self.parts[idx+2].value += self.parts[idx+1].value;
            }
            self.parts[idx].level -= 1;
            self.parts[idx].value = 0;
            self.parts.remove(idx+1);
			true
		} else {
			false
		}
	}

	fn split(&mut self) -> bool {
		if let Some(idx) = self.parts.iter().position(|p| p.value > 9) {
            let (old_level, old_value) = (self.parts[idx].level, self.parts[idx].value);
			self.parts[idx].value /= 2;
			self.parts[idx].level += 1;
			self.parts.insert(idx+1, NumberPart{ level: old_level+1, value: (old_value+1)/2 });
			true
		} else {
			false
		}
	}

	fn magnitude(&mut self) -> usize {
		if self.parts.len() == 1 {
			self.parts[0].value
		} else {
			let idx = self.parts.windows(2).position(|w| w[0].level == w[1].level).expect(&format!("Unable to reduce {:?}", self.parts)[..]);
			self.parts[idx].value = 3*self.parts[idx].value + 2*self.parts[idx+1].value;
			self.parts[idx].level = self.parts[idx].level.saturating_sub(1);
			self.parts.remove(idx+1);
			self.magnitude()
		}
	}
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut number = SnailNumber::try_from(input.lines().next().unwrap())?;
    for line in input.lines().skip(1) {
        number.add(&mut SnailNumber::try_from(line)?);
        number.reduce();
    }
    let first = number.magnitude();
    let numbers: Vec<_> = input.lines().map(SnailNumber::try_from).collect::<Result<Vec<_>, _>>()?;
    let mut second = 0;
    (0..numbers.len()).for_each(|a| (0..numbers.len()).for_each(|b| {
        if a != b {
            let mut lhs = numbers[a].clone();
            lhs.add(&mut numbers[b].clone());
            lhs.reduce();
            second = second.max(lhs.magnitude());
        }
    }));
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
        assert_eq!(run(&sample_input), Ok((4140, 3993)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((3987, 4500)));
    }
}
