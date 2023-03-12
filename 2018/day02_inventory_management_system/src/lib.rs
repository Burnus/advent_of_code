use std::collections::HashMap;

pub fn run(input: &str) -> (usize, String) {
    let ids: Vec<_> = input.lines().map(to_id).collect();
    let first = ids.iter().filter(|id| id.values().any(|v| v == &2)).count() * ids.iter().filter(|id| id.values().any(|v| v == &3)).count();
    let mut known_ids: Vec<Vec<u8>> = Vec::new();
    let mut second = String::new();
    input.lines().for_each(|line| {
        for known in &known_ids {
            if let Some(idx) = idx_for_one_off(line.as_bytes(), known) {
                let mut res = line.as_bytes().to_vec();
                res.remove(idx);
                second = String::from_utf8(res).unwrap();
                return;
            }
        }
        known_ids.push(line.as_bytes().to_vec());
    });
    (first, second)
}

fn idx_for_one_off(lhs: &[u8], rhs: &[u8]) -> Option<usize> {
    if lhs.len() == rhs.len() {
        let offsets: Vec<_> = (0..lhs.len()).filter(|i| lhs[*i] != rhs[*i]).collect();
        if offsets.len() == 1 {
            return Some(offsets[0]);
        }
    }
    None
}

fn to_id(line: &str) -> HashMap<&u8, usize> {
    let mut id = HashMap::new();
    line.as_bytes().iter().for_each(|byte| { 
        id.entry(byte).and_modify(|count| *count += 1).or_insert(1);
    });

    id
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
        assert_eq!(run(&sample_input), (12, "fgij".to_string()));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (5952, "krdmtuqjgwfoevnaboxglzjph".to_string()));
    }
}
