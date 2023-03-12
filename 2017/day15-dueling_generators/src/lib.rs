pub fn run(input: &str) -> (usize, usize) {
    let starting: Vec<usize> = input.lines().map(|line| line.split_whitespace().last().unwrap().parse::<usize>().unwrap()).collect();
    let (mut curr_a, mut curr_b) = (starting[0], starting[1]);
    let mut first = 0;
    for _ in 0..40_000_000 {
        curr_a = (curr_a * 16807) % 2147483647;
        curr_b = (curr_b * 48271) % 2147483647;
        if curr_a % 0x10000 == curr_b % 0x10000 {
            first += 1;
        }
    }
    (curr_a, curr_b) = (starting[0], starting[1]);
    let mut second = 0;
    for _ in 0..5_000_000 {
        loop {
            curr_a = (curr_a * 16807) % 2147483647;
            if curr_a % 4 == 0 {
                break;
            }
        }
        loop {
            curr_b = (curr_b * 48271) % 2147483647;
            if curr_b % 8 == 0 {
                break;
            }
        }
        if curr_a % 0x10000 == curr_b % 0x10000 {
            second += 1;
        }
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
        assert_eq!(run(&sample_input), (588, 309));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (638, 343));
    }
}
