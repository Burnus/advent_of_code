pub fn run(input: &str) -> (usize, usize) {
    let first = decompress_v1(input.trim()).chars().count();
    let second = decompress_v2(input.trim());
    (first, second)
}

fn decompress_v2(input: &str) -> usize {
    if let Some(start_idx) = input.find('(') {
        let end_idx = input[start_idx..].find(')').unwrap();
        let (l, r) = input[start_idx+1..start_idx+end_idx].split_once('x').unwrap();
        let substring_len = l.parse::<usize>().unwrap();
        let multiplier = r.parse::<usize>().unwrap();
        assert!(start_idx+end_idx+substring_len<input.len());

        start_idx + multiplier*decompress_v2(&input[start_idx+end_idx+1..=start_idx+end_idx+substring_len]) + decompress_v2(&input[start_idx+end_idx+substring_len+1..])
    } else {
        input.chars().count()
    }
}

fn decompress_v1(input: &str) -> String {
    let mut res = String::new();

    let mut idx = 0;
    while idx < input.chars().count() {
        let this_char = input.chars().nth(idx).unwrap();
        if this_char == '(' {
            let marker_len = input[idx..].find(')').unwrap();
            let (l, r) = input[idx+1..idx+marker_len]
                        .split_once('x')
                        .unwrap_or_else(|| panic!("Error between {} and {}.", idx+1, idx+marker_len));
            let substring_len = l.parse::<usize>().unwrap();
            let repetitions = r.parse::<usize>().unwrap();
            res += &input[idx+marker_len+1..idx+marker_len+substring_len+1].repeat(repetitions);
            idx += marker_len+substring_len+1;
        } else {
            res.push(this_char);
            idx += 1;
        }
    }

    res
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
        let sample_input = read_file("tests/sample_input");
        let input: Vec<_> = sample_input.lines().collect();
        let expected_v1 = [
            "ADVENT",
            "ABBBBBC",
            "XYZXYZXYZ",
            "ABCBCDEFEFG",
            "(1x3)A",
            "X(3x3)ABC(3x3)ABCY",
        ];
        let expected_v2 = [
            "ADVENT",
            "ABBBBBC",
            "XYZXYZXYZ",
            "ABCBCDEFEFG",
            "AAA",
            "XABCABCABCABCABCABCY",
        ];
        for (idx, res) in expected_v1.iter().enumerate() {
            assert_eq!(decompress_v1(input[idx]), *res.to_string());
            assert_eq!(run(input[idx]), (res.chars().count(), expected_v2[idx].chars().count()));
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (102239, 10780403063));
    }
}
