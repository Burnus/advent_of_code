use md5::{Md5, Digest};

pub fn run(input: &str) -> (usize, usize) {
    let first = iv_for_nth_key(input.trim(), 64, 1);
    let second = iv_for_nth_key(input.trim(), 64, 2017);
    (first, second)
}

fn get_hash(salt: &str, iv: usize, stretching: u16, known_hashes: &mut Vec<([u8; 32], Option<u8>, bool)>) -> ([u8; 32], Option<u8>, bool) {
    if known_hashes.len() > iv {
        return known_hashes[iv];
    }
    let mut previous = salt.to_owned() + &(iv.to_string()[..]);
    let mut hex = [0_u8; 32];
    (0..stretching).for_each(|_| {
        let mut hasher = Md5::new();
        hasher.update(&previous);
        hex = hasher.finalize().iter().flat_map(|byte| [byte / 16, byte % 16]).collect::<Vec<u8>>().try_into().unwrap();
        previous = hex.iter().map(|i| match i {
                digit if digit < &10 => (b'0' + digit) as char,
                alpha => (b'a' + alpha - 10) as char,
            }).collect();
    });
    let first_3_tuple = hex.windows(3).find(|&w| w[0] == w[1] && w[1] == w[2]).map(|w| w[0]);
    let contains_5_tuple = first_3_tuple.is_some() && hex.windows(5).any(|w| w[0] == w[1] && w[1] == w[2] && w[2] == w[3] && w[3] == w[4]);
    known_hashes.push((hex, first_3_tuple, contains_5_tuple));
    (hex, first_3_tuple, contains_5_tuple)
}

fn is_key(first_3_tuple: u8, salt: &str, next_iv: usize, stretching: u16, known_hashes: &mut Vec<([u8; 32], Option<u8>, bool)>) -> bool {
        for iv in next_iv..next_iv+1000 {
            let (hash, _, contains_5_tuple) = get_hash(salt, iv, stretching, known_hashes);
            if contains_5_tuple && hash.windows(5).any(|w| w[0] == first_3_tuple && w[0] == w[1] && w[0] == w[2] && w[0] == w[3] && w[0] == w[4]) {
                return true;
            }
        }
    false
}

fn iv_for_nth_key(salt: &str, n: usize, stretching: u16) -> usize {
    let mut iv = 0;
    let mut known_hashes = Vec::new();
    (0..n).for_each(|_| {
        loop {
            iv += 1;
            if let Some(chars) = get_hash(salt, iv-1, stretching, &mut known_hashes).1 {
                if is_key(chars, salt, iv, stretching, &mut known_hashes) { break; }
            }
        }
    });

    iv-1
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
        assert_eq!(run(&sample_input), (22728, 22551));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (23769, 20606));
    }
}
