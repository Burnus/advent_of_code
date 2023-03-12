use asm_interpreter::assembly_interpreter::Cpu;

pub fn run(input: &str) -> (isize, isize) {
    let mut cpu = Cpu::new(input);
    cpu.run();
    let first = cpu.get(0);
    cpu.reset();
    cpu.set(2, 1);
    cpu.run();
    let second = cpu.get(0);
    (first, second)
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
        assert_eq!(run(&sample_input), (42, 42));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (318007, 9227661));
    }
}
