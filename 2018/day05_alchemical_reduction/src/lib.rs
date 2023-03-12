pub fn run(input: &str) -> (usize, usize) {
    let mut bytes: Vec<u8> = input.bytes().collect();
    bytes = fully_reduce(&bytes);
    let first = bytes.len();
    let second = (b'A'..=b'Z').map(|b| {
            let mut new_bytes = bytes.to_vec();
            while let Some(idx) = new_bytes.iter().position(|c| *c == b || *c == b+b'a'-b'A') {
                new_bytes.remove(idx);
            }
            fully_reduce(&new_bytes).len()
        }).min().unwrap();
    (first, second)
}

fn fully_reduce(bytes: &[u8]) -> Vec<u8> {
    let mut bytes = bytes.to_vec();
    let mut len = bytes.len();
    loop {
        bytes = reduce(&bytes);
        if bytes.len() == len {
            return bytes;
        }
        len = bytes.len();
    }
}

fn reduce(bytes: &[u8]) -> Vec<u8> {
    let mut res = bytes.to_vec();
    let cancelled: Vec<_> = (0..bytes.len()-1).filter(|i| bytes[*i].abs_diff(bytes[*i+1]) == b'a'-b'A').collect();
    cancelled.iter().rev().for_each(|i| {
        if !cancelled.contains(&(i+1)) {
            res.remove(*i+1);
            res.remove(*i);
        }
    });
    res
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
        assert_eq!(run(&sample_input), (10, 4));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (9348, 4996));
    }
}
