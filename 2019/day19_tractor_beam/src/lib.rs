use intcode_processor::intcode_processor::{Cpu, OutputState};
use std::{num::ParseIntError, collections::{HashMap, HashSet}};

pub fn run(input: &str) -> Result<(usize, isize), ParseIntError> {
    let cpu = Cpu::try_with_memory_from_str(input)?;
    let first = (0..50).map(|x| (0..50).filter(|y| is_pulled(&cpu, x, *y)).count()).sum();
    let mut x = 0;
    let mut y = 0;
    while !is_pulled(&cpu, x, y+99) {
        while !is_pulled(&cpu, x, y+99) {
            x += 1;
        }
        while !is_pulled(&cpu, x+99, y) {
            y += 1;
        }
    }
    let second = 10_000*x + y;

    Ok((first, second))
}

fn is_pulled(cpu: &Cpu, x: isize, y: isize) -> bool {
    let mut scan = cpu.clone();
    scan.set_input(x);
    scan.set_input(y);
    match scan.run() {
        OutputState::DiagnosticCode(1) => true,
        OutputState::DiagnosticCode(0) => false,
        other => panic!("Unexpected Output State: {other:?}"),
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
        assert_eq!(run(&challenge_input), Ok((226, 7900946)));
    }
}
