use asm_interpreter::assembly_interpreter::Cpu;

pub fn run(input: &str) -> isize {
    let mut cpu = Cpu::new(input);
    for res in 0.. {
        cpu.reset();
        let mut clock = 1;
        let mut states = Vec::new();
        cpu.set(0, res);
        while let Some(output) = cpu.run() {
            match (clock, output) {
                (0, 1) | (1, 0) => {
                    let new_state = cpu.clone_volatile_state();
                    if states.contains(&new_state) {
                        return res;
                    }
                    clock = output;
                    states.push(new_state);
                },
                _ => break,
            }
        }
    }
    unreachable!("The loop either runs endlessly, or returns early.");
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
        assert_eq!(run(&challenge_input), 158);
    }
}
