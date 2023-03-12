pub fn run(input: &str) -> (usize, usize) {
    let elf_count = input.parse().unwrap();
    let first = steal_left(elf_count);
    let second = steal_across(elf_count);
    (first, second)
}


fn steal_across(elf_count: usize) -> usize {
    let prev_pow_of_3 = 3_usize.pow(elf_count.ilog(3));
    if elf_count <= 2 * prev_pow_of_3 {
        elf_count - prev_pow_of_3
    } else {
        2 * elf_count - 3 * prev_pow_of_3
    }
}

fn steal_left(elf_count: usize) -> usize {
    2 * elf_count - (elf_count+1).next_power_of_two() + 1
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
        assert_eq!(run(&sample_input), (3, 2));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (1834903, 1420280));
    }
}
