use std::fs;

enum Mode { SingleCrate, MultiCrate }

#[derive(Clone)]
struct State {
    stacks: Vec<Vec<char>>,
}

impl State {
    fn from_initial(initial_state: &str) -> Self {
        let stack_count = (initial_state.lines().last().unwrap().len() + 1) / 4;
        let mut stacks = vec![Vec::new(); stack_count];
        
        for line in initial_state.lines().rev().skip(1) {
            for stack in 0..stack_count {
                let supply_crate = line.chars().nth(stack*4+1).unwrap();
                if supply_crate != ' ' {
                    stacks[stack].push(supply_crate);
                }
            }
        }
        State {
            stacks,
        }
    }

    fn move_crates(&mut self, crate_count: usize, source: usize, destination: usize,) {
        let source_length = self.stacks[source].len();
        let mut move_stack = self.stacks[source].split_off(source_length-crate_count);
        self.stacks[destination].append(&mut move_stack);
    }

    fn perform(&mut self, instruction: &str, operation_mode: Mode) {
        let elements: Vec<&str> = instruction.split(' ').collect();
        if elements.len()>4 {
            let (crate_count, source, destination) = (elements[1].parse::<usize>().unwrap(), elements[3].parse::<usize>().unwrap(), elements[5].parse::<usize>().unwrap());
            match operation_mode {
                Mode::SingleCrate => for _ in 0..crate_count {
                        self.move_crates(1, source-1, destination-1);
                    },
                Mode::MultiCrate => {self.move_crates(crate_count, source-1, destination-1);},
            }
        }
    }

    fn top_str(&self) -> String {
        self.stacks.iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .collect()
    }
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn main() {
    //let contents = read_file("sample_input");
    let contents = read_file("input");
    
    if let Some((initial_state, instructions)) = contents.split_once("\n\n") {
        let mut state_9000 = State::from_initial(initial_state);
        let mut state_9001 = state_9000.clone();
        for instruction in instructions.lines() {
            state_9000.perform(instruction, Mode::SingleCrate);
            state_9001.perform(instruction, Mode::MultiCrate);
        }
        println!("With CrateMover 9000, the top crates at the end are {}", state_9000.top_str());
        println!("With CrateMover 9001, the top crates at the end are {}", state_9001.top_str());
    }
}

 #[test]
fn sample_input() {
    let contents = read_file("tests/sample_input");

    if let Some((initial_state, instructions)) = contents.split_once("\n\n") {
        let mut state_9000 = State::from_initial(initial_state);
        let mut state_9001 = state_9000.clone();
        for instruction in instructions.lines() {
            state_9000.perform(instruction, Mode::SingleCrate);
            state_9001.perform(instruction, Mode::MultiCrate);
        }
        assert_eq!(state_9000.top_str(), "CMZ");
        assert_eq!(state_9001.top_str(), "MCD");
    } else {
        panic!("Unable to split input: {contents}");
    }
}

 #[test]
fn challenge_input() {
    let contents = read_file("tests/input");

    if let Some((initial_state, instructions)) = contents.split_once("\n\n") {
        let mut state_9000 = State::from_initial(initial_state);
        let mut state_9001 = state_9000.clone();
        for instruction in instructions.lines() {
            state_9000.perform(instruction, Mode::SingleCrate);
            state_9001.perform(instruction, Mode::MultiCrate);
        }
        assert_eq!(state_9000.top_str(), "QNHWJVJZW");
        assert_eq!(state_9001.top_str(), "BPCZJLFJW");
    } else {
        panic!("Unable to split input: {contents}");
    }
}
