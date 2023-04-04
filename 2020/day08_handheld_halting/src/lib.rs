use core::fmt::Display;
use std::{num::ParseIntError, collections::HashSet};

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

#[derive(Clone)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl TryFrom<&str> for Instruction {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<_> = value.split_whitespace().collect();
        if parts.len() == 2 {
            let argument = if parts[1].starts_with('+') {
                parts[1][1..].parse::<isize>()?
            } else {
                parts[1].parse::<isize>()?
            };
            match parts[0] {
                "acc" => Ok(Self::Acc(argument)),
                "jmp" => Ok(Self::Jmp(argument)),
                "nop" => Ok(Self::Nop(argument)),
                _ => Err(Self::Error::LineMalformed(value.to_string())),
            }
        } else {
            Err(Self::Error::LineMalformed(value.to_string()))
        }
    }
}

#[derive(Default, Clone)]
struct Cpu {
    program: Vec<Instruction>,
    next_instr: usize,
    accumulator: isize,
    visited: HashSet<usize>,
}

impl TryFrom<&str> for Cpu {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let program: Vec<_> = value.lines().map(Instruction::try_from).collect::<Result<Vec<_>, _>>()?;
        Ok(Self { program, ..Default::default() })
    }
}

impl Cpu {
    fn run_until_loop(&mut self) -> Result<isize, isize> {
        match self.program[self.next_instr] {
            Instruction::Acc(i) => {
                self.accumulator += i;
                self.next_instr += 1;
            },
            Instruction::Jmp(p) if p >= 0 => self.next_instr += p.unsigned_abs(),
            Instruction::Jmp(n) => self.next_instr -= n.unsigned_abs(),
            Instruction::Nop(_) => self.next_instr += 1,
        }
        if self.visited.contains(&self.next_instr) {
            Err(self.accumulator)
        } else if self.next_instr == self.program.len() {
            Ok(self.accumulator)
        } else {
            self.visited.insert(self.next_instr);
            self.run_until_loop()
        }
    }
}

pub fn run(input: &str) -> Result<(isize, isize), ParseError> {
    let cpu = Cpu::try_from(input)?;
    let mut cpu_1 = cpu.clone();
    let first = cpu_1.run_until_loop().err().unwrap();
    for i in 0..cpu.program.len() {
        let new = match cpu.program[i] {
            Instruction::Acc(_) => continue,
            Instruction::Jmp(i) => Instruction::Nop(i),
            Instruction::Nop(i) => Instruction::Jmp(i),
        };
        let mut cpu_2 = cpu.clone();
        cpu_2.program[i] = new;
        if let Ok(second) = cpu_2.run_until_loop() {
            return Ok((first, second));
        }
    }
    panic!("No way found to break the infinite loop");
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
        assert_eq!(run(&sample_input), Ok((5, 8)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((2080, 2477)));
    }
}
