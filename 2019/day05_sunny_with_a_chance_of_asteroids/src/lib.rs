use std::num::ParseIntError;

use intcode_processor::intcode_processor::{Cpu, OutputState};

pub fn run(input: &str) -> Result<(isize, isize), ParseIntError> {
    let mut cpu = Cpu::try_with_memory_from_str(input)?;
    let mut cpu_2 = cpu.clone();

    cpu.set_input(1);
    let first = run_diagnostics(&mut cpu);
    cpu_2.set_input(5);
    let second = run_diagnostics(&mut cpu_2);
    
    Ok((first, second))
}

fn run_diagnostics(cpu: &mut Cpu) -> isize {
    loop {
        match cpu.run() {
            OutputState::Halt => (),
            OutputState::Output(e) => eprintln!("{e}"),
            OutputState::DiagnosticCode(out) => {
                    return out;
                },
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
        assert_eq!(run(&challenge_input), Ok((9025675, 11981754)));
    }
}
