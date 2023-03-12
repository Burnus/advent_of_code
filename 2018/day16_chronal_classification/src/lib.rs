use std::collections::{HashMap, HashSet};

type Register = usize;
type Value = isize;

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
}

impl Operation {
    fn perform(&self, registers: &mut [Value; 4]) {
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
        }
    }

    fn try_parse(op_nr: usize, opcode: &[&str]) -> Option<Self> {
        assert_eq!(opcode.len(), 3);
        let a_reg = opcode[0].parse::<usize>();
        let a_val = opcode[0].parse::<isize>();
        let b_reg = opcode[1].parse::<usize>();
        let b_val = opcode[1].parse::<isize>();
        let c_reg = opcode[2].parse::<usize>();
        match (op_nr, a_reg, a_val, b_reg, b_val, c_reg) {
            (0, Ok(a), _, Ok(b), _, Ok(c)) if a<4 && b<4 && c<4 => Some(Self::AddR(a, b, c)),
            (1, Ok(a), _, _, Ok(b), Ok(c)) if a<4 && c<4 => Some(Self::AddI(a, b, c)),
            (2, Ok(a), _, Ok(b), _, Ok(c)) if a<4 && b<4 && c<4 => Some(Self::MulR(a, b, c)),
            (3, Ok(a), _, _, Ok(b), Ok(c)) if a<4 && c<4 => Some(Self::MulI(a, b, c)),
            (4, Ok(a), _, Ok(b), _, Ok(c)) if a<4 && b<4 && c<4 => Some(Self::BAnR(a, b, c)),
            (5, Ok(a), _, _, Ok(b), Ok(c)) if a<4 && c<4 => Some(Self::BAnI(a, b, c)),
            (6, Ok(a), _, Ok(b), _, Ok(c)) if a<4 && b<4 && c<4 => Some(Self::BOrR(a, b, c)),
            (7, Ok(a), _, _, Ok(b), Ok(c)) if a<4 && c<4 => Some(Self::BOrI(a, b, c)),
            (8, Ok(a), _, _, _, Ok(c)) if a<4 && c<4 => Some(Self::SetR(a, 0, c)),
            (9, _, Ok(a), _, _, Ok(c)) if c<4 => Some(Self::SetI(a, 0, c)),
            (10, _, Ok(a), Ok(b), _, Ok(c)) if b<4 && c<4 => Some(Self::GtIR(a, b, c)),
            (11, Ok(a), _, _, Ok(b), Ok(c)) if a<4 && c<4 => Some(Self::GtRI(a, b, c)),
            (12, Ok(a), _, Ok(b), _, Ok(c)) if a<4 && b<4 && c<4 => Some(Self::GtRR(a, b, c)),
            (13, _, Ok(a), Ok(b), _, Ok(c)) if b<4 && c<4 => Some(Self::EqIR(a, b, c)),
            (14, Ok(a), _, _, Ok(b), Ok(c)) if a<4 && c<4 => Some(Self::EqRI(a, b, c)),
            (15, Ok(a), _, Ok(b), _, Ok(c)) if a<4 && b<4 && c<4 => Some(Self::EqRR(a, b, c)),
            _ => None,
        }
    }

    fn try_all(sample: &str) -> (usize, Vec<usize>) {
        let mut possibilities = Vec::new();

        let lines: Vec<_> = sample.split('\n').collect();
        assert_eq!(lines.len(), 3);

        let before: Vec<_> = lines[0].split(&[' ', ',', '[', ']']).collect();
        assert_eq!(before.len(), 10);
        let registers = [
            before[2].parse().unwrap(),
            before[4].parse().unwrap(),
            before[6].parse().unwrap(),
            before[8].parse().unwrap(),
        ];

        let opcode: Vec<_> = lines[1].split_whitespace().collect();
        assert_eq!(opcode.len(), 4);

        let after: Vec<_> = lines[2].split(&[' ', ',', '[', ']']).collect();
        assert_eq!(after.len(), 11);
        let expected = [
            after[3].parse().unwrap(),
            after[5].parse().unwrap(),
            after[7].parse().unwrap(),
            after[9].parse().unwrap(),
        ];

        for op_nr in 0..16 {
            if let Some(op) = Self::try_parse(op_nr, &opcode[1..4]) {
                let mut actual = registers;
                op.perform(&mut actual);
                if actual == expected {
                    possibilities.push(op_nr);
                }
            }
        }
        (opcode[0].parse().unwrap(), possibilities)
    }
    fn from(line: &str, mappings: &HashMap<usize, HashSet<usize>>) -> Self {
        let components: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(components.len(), 4);

        let op_nr = *mappings.get(&components[0].parse()
                                                 .unwrap())
                                .unwrap()
                                .iter()
                                .next()
                                .unwrap();

        Self::try_parse(op_nr, &components[1..4]).unwrap()
    }
}

pub fn run(input: &str) -> (usize, isize) {
    let (samples, program) = input.split_once("\n\n\n\n").unwrap();
    let possible_mappings: Vec<_> = samples.split("\n\n").map(Operation::try_all).collect();
    // dbg!(&possible_mappings);
    let first = possible_mappings.iter().filter(|(_code, ops)| ops.len() > 2).count();
    let mut mappings: HashMap<usize, HashSet<usize>> = HashMap::new();
    possible_mappings.iter().for_each(|(op_code, possibilities)| {
        mappings.entry(*op_code)
                .and_modify(|e| *e = e.intersection(&possibilities.iter().cloned().collect()).cloned().collect())
                .or_insert(possibilities.iter().cloned().collect());
    });
    loop {
        let known: Vec<_> = mappings.clone().into_iter().filter(|(_op_code, possibilities)| possibilities.len() == 1).collect();
        if known.len() == mappings.len() {
            break;
        }
        for (op_code, mapping) in known {
            let mapping = mapping.iter().next().unwrap();
            mappings.iter_mut().filter(|(code, possibilities)| **code != op_code && possibilities.contains(mapping)).for_each(|(_, possibilities)| { possibilities.remove(mapping); });
        }
    }
    let mut registers = [0; 4];
    program.lines().map(|line| Operation::from(line, &mappings)).for_each(|op| op.perform(&mut registers));

    let second = registers[0];
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    // #[test]
    // fn test_sample() {
    //     let sample_input = read_file("tests/sample_input");
    //     assert_eq!(run(&sample_input), (0, 0));
    // }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (588, 0));
    }
}
