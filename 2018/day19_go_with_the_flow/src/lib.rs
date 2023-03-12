type Register = usize;
type Value = isize;

#[derive(Clone)]
enum Operation {
    AddR(Register, Register, Register),
    AddI(Register, Value, Register),
    MulR(Register, Register, Register),
    MulI(Register, Value, Register),
    BAnR(Register, Register, Register),
    BAnI(Register, Value, Register),
    BOrR(Register, Register, Register),
    BOrI(Register, Value, Register),
    SetR(Register, Value, Register),
    SetI(Value, Value, Register),
    GtIR(Value, Register, Register),
    GtRI(Register, Value, Register),
    GtRR(Register, Register, Register),
    EqIR(Value, Register, Register),
    EqRI(Register, Value, Register),
    EqRR(Register, Register, Register),
    Ip(Register),
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        let components: Vec<_> = value.split_whitespace().collect();
        match components[0] {
            "addr" => Self::AddR(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "addi" => Self::AddI(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "mulr" => Self::MulR(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "muli" => Self::MulI(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "banr" => Self::BAnR(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "bani" => Self::BAnI(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "borr" => Self::BOrR(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "bori" => Self::BOrI(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "setr" => Self::SetR(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "seti" => Self::SetI(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "gtir" => Self::GtIR(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "gtri" => Self::GtRI(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "gtrr" => Self::GtRR(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "eqir" => Self::EqIR(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "eqri" => Self::EqRI(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "eqrr" => Self::EqRR(components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap()),
            "#ip" => Self::Ip(components[1].parse().unwrap()),
            _ => panic!("Unexpected instruction: {components:?}"),
        }
    }
}

impl Operation {
    fn perform(&self, registers: &mut [Value; 6]) {
        match self {
            Self::AddR(a, b, c) => registers[*c] = registers[*a] + registers[*b],
            Self::AddI(a, b, c) => registers[*c] = registers[*a] + *b,
            Self::MulR(a, b, c) => registers[*c] = registers[*a] * registers[*b],
            Self::MulI(a, b, c) => registers[*c] = registers[*a] * *b,
            Self::BAnR(a, b, c) => registers[*c] = registers[*a] & registers[*b],
            Self::BAnI(a, b, c) => registers[*c] = registers[*a] & *b,
            Self::BOrR(a, b, c) => registers[*c] = registers[*a] | registers[*b],
            Self::BOrI(a, b, c) => registers[*c] = registers[*a] | *b,
            Self::SetR(a, _b, c) => registers[*c] = registers[*a],
            Self::SetI(a, _b, c) => registers[*c] = *a,
            Self::GtIR(a, b, c) => registers[*c] = if *a > registers[*b] { 1 } else { 0 },
            Self::GtRI(a, b, c) => registers[*c] = if registers[*a] > *b { 1 } else { 0 },
            Self::GtRR(a, b, c) => registers[*c] = if registers[*a] > registers[*b] { 1 } else { 0 },
            Self::EqIR(a, b, c) => registers[*c] = if *a == registers[*b] { 1 } else { 0 },
            Self::EqRI(a, b, c) => registers[*c] = if registers[*a] == *b { 1 } else { 0 },
            Self::EqRR(a, b, c) => registers[*c] = if registers[*a] == registers[*b] { 1 } else { 0 },
            Self::Ip(_) => panic!("Instruction Register reset"),
        }
    }
}

#[derive(Default)]
struct Cpu {
    registers: [isize; 6],
    instructions: Vec<Operation>,
    instr_reg: usize,
}

impl Cpu {
    fn with_instructions(instructions: &[Operation]) -> Self {
        if let Operation::Ip(instr_reg) = instructions[0] {
            Self {
                instr_reg,
                instructions: instructions[1..].to_vec(),
                ..Default::default()
            }
        } else {
            panic!("Instruction Register not set")
        }
    }

    fn run(&mut self) {
        loop {
            let instr_ptr = self.registers[self.instr_reg] as usize;
            if instr_ptr >= self.instructions.len() {
                self.registers[self.instr_reg] -= 1;
                return;
            }
            let intstruction = &self.instructions[instr_ptr];
            if instr_ptr > 7 {
                match (&self.instructions[instr_ptr-8], &self.instructions[instr_ptr-7], &self.instructions[instr_ptr-6], &self.instructions[instr_ptr-5], &self.instructions[instr_ptr-4], &self.instructions[instr_ptr-3], &self.instructions[instr_ptr-2], &self.instructions[instr_ptr-1], &self.instructions[instr_ptr]) {
                    (Operation::MulR(b1, d1, f1), Operation::EqRR(f2, c1, f3), Operation::AddR(f4, e1, e2), Operation::AddI(e3, 1, e4), Operation::AddR(b2, a1, a2), Operation::AddI(d2, 1, d3), Operation::GtRR(d4, c2, f5), Operation::AddR(e5, f6, e6), Operation::SetI(2, _, e7)) if a1 == a2 && b1 == b2 && c1 == c2 && d1 == d2 && d1 == d3 && d1 == d4 && e1 == e2 && e1 == e3 && e1 == e4 && e1 == e5 && e1 == e6 && e1 == e7 && e1 == &self.instr_reg && f1 == f2 && f1 == f3 && f1 == f4 && f1 == f5 && f1 == f6 => {
                        self.registers[*d1] = self.registers[*c1] + 1;
                        if self.registers[*c1] % self.registers[*b1] == 0 && self.registers[*c1] > self.registers[*b1] {
                            self.registers[*a1] += self.registers[*b1];
                        }
                        self.registers[*f1] = 1;
                        self.registers[self.instr_reg] = 12;
                    },
                    _ => {
                        intstruction.perform(&mut self.registers);
                        self.registers[self.instr_reg] += 1;
                    }
                    }
                } else {
                    intstruction.perform(&mut self.registers);
                    self.registers[self.instr_reg] += 1;
                }
            }
        }
}

pub fn run(input: &str) -> (isize, isize) {
    let instructions: Vec<_> = input.lines().map(Operation::from).collect();
    let mut cpu = Cpu::with_instructions(&instructions);
    cpu.run();
    let first = cpu.registers[0];
    let mut cpu = Cpu::with_instructions(&instructions);
    cpu.registers[0] = 1;
    cpu.run();
    let second = cpu.registers[0];
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
        assert_eq!(run(&sample_input), (6, 6));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (1350, 15844608));
    }
}
