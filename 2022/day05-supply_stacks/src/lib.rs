use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidInput,
    ParseIntError(std::num::ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput => write!(f, "Invalid Input: Didn't contain two areas separated by an empty line"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

enum Mode { SingleCrate, MultiCrate }

#[derive(Clone)]
struct State {
    stacks: Vec<Vec<char>>,
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        let stack_count = (value.lines().last().unwrap().len() + 1) / 4;
        let mut stacks = vec![Vec::new(); stack_count];
        
        value.lines().rev().skip(1).for_each(|line| {
            (0..stack_count).for_each(|stack| {
                let supply_crate = line.chars().nth(stack*4+1).unwrap_or(' ');
                if supply_crate != ' ' {
                    stacks[stack].push(supply_crate);
                }
            });
        });
        Self {
            stacks,
        }
    }
}

impl State {
    fn move_crates(&mut self, crate_count: usize, source: usize, destination: usize,) {
        let source_length = self.stacks[source].len();
        let move_stack = self.stacks[source].split_off(source_length.saturating_sub(crate_count));
        self.stacks[destination].extend(move_stack.into_iter());
    }

    fn perform(&mut self, instruction: &str, operation_mode: Mode) -> Result<(), ParseError> {
        let elements: Vec<&str> = instruction.split(' ').collect();
        if elements.len()>4 {
            let (crate_count, source, destination) = (elements[1].parse::<usize>()?, elements[3].parse::<usize>()?, elements[5].parse::<usize>()?);
            match operation_mode {
                Mode::SingleCrate => for _ in 0..crate_count {
                        self.move_crates(1, source-1, destination-1);
                    },
                Mode::MultiCrate => {self.move_crates(crate_count, source-1, destination-1);},
            }
        }
        Ok(())
    }

    fn top_str(&self) -> String {
        self.stacks.iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .collect()
    }
}

pub fn run(input: &str) -> Result<(String, String), ParseError> {
    if let Some((initial_state, instructions)) = input.split_once("\n\n") {
        let mut state_9000 = State::from(initial_state);
        let mut state_9001 = state_9000.clone();
        for instruction in instructions.lines() {
            state_9000.perform(instruction, Mode::SingleCrate)?;
            state_9001.perform(instruction, Mode::MultiCrate)?;
        }
        let first = state_9000.top_str();
        let second = state_9001.top_str();
        Ok((first, second))
    } else {
        Err(ParseError::InvalidInput)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..])
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), Ok(("CMZ".to_string(), "MCD".to_string())));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok(("QNHWJVJZW".to_string(), "BPCZJLFJW".to_string())));
    }
}
