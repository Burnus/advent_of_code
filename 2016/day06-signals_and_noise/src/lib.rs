use std::collections::HashMap;

pub fn run(input: &str) -> (String, String) {
    let mut char_map: Vec<HashMap<char, usize>> = Vec::new();
    for _ in 0..input.lines().next().unwrap().len() {
        char_map.push(HashMap::new());
    }
    for l in input.lines() {
        l.chars().enumerate().for_each(|(idx, c)| { 
            char_map[idx].entry(c).and_modify(|freq| *freq += 1).or_insert(0); 
        });
    }
    let first: String = char_map.iter().map(|map| *map.iter().max_by_key(|m| m.1).unwrap().0).collect();
    let second: String = char_map.iter().map(|map| *map.iter().min_by_key(|m| m.1).unwrap().0).collect();
    (first, second)
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
        assert_eq!(run(&sample_input), ("easter".to_string(), "advent".to_string()));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), ("xdkzukcf".to_string(), "cevsgyvd".to_string()));
    }
}
