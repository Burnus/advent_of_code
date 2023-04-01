use intcode_processor::intcode_processor::{Cpu, OutputState};
use std::{num::ParseIntError, collections::VecDeque};

pub fn run(input: &str) -> Result<(isize, isize), ParseIntError> {
    let program = Cpu::try_with_memory_from_str(input)?;
    let mut cpus = Vec::new();
    for addr in 0..50 {
        let mut cpu = program.clone();
        cpu.set_input(addr);
        cpu.set_input(-1);
        cpus.push(cpu);
    }
    let mut first = 0;
    let mut nat = (0, 0);
    let mut last = 0;
    let mut messages = vec![VecDeque::new(); 50];
    loop {
        let mut halted = 0;
        for (id, cpu) in cpus.iter_mut().enumerate() {
            while let Some((x, y)) = messages[id].pop_front() {
                cpu.set_input(x);
                cpu.set_input(y);
            }
            cpu.set_input(-1);
            match cpu.run() {
                OutputState::Halt => halted += 1,
                OutputState::DiagnosticCode(i) => eprintln!("CPU {id} stopped with DiagnosticCode {i}"),
                OutputState::Output(i) => {
                    let x = match cpu.run() {
                        OutputState::Halt => panic!("Received Halt instead of X"),
                        OutputState::DiagnosticCode(i) => panic!("CPU {id} stopped with DiagnosticCode {i}"),
                        OutputState::Output(i) => i,
                    };
                    let y = match cpu.run() {
                        OutputState::Halt => panic!("Received Halt instead of Y"),
                        OutputState::DiagnosticCode(i) => panic!("CPU {id} stopped with DiagnosticCode {i}"),
                        OutputState::Output(i) => i,
                    };
                    if i == 255 {
                        if first == 0 {
                            first = y;
                        }
                        nat = (x, y);
                    } else {
                        messages[i as usize].push_back((x, y));
                    }
                }
            }
        }
        if halted == 50 {
            if last == nat.1 {
                return Ok((first, last));
            } else {
                last = nat.1;
                messages[0].push_back(nat);
            }
        }
    }
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
        assert_eq!(run(&challenge_input), Ok((17949, 12326)));
    }
}
