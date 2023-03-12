#[derive(Debug)]
enum Instruction {
    Half(usize),
    Triple(usize),
    Increment(usize),
    Jump(isize),
    JumpIfEven(usize, isize),
    JumpIfOne(usize, isize),
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let components: Vec<_> = line.split(" ").collect();
        let register = components[1].bytes().next().unwrap().saturating_sub(b'a') as usize;
        let offset = if components.len() == 3 {
            components[2].parse::<isize>().unwrap()
        } else {
            components[1].parse::<isize>().unwrap_or(0)
        };
        match components[0] {
            "hlf" => Self::Half(register),
            "tpl" => Self::Triple(register),
            "inc" => Self::Increment(register),
            "jmp" => Self::Jump(offset),
            "jie" => Self::JumpIfEven(register, offset),
            "jio" => Self::JumpIfOne(register, offset),
            _ => panic!("Unexpected token in {line}"),
        }
    }
}

struct Cpu {
    instructions: Vec<Instruction>,
    registers: [usize; 2],
    current_instruction_index: usize,
}

impl Cpu {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            registers: [0, 0],
            current_instruction_index: 0,
        }
    }

    fn step(&mut self) -> bool {
        if self.current_instruction_index >= self.instructions.len() {
            return true;
        }
        let next_instruction = &self.instructions[self.current_instruction_index];
        self.current_instruction_index += 1;
        match next_instruction {
            Instruction::Half(r) => self.registers[*r] /= 2,
            Instruction::Triple(r) => self.registers[*r] *= 3,
            Instruction::Increment(r) => self.registers[*r] += 1,
            Instruction::Jump(offset) => self.current_instruction_index = (self.current_instruction_index as isize + *offset - 1) as usize,
            Instruction::JumpIfEven(r, offset) => { 
                if self.registers[*r] % 2 == 0 { 
                    self.current_instruction_index = (self.current_instruction_index as isize + *offset - 1) as usize; 
                } 
            },
            Instruction::JumpIfOne(r, offset) => { 
                if self.registers[*r] == 1 { 
                    self.current_instruction_index = (self.current_instruction_index as isize + *offset - 1) as usize; 
                } 
            },
        }
        false
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let mut cpu = Cpu::new();
    let instructions = input.lines().map(Instruction::parse).collect();
    cpu.instructions = instructions;
    loop {
        let finished = cpu.step();
        if finished {
            break;
        }
    }
    let first = cpu.registers[1];
    cpu.registers = [1, 0];
    cpu.current_instruction_index = 0;
    loop {
        let finished = cpu.step();
        if finished {
            break;
        }
    }
    let second = cpu.registers[1];
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), (2, 2));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (255, 334));
    }
}
