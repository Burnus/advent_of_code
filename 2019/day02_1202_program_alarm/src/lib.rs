use std::num::ParseIntError;

use intcode_processor::intcode_processor::Cpu;

pub fn run(input: &str) -> Result<(isize, isize), ParseIntError> {
    let cpu = Cpu::try_with_memory_from_str(input)?;
    let mut cpu_1 = cpu.clone();
    cpu_1.set(1, 12);
    cpu_1.set(2, 2);
    cpu_1.run();
    let first = cpu_1.get(0);
    let mut second = 0;
    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut cpu_2 = cpu.clone();
            cpu_2.set(1, noun);
            cpu_2.set(2, verb);
            cpu_2.run();
            if cpu_2.get(0) == 19690720 {
                second = 100 * noun + verb;
                break 'outer;
            }
        }
    }
    Ok((first, second))
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
        assert_eq!(run(&sample_input), Ok((3500, 213)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((3760627, 7195)));
    }
}
