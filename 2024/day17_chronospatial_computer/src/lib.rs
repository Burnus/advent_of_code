use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ParseIntError(std::num::ParseIntError),
    LineCount,
    ProgramNotSet,
    RegisterNotSet(u8),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineCount => write!(f, "Input must consist of 5 lines: One setting each register, an empty line, and the program"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
            Self::ProgramNotSet => write!(f, "Line 5 must contain the program at its last position (separated by whitespace)"),
            Self::RegisterNotSet(n) => write!(f, "Line {n} must contain the value for register {} at its last position (separated by whitespace)", b'@' + n),
        }
    }
}

#[derive(Default, Clone)]
struct Computer {
    opcode: Vec<u8>,
    instruction_ptr: usize,
    registers: [usize; 3],
    output: Vec<u8>,
}

impl TryFrom<&str> for Computer {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines: Vec<_> = value.lines().collect();
        if lines.len() != 5 {
            return Err(Self::Error::LineCount);
        }
        let a = lines[0].split_whitespace().last().ok_or(Self::Error::RegisterNotSet(1))?.parse::<usize>()?;
        let b = lines[1].split_whitespace().last().ok_or(Self::Error::RegisterNotSet(2))?.parse::<usize>()?;
        let c = lines[2].split_whitespace().last().ok_or(Self::Error::RegisterNotSet(3))?.parse::<usize>()?;

        let registers = [a, b, c];

        let opcode = lines[4]
            .split_whitespace()
            .last()
            .ok_or(Self::Error::ProgramNotSet)?
            .split(',')
            .map(|n| n.parse::<u8>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            opcode,
            registers,
            ..Default::default()
        })
    }
}

impl Computer {
    fn combo(&self, operand: usize) -> usize {
        if operand < 4 {
            operand
        } else {
            self.registers[operand - 4]
        }
    }

    fn adv(&mut self, operand: usize) {     
        self.registers[0] >>= self.combo(operand);
    }

    fn bxl(&mut self, operand: usize) {            
        self.registers[1] ^= operand
    }

    fn bst(&mut self, operand: usize) {     
        self.registers[1] = self.combo(operand) & 7;
    }
    
    fn jnz(&mut self, operand: usize) {
        self.instruction_ptr = if self.registers[0] == 0 {
            self.instruction_ptr + 2
        } else {
            operand
        };
    }
    
    fn bxc(&mut self) {
        self.registers[1] ^= self.registers[2];
    }
    
    fn out(&mut self, operand: usize) {
        self.output.push((self.combo(operand) & 7) as u8);
    }
    
    fn bdv(&mut self, operand: usize) {
        self.registers[1] = self.registers[0] >> self.combo(operand);
    }
    
    fn cdv(&mut self, operand: usize) {
        self.registers[2] = self.registers[0] >> self.combo(operand);
    }
    
    fn run(&mut self) {
        while self.instruction_ptr < self.opcode.len()-1 {
            let instruction = self.opcode[self.instruction_ptr];
            let operand = self.opcode[self.instruction_ptr+1] as usize;
            match instruction {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => unreachable!()
            }
            if instruction != 3 {
                self.instruction_ptr += 2;
            }
        }
    }
}

pub fn run(input: &str) -> Result<(String, usize), ParseError> {
    let computer = Computer::try_from(input)?;
    let mut computer_1 = computer.clone();
    computer_1.run();
    let first = computer_1.output.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");

    // For part 2, we take advantage of the fact that both inputs are 
    // structured in a way that makes every nth last output only dependent 
    // on the first n octal digits of A, excluding leading zeros.
    let mut possible_starts = Vec::from([0]);
    let mut next_possible_starts: Vec<usize>;
    for idx in (0..computer.opcode.len()).rev() {
        next_possible_starts = possible_starts
            .iter()
            .flat_map(|&start| (start << 3 .. (start + 1) << 3).filter(|&a| {
                let mut computer = computer.clone();
                computer.registers[0] = a;
                computer.run();
                computer.output == computer.opcode[idx..]
            })).collect();
        std::mem::swap(&mut possible_starts, &mut next_possible_starts);
    }
    let second = *possible_starts.iter().min().unwrap();
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
        assert_eq!(run(&sample_input), Ok(("5,7,3,0".to_string(), 117440)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok(("7,4,2,5,1,4,6,0,4".to_string(), 164278764924605)));
    }
}
