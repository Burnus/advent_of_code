use std::collections::HashSet;

struct State {
    write_0: bool,
    write_1: bool,
    move_0: isize,
    move_1: isize,
    next_0: usize,
    next_1: usize,
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        let lines: Vec<_> = value.lines().collect();
        let write_0 = match lines[2].split_whitespace().nth(4) {
            Some("1.") => true,
            Some("0.") => false,
            v => panic!("Illegal value to write on 0: {v:?}"),
        };
        let move_0 = match lines[3].split_whitespace().nth(6) {
            Some("right.") => 1,
            Some("left.") => -1,
            v => panic!("Illegal direction to move on 0: {v:?}"),
        };
        let next_0 = match lines[4].split_whitespace().nth(4).unwrap().bytes().next() {
            Some(v) if (b'A'..=b'Z').contains(&v) => (v - b'A') as usize,
            e => panic!("Illegal state to continue on 0: {e:?}"),
        };
        let write_1 = match lines[6].split_whitespace().nth(4) {
            Some("1.") => true,
            Some("0.") => false,
            v => panic!("Illegal value to write on 1: {v:?}"),
        };
        let move_1 = match lines[7].split_whitespace().nth(6) {
            Some("right.") => 1,
            Some("left.") => -1,
            v => panic!("Illegal direction to write on 1: {v:?}"),
        };
        let next_1 = match lines[8].split_whitespace().nth(4).unwrap().bytes().next() {
            Some(v) if (b'A'..=b'Z').contains(&v) => (v - b'A') as usize,
            e => panic!("Illegal state to continue on 1: {e:?}"),
        };

        Self {
            write_0,
            write_1,
            move_0,
            move_1,
            next_0,
            next_1,
        }
    }
}

#[derive(Default)]
struct Cpu {
    tape: HashSet<isize>,
    cursor: isize,
    state_machine: Vec<State>,
    state_ptr: usize,
}

impl Cpu {
    fn run_for(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    } 

    fn step(&mut self) {
        let current_state = &self.state_machine[self.state_ptr];
        match self.tape.contains(&self.cursor) {
            true => {
                    if !current_state.write_1 {
                        self.tape.remove(&self.cursor);
                    }
                    self.cursor += current_state.move_1;
                    self.state_ptr = current_state.next_1;
                },
            false => {
                    if current_state.write_0 {
                        self.tape.insert(self.cursor);
                    }
                    self.cursor += current_state.move_0;
                    self.state_ptr = current_state.next_0;
                },
        }
    }

    fn checksum(&self) -> usize {
        self.tape.len()
    }
}

pub fn run(input: &str) -> usize {
    let states: Vec<_> = input.split("\n\n").collect();
    let initial = states[0].split_once('\n').unwrap();
    let initial_state_str = initial.0.split_whitespace().nth(3).unwrap();
    let initial_state = (initial_state_str.bytes().next().unwrap() - b'A') as usize;
    let step_count = initial.1.split_whitespace().nth(5).unwrap().parse::<usize>().unwrap();
    
    let state_machine: Vec<_> = states[1..].iter().map(|s| State::from(*s)).collect();
    let mut cpu = Cpu {
        state_machine,
        state_ptr: initial_state,
        ..Default::default()
    };
    cpu.run_for(step_count);
    cpu.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), 3);
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), 4769);
    }
}
