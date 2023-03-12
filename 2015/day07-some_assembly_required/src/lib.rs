use std::collections::HashMap;

#[derive(Clone)]
pub enum Instruction {
    And(String, String),
    Or(String, String),
    Not(String),
    Lshift(String, u8),
    Rshift(String, u8),
    Equal(String),
}

impl Instruction {
    fn parse(line: &str) -> (String, Instruction) {
        let components: Vec<&str> = line.split(' ').collect();
        match components[1] {
            "->" => (components[2].to_string(), Instruction::Equal(components[0].to_string())),
            "AND" => (components[4].to_string(), Instruction::And(components[0].to_string(), components[2].to_string())),
            "OR" => (components[4].to_string(), Instruction::Or(components[0].to_string(), components[2].to_string())),
            "LSHIFT" => (components[4].to_string(), Instruction::Lshift(components[0].to_string(), components[2].parse().unwrap())),
            "RSHIFT" => (components[4].to_string(), Instruction::Rshift(components[0].to_string(), components[2].parse().unwrap())),
            _ => (components[3].to_string(), Instruction::Not(components[1].to_string())),
        }
    }
}

pub fn eval_for(wire: String, circuit: &mut HashMap<String, Instruction>) -> u16 {
    if let Ok(num) = wire.parse::<u16>() {
        return num;
    }
    let instruction = Instruction::clone(circuit.get(&wire).unwrap_or_else(|| panic!("Wire not found: {wire}")));
    let result = match instruction {
        Instruction::Equal(val) => eval_for(val, circuit),
        Instruction::Not(val) => !(eval_for(val, circuit)),
        Instruction::And(l, r) => (eval_for(l, circuit)) & (eval_for(r, circuit)),
        Instruction::Or(l, r) => (eval_for(l, circuit)) | (eval_for(r, circuit)),
        Instruction::Lshift(val, bits) => eval_for(val, circuit) << bits,
        Instruction::Rshift(val, bits) => eval_for(val, circuit) >> bits,
    };
    circuit.insert(wire, Instruction::Equal(result.to_string()));
    result
}

pub fn assemble(input: &str) -> HashMap<String, Instruction> {
    input.lines()
        .map(Instruction::parse)
        .collect()
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
        let mut circuit = assemble(&sample_input);
        let expected = [
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ];
        for (wire, output) in expected {
            assert_eq!(eval_for(wire.to_string(), &mut circuit), output);
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        let mut circuit = assemble(&challenge_input);
        let first_a = eval_for(String::from("a"), &mut circuit.clone());
        assert_eq!(first_a, 46065);
        circuit.insert(String::from("b"), Instruction::Equal(first_a.to_string()));
        assert_eq!(eval_for(String::from("a"), &mut circuit), 14134);
    }
}
