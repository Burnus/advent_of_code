use intcode_processor::intcode_processor::{Cpu, OutputState};
use std::num::ParseIntError;

pub fn run(input: &str) -> Result<(isize, isize), ParseIntError> {
    let mut cpu = Cpu::try_with_memory_from_str(input)?;
    let mut cpu_2 = cpu.clone();
    let instructions = "OR A T
AND B T
AND C T
NOT T J
AND D J
WALK
"; // Jump if A or B or C are false AND D is true (i. e. there is a whole in front of us and we can reach the other end)
    for c in instructions.bytes() {
        cpu.set_input(c as isize);
    }

    let mut first = 0;
    loop {
        match cpu.run() {
            OutputState::Output(i) => print!("{}", (i as u8) as char),
            OutputState::DiagnosticCode(i) => first = i,
            OutputState::Halt => break,
        }
    }

    let instructions = "OR A T
AND B T
AND C T
NOT T J
AND J T
AND D J
OR H T
OR E T
AND T J
RUN
"; // Jump if A or B or C are false AND D is true AND at least one of E and H are true 
//    (i. e. there is a whole in front, we can reach the other end and then step or jump)
    for c in instructions.bytes() {
        cpu_2.set_input(c as isize);
    }

    let mut second = 1;
    loop {
        match cpu_2.run() {
            OutputState::Output(i) => print!("{}", (i as u8) as char),
            OutputState::DiagnosticCode(i) => second = i,
            OutputState::Halt => break,
        }
    }

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
        assert_eq!(run(&challenge_input), Ok((19348404, 1139206699)));
    }
}
