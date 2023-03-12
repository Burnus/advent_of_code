use std::str::FromStr;
// use std::thread;
// use std::sync::mpsc::{channel, Receiver, Sender};

type RegIdx = usize;
type RegVal = isize;

#[derive(Debug)]
struct ParseError;

#[derive(Debug, Clone)]
enum Operand {
    Reg(RegIdx),
    Val(RegVal),
}

impl FromStr for Operand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = s.parse::<RegVal>() {
            Ok(Self::Val(val))
        } else {
            match s.as_bytes()[0] {
                c @ b'a'..=b'z' => Ok(Self::Reg((c - b'a') as RegIdx)),
                _ => Err(ParseError),
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    // Snd(Operand),
    Set(RegIdx, Operand),
    // Add(RegIdx, Operand),
    Sub(RegIdx, Operand),
    Mul(RegIdx, Operand),
    // Mod(RegIdx, Operand),
    // Rcv(RegIdx),
    Jnz(Operand, Operand),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<_> = s.split_whitespace().collect();

        match components[0] {
            "set" => Ok(Self::Set((components[1].as_bytes()[0] - b'a') as RegIdx, components[2].parse()?)),
            "sub" => Ok(Self::Sub((components[1].as_bytes()[0] - b'a') as RegIdx, components[2].parse()?)), 
            "mul" => Ok(Self::Mul((components[1].as_bytes()[0] - b'a') as RegIdx, components[2].parse()?)), 
            "jnz" => Ok(Self::Jnz(components[1].parse()?, components[2].parse()?)),
            _ => panic!("Instruction not recognized: {s}"),
        }
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        value.parse().expect("Unable to parse {value} into an instruction")
    }
}

impl Instruction {
    fn perform(&self, registers: &mut [RegVal], out: &mut RegVal, next_instr_ptr: &mut RegIdx) {
        *next_instr_ptr += 1;
        match self {
            Self::Set(to, Operand::Reg(from)) => registers[*to] = registers[*from],
            Self::Set(to, Operand::Val(val)) => registers[*to] = *val,
            Self::Sub(to, Operand::Reg(from)) => registers[*to] -= registers[*from],
            Self::Sub(to, Operand::Val(val)) => registers[*to] -= *val,
            Self::Mul(to, Operand::Reg(from)) => { 
                    registers[*to] *= registers[*from];
                    *out += 1;
                }
            Self::Mul(to, Operand::Val(val)) => {
                    registers[*to] *= *val;
                    *out += 1;
                }
            Self::Jnz(Operand::Reg(reg_idx), Operand::Val(offset)) => self.jump_not_zero(registers[*reg_idx], *offset, next_instr_ptr),
            Self::Jnz(Operand::Val(val), Operand::Val(offset)) => self.jump_not_zero(*val, *offset, next_instr_ptr),
            Self::Jnz(Operand::Reg(reg_idx), Operand::Reg(offset_reg)) => self.jump_not_zero(registers[*reg_idx], registers[*offset_reg], next_instr_ptr),
            Self::Jnz(Operand::Val(val), Operand::Reg(offset_reg)) => self.jump_not_zero(*val, registers[*offset_reg], next_instr_ptr),
        }
    }

    fn jump_not_zero(&self, compared: RegVal, offset: isize, next_instr_ptr: &mut RegIdx) {
        if compared != 0 {
            *next_instr_ptr = (*next_instr_ptr as isize + offset - 1) as usize;
        }
    }
}

#[derive(Clone, Default)]
struct Cpu {
    instructions: Vec<Instruction>,
    registers: [RegVal; 26],
    next_instr_ptr: usize,
    out: RegVal,
}

impl Cpu {
    /// This only works for this particular AoC challenge input. The only allowed modifications are the numbers range set in instructions (0..=7)!
    fn run_optimized(&mut self) {
        while self.next_instr_ptr <= 8 {
            let next_instruction = &self.instructions[self.next_instr_ptr];
            next_instruction.perform(&mut self.registers, &mut self.out, &mut self.next_instr_ptr);
        }
        self.registers[7] = (self.registers[1]..=self.registers[2]).step_by(17).filter(|i| !Self::is_prime(*i)).count() as isize;
    }

    fn is_prime(n: isize) -> bool {
        !(2..n).any(|a| n % a == 0)
    }

    fn run(&mut self) -> RegVal {
        loop {
            if self.next_instr_ptr >= self.instructions.len() {
                return self.out;
            }
            match &self.instructions[self.next_instr_ptr] {
                Instruction::Jnz(Operand::Reg(g), Operand::Val(-8)) => {
                if self.registers[*g] == 0 {
                    self.next_instr_ptr += 1;
                } else {
                    match ( &self.instructions[self.next_instr_ptr - 8], &self.instructions[self.next_instr_ptr - 7], &self.instructions[self.next_instr_ptr - 6], &self.instructions[self.next_instr_ptr - 5], &self.instructions[self.next_instr_ptr - 4], &self.instructions[self.next_instr_ptr - 3], &self.instructions[self.next_instr_ptr - 2], &self.instructions[self.next_instr_ptr - 1] ) {
                        ( Instruction::Set(g1, Operand::Reg(d)),
                          Instruction::Mul(g2, Operand::Reg(e)),
                          Instruction::Sub(g3, Operand::Reg(b)),
                          Instruction::Jnz(Operand::Reg(g4), Operand::Val(2)),
                          Instruction::Set(f, Operand::Val(0)),
                          Instruction::Sub(e1, Operand::Val(-1)),
                          Instruction::Set(g5, Operand::Reg(e2)),
                          Instruction::Sub(g6, Operand::Reg(b1))
                          ) if g == g1 && g == g2 && g == g3 && g == g4 && g == g5 && g == g6 && b == b1 && e == e1 && e == e2 => {
                                    let new_e = self.registers[*b];
                                    if new_e % self.registers[*d] == 0 {
                                        self.registers[*f] = 0;
                                    }
                                    self.out += new_e - self.registers[*e];
                                    self.registers[*e] = new_e;
                                    self.registers[*g] = 0;
                                    self.next_instr_ptr += 1;
                                }
                        _ => self.next_instr_ptr -= 8,
                    }
                }
                
            }, 
            next_instruction => next_instruction.perform(&mut self.registers, &mut self.out, &mut self.next_instr_ptr),
            }
        }
    }
}

pub fn run(input: &str) -> (isize, isize) {
    let mut cpu = Cpu {
        instructions: input.lines().map(Instruction::from).collect(),
        ..Default::default()
    };
    let mut cpu_1 = cpu.clone();
    cpu_1.registers[0] = 1;
    cpu.run();
    let first = cpu.out;
    // cpu_1.run();
    cpu_1.run_optimized();
    let second = cpu_1.registers[7];
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (9409, 913));
    }
}

