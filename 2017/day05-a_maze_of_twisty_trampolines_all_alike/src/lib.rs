pub fn run(input: &str) -> (usize, usize) {
    let mut jumps: Vec<_> = input.lines().map(|i| i.parse::<isize>().unwrap()).collect();
    let mut jumps_2 = jumps.to_vec();
    let max = jumps.len() as isize;
    let mut instr_ptr = 0;
    let mut first = 0;
    while (0..max).contains(&instr_ptr) {
        let this_num = jumps[instr_ptr as usize];
        jumps[instr_ptr as usize] += 1;
        instr_ptr += this_num;
        first += 1;
    }
    instr_ptr = 0;
    let mut second = 0;
    while (0..max).contains(&instr_ptr) {
        let this_num = jumps_2[instr_ptr as usize];
        if this_num >= 3 {
            jumps_2[instr_ptr as usize] -= 1;
        } else {
            jumps_2[instr_ptr as usize] += 1;
        }
        instr_ptr += this_num;
        second += 1;
    }
    (first, second)
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
        assert_eq!(run(&sample_input), (5, 10));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (339351, 24315397));
    }
}
