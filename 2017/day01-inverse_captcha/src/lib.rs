pub fn run(input: &str) -> (usize, usize) {
    let in_bytes = input.as_bytes();
    let first = in_bytes.windows(2)
        .filter(|b| b[0] == b[1])
        .map(|b| (b[0] - b'0') as usize)
        .sum::<usize>() + 
            if in_bytes.first().unwrap() == in_bytes.last().unwrap() { 
                (*in_bytes.first().unwrap() - b'0') as usize 
            } else { 
                0 
            };
    let half = in_bytes.len()/2;
    let second = (0..half)
        .filter(|i| in_bytes[*i] == in_bytes[*i+half])
        .map(|i| (in_bytes[i] - b'0') as usize)
        .sum::<usize>() * 2;
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
        let sample_inputs = [
            "1122",
            "1111",
            "1234",
            "91212129",
            "1212",
            "1221",
            "123425",
            "123123",
            "12131415",
        ];
        let expected = [
            (3,0),
            (4,4),
            (0,0),
            (9,6),
            (0,6),
            (3,0),
            (0,4),
            (0,12),
            (0,4),
        ];
        for (idx, sample_input) in sample_inputs.iter().enumerate() {
            assert_eq!(run(sample_input), expected[idx]);
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (1182, 1152));
    }
}
