use std::num::ParseIntError;

use intcode_processor::intcode_processor::{Cpu, OutputState};

pub fn run(input: &str) -> Result<(isize, isize), ParseIntError> {
    let mut cpu = Cpu::try_with_memory_from_str(input)?;
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
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((2316632620, 78869)));
    }
}
