use std::str::FromStr;
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};

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
    Snd(Operand),
    Set(RegIdx, Operand),
    Add(RegIdx, Operand),
    Mul(RegIdx, Operand),
    Mod(RegIdx, Operand),
    Rcv(RegIdx),
    Jgz(Operand, Operand),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<_> = s.split_whitespace().collect();

        match components[0] {
            "snd" => Ok(Self::Snd(components[1].parse()?)),
            "set" => Ok(Self::Set((components[1].as_bytes()[0] - b'a') as RegIdx, components[2].parse()?)),
            "add" => Ok(Self::Add((components[1].as_bytes()[0] - b'a') as RegIdx, components[2].parse()?)), 
            "mul" => Ok(Self::Mul((components[1].as_bytes()[0] - b'a') as RegIdx, components[2].parse()?)), 
            "mod" => Ok(Self::Mod((components[1].as_bytes()[0] - b'a') as RegIdx, components[2].parse()?)), 
            "rcv" => Ok(Self::Rcv((components[1].as_bytes()[0] - b'a') as RegIdx)),
            "jgz" => Ok(Self::Jgz(components[1].parse()?, components[2].parse()?)),
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
    fn perform(&self, registers: &mut [RegVal], out: &mut RegVal, next_instr_ptr: &mut RegIdx, version: u8, tx: &mut Sender<RegVal>, rx: &mut Receiver<RegVal>) -> Option<RegVal> {
        *next_instr_ptr += 1;
        let mut recovered = None;
        match self {
            Self::Snd(Operand::Reg(reg_idx)) => if version == 1 {
                                        *out = registers[*reg_idx];
                                    } else if tx.send(registers[*reg_idx]).is_ok() {
                                        *out += 1;
                                    } else {
                                        return Some(*out);
                                    },
            Self::Snd(Operand::Val(val)) => if version == 1 {
                                        *out = *val;
                                    } else if tx.send(*val).is_ok() {
                                        *out += 1;
                                    } else {
                                        return Some(*out);
                                    },
            Self::Set(to, Operand::Reg(from)) => registers[*to] = registers[*from],
            Self::Set(to, Operand::Val(val)) => registers[*to] = *val,
            Self::Add(to, Operand::Reg(from)) => registers[*to] += registers[*from],
            Self::Add(to, Operand::Val(val)) => registers[*to] += *val,
            Self::Mul(to, Operand::Reg(from)) => registers[*to] *= registers[*from],
            Self::Mul(to, Operand::Val(val)) => registers[*to] *= *val,
            Self::Mod(to, Operand::Reg(from)) => registers[*to] %= registers[*from],
            Self::Mod(to, Operand::Val(val)) => registers[*to] %= *val,
            Self::Rcv(reg_idx) => if version == 1 {
                                        if registers[*reg_idx] != 0 { recovered = Some(*out) }
                                    } else if let Ok(val) = rx.recv_timeout(std::time::Duration::from_millis(50)) {
                                        registers[*reg_idx] = val;
                                    } else {
                                        return Some(*out);
                                    },
            Self::Jgz(Operand::Reg(reg_idx), Operand::Val(offset)) => if registers[*reg_idx] > 0 { *next_instr_ptr = (*next_instr_ptr as isize + offset - 1) as usize },
            Self::Jgz(Operand::Val(val), Operand::Val(offset)) => if *val > 0 { *next_instr_ptr = (*next_instr_ptr as isize + offset - 1) as usize },
            Self::Jgz(Operand::Reg(reg_idx), Operand::Reg(offset_reg)) => if registers[*reg_idx] > 0 { *next_instr_ptr = (*next_instr_ptr as isize + registers[*offset_reg] - 1) as usize },
            Self::Jgz(Operand::Val(val), Operand::Reg(offset_reg)) => if *val > 0 { *next_instr_ptr = (*next_instr_ptr as isize + registers[*offset_reg] - 1) as usize },
        }
        recovered
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
    fn run(&mut self, version: u8, mut tx: Sender<RegVal>, mut rx: Receiver<RegVal>) -> RegVal {
        loop {
            if self.next_instr_ptr >= self.instructions.len() {
                return self.out;
            }
            let next_instruction = &self.instructions[self.next_instr_ptr];
            if let Some(freq) = next_instruction.perform(&mut self.registers, &mut self.out, &mut self.next_instr_ptr, version, &mut tx, &mut rx) {
                return freq;
            }
        }
    }
}

pub fn run(input: &str) -> (isize, isize) {
    let mut cpu = Cpu {
        instructions: input.lines().map(Instruction::from).collect(),
        ..Default::default()
    };
    let mut cpu_0 = cpu.clone();
    let mut cpu_1 = cpu.clone();
    cpu_1.registers[15] = 1;
    let (tx, rx) = channel();
    let (tx_1, rx_0) = channel();
    let (tx_0, rx_1) = channel();
    let first = thread::spawn(move || cpu.run(1, tx, rx)).join().unwrap();
    thread::spawn(move || cpu_0.run(2, tx_0, rx_0));
    let second = thread::spawn(move || cpu_1.run(2, tx_1, rx_1)).join().unwrap();
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
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), (4, 1));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (1187, 5969));
    }
}
