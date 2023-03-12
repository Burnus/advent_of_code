pub fn run(input: &str, length_1: usize, length_2: usize) -> (String, String) {
    let mut first = input.to_string();
    let mut len = length_1;
    while first.len() < len {
        first = dragon_tail(&first);
    }
    let mut second = first.to_string();
    first.truncate(len);
    while len % 2 == 0 {
        checksum(&mut first, len);
        len >>= 1;
    }
    len = length_2;
    while second.len() < len {
        second = dragon_tail(&second);
    }
    second.truncate(len);
    while len % 2 == 0 {
        checksum(&mut second, len);
        len >>= 1;
    }
    (first, second)
}

fn dragon_tail(input: &str) -> String {
    let mut res = input.to_string();
    res.push('0');
    input.bytes().rev().for_each(|b| {res.push(match b { b'0' => '1', _ => '0' })});
    res
}

fn checksum(data: &mut String, length: usize) {
    let checksum_len = length/2;
    let mut res = String::with_capacity(checksum_len);
    let mut old = data.bytes();
    for _ in 0..checksum_len {
        let l = &old.next().unwrap();
        let r = &old.next().unwrap();
        res.push(match l == r { true => '1', false => '0' });
    }
    std::mem::swap(data, &mut res);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
    }

    #[test]
    fn test_sample() {
        let mut sample_input = read_file("tests/sample_input");
        sample_input = sample_input.trim().to_string();
        assert_eq!(run(&sample_input, 20, 35651584), ("01100".to_string(), "10111110011110111".to_string()));
    }

    #[test]
    fn test_challenge() {
        let mut challenge_input = read_file("tests/challenge_input");
        challenge_input = challenge_input.trim().to_string();
        assert_eq!(run(&challenge_input, 272, 35651584), ("10010010110011010".to_string(), "01010100101011100".to_string()));
    }
}
