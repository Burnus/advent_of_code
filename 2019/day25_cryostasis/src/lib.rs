use intcode_processor::intcode_processor::{Cpu, OutputState};
use std::num::ParseIntError;

pub fn run(input: &str) -> Result<(), ParseIntError> {
    let mut cpu = Cpu::try_with_memory_from_str(input)?;
    loop {
        match cpu.run() {
            OutputState::Output(i) => { print!("{}", (i as u8) as char); continue; },
            OutputState::DiagnosticCode(i) => println!("{}", (i as u8) as char),
            OutputState::Halt => (),
        }
        let mut read = String::new();
        std::io::stdin().read_line(&mut read).expect("Unable to read input");
        read.chars().for_each(|c| cpu.set_input(c as isize));
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
    #[ignore = "Requires manual inputs."]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        // Required items are Cake + Coin + Monolith + Mug. Should output 19013632 in the end.
        assert_eq!(run(&challenge_input), Ok(()));
    }
}
