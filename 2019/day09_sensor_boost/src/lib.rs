use intcode_processor::intcode_processor::{Cpu, OutputState};

pub fn run(input: &str) -> (isize, isize) {
    let mut cpu = Cpu::try_with_memory_from_str(input).unwrap();
    let mut cpu_2 = cpu.clone();
    cpu.set_input(1);
    let first = match cpu.run() {
        OutputState::DiagnosticCode(d) => d,
        e => panic!("Unexpected return code: {e:?}"),
    };
    cpu_2.set_input(2);
    let second = match cpu_2.run() {
        OutputState::DiagnosticCode(d) => d,
        e => panic!("Unexpected return code: {e:?}"),
    };
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
        assert_eq!(run(&challenge_input), (2316632620, 78869));
    }
}
