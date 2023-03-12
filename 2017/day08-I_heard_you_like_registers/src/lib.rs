use std::collections::HashMap;

enum Operation { Inc, Dec }

impl Operation {
    fn from(s: &str) -> Self {
        match s {
            "inc" => Self::Inc,
            "dec" => Self::Dec,
            _ => panic!("Unexpected Operation: {s}"),
        }
    }
}

enum Comparison { LessThan, LessOrEqual, Equal, NotEqual, GreaterOrEqual, GreaterThan }

impl Comparison{
    fn from(s: &str) -> Self {
        match s {
            "<" => Self::LessThan,
            "<=" => Self::LessOrEqual,
            "==" => Self::Equal,
            "!=" => Self::NotEqual,
            ">=" => Self::GreaterOrEqual,
            ">" => Self::GreaterThan,
            _ => panic!("Unexpected Comparison: {s}"),
        }
    }
}

struct Condition {
    register: usize,
    comparator: Comparison,
    value: isize
}

impl Condition {
    fn is_true(&self, registers: &[isize]) -> bool {
        match self.comparator {
            Comparison::LessThan => registers[self.register] < self.value,
            Comparison::LessOrEqual => registers[self.register] <= self.value,
            Comparison::Equal => registers[self.register] == self.value,
            Comparison::NotEqual => registers[self.register] != self.value,
            Comparison::GreaterOrEqual => registers[self.register] >= self.value,
            Comparison::GreaterThan => registers[self.register] > self.value,
        }
    }
}

struct Instruction {
    register: usize,
    operation: Operation,
    operand: isize,
    condition: Condition,
}

struct Cpu {
    registers: Vec<isize>,
    program: Vec<Instruction>,
    max_val: isize,
}

impl Cpu {
    fn from(prog: &str) -> Self {
        let mut regs = HashMap::new();
        let mut program = Vec::new();

        prog.lines().for_each(|line| {
            let components: Vec<_> = line.split_whitespace().collect();
            assert_eq!(components.len(), 7);

            let op_register = components[0];
            let operation = Operation::from(components[1]);
            let operand = components[2].parse().unwrap();
            let comp_register = components[4];
            let comparator = Comparison::from(components[5]);
            let value = components[6].parse().unwrap();

            let next_reg = regs.len();
            let register = *regs.entry(comp_register).or_insert(next_reg);
            let condition = Condition {
                register,
                comparator,
                value,
            };

            let next_reg = regs.len();
            let register = *regs.entry(op_register).or_insert(next_reg);
            program.push(Instruction {
                register,
                operation,
                operand,
                condition,
            });
        });

        Self {
            registers: vec![0; regs.len()],
            program,
            max_val: 0,
        }
    }

    fn run(&mut self) {
        for instruction in &self.program {
            if instruction.condition.is_true(&self.registers) {
                let old_value = self.registers[instruction.register];
                let new_value = match instruction.operation {
                    Operation::Inc => old_value + instruction.operand,
                    Operation::Dec => old_value - instruction.operand,
                };
                self.registers[instruction.register] = new_value;
                self.max_val = self.max_val.max(new_value);
            }
        }
    }
}

pub fn run(input: &str) -> (isize, isize) {
    let mut cpu = Cpu::from(input);
    cpu.run();
    let first = *cpu.registers.iter().max().unwrap();
    let second = cpu.max_val;
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
        assert_eq!(run(&sample_input), (1, 10));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (6343, 7184));
    }
}
