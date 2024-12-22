use core::fmt::Display;
use std::{collections::HashMap, num::ParseIntError};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidChar(char),
    ParseIntError(ParseIntError)
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChar(e) => write!(f, "Unable to parse into a keycode: {e}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse keycode into numeric part: {e}"),
        }
    }
}

type Coordinates = (i8, i8);

struct KeyCode {
    code: Vec<Coordinates>,
    numeric: usize,
}

impl TryFrom<&str> for KeyCode {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let code = value.chars().map(|c| {
            match c {
                u if (b'7'..=b'9').contains(&(u as u8)) => Ok((u as i8 - b'7' as i8, 0)),
                m if (b'4'..=b'6').contains(&(m as u8)) => Ok((m as i8 - b'4' as i8, 1)),
                l if (b'1'..=b'3').contains(&(l as u8)) => Ok((l as i8 - b'1' as i8, 2)),
                '0' => Ok((1, 3)),
                'A' => Ok((2, 3)),
                e => Err(Self::Error::InvalidChar(e))
            }
        }).collect::<Result<Vec<_>, _>>()?;
        let numeric = value[..value.len()-1].parse()?;
        Ok(Self { code, numeric, })
    }
}

impl KeyCode {
    fn buttons_to_move(arms: &mut [Arm], to: Coordinates, intermediate_robots: usize, mem: &mut HashMap<(Coordinates, Coordinates, usize), usize>) -> usize {
        if let Some(res) = mem.get(&(arms[intermediate_robots].pointing, to, intermediate_robots)) {
            arms[intermediate_robots].pointing = to;
            return *res;
        }
        let res = if intermediate_robots == 0 {
            arms[0].ways(to).into_iter().map(|w| w.len()).min().unwrap()
        } else {
            arms[intermediate_robots].ways(to).into_iter()
                .map(|w| {
                    w.into_iter().map(|btn| {
                        Self::buttons_to_move(arms, btn, intermediate_robots-1, mem)
                    }).sum()
                }).min().unwrap()
        };
        mem.insert((arms[intermediate_robots].pointing, to, intermediate_robots), res);
        arms[intermediate_robots].pointing = to;
        res
    }

    fn enter(&self, arms: &mut [Arm]) -> usize {
        let mut buttons_pushed = 0;
        let mut mem = HashMap::new();
        for digit in self.code.clone() {
            buttons_pushed += Self::buttons_to_move(arms, digit, arms.len()-1, &mut mem);
        }
        buttons_pushed
    }
}

#[derive(Clone, Copy)]
struct Arm {
    pointing: Coordinates,
    numeric: bool,
}

impl Arm {
    fn default() -> Self {
        Self {
            pointing: (2, 0),
            numeric: false,
        }
    }

    fn ways(&self, to: Coordinates) -> Vec<Vec<Coordinates>> {
        const ACTIVATE: Coordinates = (2, 0);
        const UP: Coordinates = (1, 0);
        const DOWN: Coordinates = (1, 1);
        const LEFT: Coordinates = (0, 1);
        const RIGHT: Coordinates = (2, 1);
        let (dx, dy) = (to.0 - self.pointing.0, to.1 - self.pointing.1);
        match (dx.signum(), dy.signum()) {
            (0, 0) => vec![vec![ACTIVATE]],
            (1, 0) => vec![(0..dx).map(|_| RIGHT).chain([ACTIVATE]).collect()],
            (-1, 0) => vec![(0..-dx).map(|_| LEFT).chain([ACTIVATE]).collect()],
            (0, 1) => vec![(0..dy).map(|_| DOWN).chain([ACTIVATE]).collect()],
            (0, -1) => vec![(0..-dy).map(|_| UP).chain([ACTIVATE]).collect()],
            (1, 1) => {
                let mut res = vec![
                    (0..dx).map(|_| RIGHT)
                        .chain((0..dy).map(|_| DOWN))
                        .chain([ACTIVATE]).collect()
                ];
                if !self.numeric || self.pointing.0 > 0 || to.1 < 3 {
                    res.push(
                    (0..dy).map(|_| DOWN)
                        .chain((0..dx).map(|_| RIGHT))
                        .chain([ACTIVATE]).collect()
                    );
                }
                res
            },
            (-1, 1) => {
                let mut res = vec![
                    (0..dy).map(|_| DOWN)
                        .chain((0..-dx).map(|_| LEFT))
                        .chain([ACTIVATE]).collect()
                ];
                if self.numeric || to.0 > 0 {
                    res.push(
                    (0..-dx).map(|_| LEFT)
                        .chain((0..dy).map(|_| DOWN))
                        .chain([ACTIVATE]).collect()
                    );
                }
                res
            },
            (1, -1) => {
                let mut res = vec![
                    (0..dx).map(|_| RIGHT)
                        .chain((0..-dy).map(|_| UP))
                        .chain([ACTIVATE].into_iter()).collect()
                ];
                if self.numeric || self.pointing.0 > 0 {
                    res.push(
                    (0..-dy).map(|_| UP)
                        .chain((0..dx).map(|_| RIGHT))
                        .chain([ACTIVATE]).collect()
                    );
                }
                res
            },
            (-1, -1) => {
                let mut res = vec![
                    (0..-dy).map(|_| UP)
                        .chain((0..-dx).map(|_| LEFT))
                        .chain([ACTIVATE]).collect()
                ];
                if !self.numeric || to.0 > 0 || self.pointing.1 < 3 {
                    res.push(
                    (0..-dx).map(|_| LEFT)
                        .chain((0..-dy).map(|_| UP))
                        .chain([ACTIVATE]).collect()
                    );
                }
                res
            },
            _ => unreachable!(),
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let codes: Vec<_> = input.lines().map(KeyCode::try_from).collect::<Result<Vec<_>, _>>()?;
    let mut arms = [
        Arm::default(),
        Arm::default(),
        Arm { pointing: (2, 3), numeric: true, },
    ];
    let first = codes.iter().map(|c| c.enter(&mut arms) * c.numeric).sum();
    let mut arms = [Arm::default(); 26];
    arms[25] = Arm { pointing: (2, 3), numeric: true, };
    let second = codes.iter().map(|c| c.enter(&mut arms) * c.numeric).sum();
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
        assert_eq!(run(&sample_input), Ok((126384, 154115708116294)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((123096, 154517692795352)));
    }
}
