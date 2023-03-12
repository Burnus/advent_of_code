use asm_interpreter::assembly_interpreter::Cpu;

pub fn run(input: &str) -> (isize, isize) {
    let mut cpu = Cpu::new(input);
    let mut cpu_2 = Cpu::new(input);
    cpu.set(0, 7);
    cpu_2.set(0, 12);
    cpu.run();
    cpu_2.run();
    (cpu.get(0), cpu_2.get(0))
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
        assert_eq!(run(&sample_input), (3, 3));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (12654, 479009214));
    }
}
