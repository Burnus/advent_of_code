use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    ParseIntError(std::num::ParseIntError),
    UnknownInstruction(&'a str),
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
            Self::UnknownInstruction(i) => write!(f, "Tried to parse unknown instruction {i}"),
        }
    }
}

struct Cpu {
    states: Vec<i32>,
}

impl <'a> TryFrom<&'a str> for Cpu {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut cpu = Cpu { states: vec![1, 1] };
        for instruction in value.lines() {
            match &instruction[0..4] {
                "noop" => cpu.noop(),
                "addx" => cpu.addx(instruction[5..].parse()?),
                instr => return Err(Self::Error::UnknownInstruction(instr)),
            }
        }
        Ok(cpu)
    }
}

impl Cpu {
    fn addx(&mut self, x: i32) {
        let old_state = *self.states.last().unwrap();
        self.states.push(old_state);
        self.states.push(old_state + x);
    }

    fn noop(&mut self) {
        let old_state = *self.states.last().unwrap();
        self.states.push(old_state);
    }

    fn get_rendering(&self) -> String {
        let mut rendering = String::new();
        (0..self.states.len()/40).for_each(|line_number| {
            if line_number != 0 {
                rendering.push('\n');
            }
            rendering.extend(
                (0..40).map(|col_number| {
                    let clock_cycle = 40*line_number+col_number;
                    if (clock_cycle as i32 % 40 - self.states[clock_cycle + 1]).abs() < 2 {
                        '#'
                    } else {
                        '.'
                    }
                })
            );
        });
        rendering
    }
}

pub fn run(input: &str) -> Result<(i32, String), ParseError> {
    let cpu = Cpu::try_from(input)?;
    let first = [20, 60, 100, 140, 180, 220].iter()
        .map(|&i| i as i32 * cpu.states[i])
        .sum();
    let second = cpu.get_rendering();
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
        let expected = &r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#[1..];
        assert_eq!(run(&sample_input), Ok((13140, expected.to_string())));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        let expected = &r#"
####.####.###..###..###..####.####.####.
#.......#.#..#.#..#.#..#.#.......#.#....
###....#..###..#..#.###..###....#..###..
#.....#...#..#.###..#..#.#.....#...#....
#....#....#..#.#....#..#.#....#....#....
#....####.###..#....###..#....####.#...."#[1..];
        assert_eq!(run(&challenge_input), Ok((14720, expected.to_string())));
    }
}
