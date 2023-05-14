fn find_start_marker(message: &Vec<char>, distinct_character_count: usize) -> Option<usize> {
    'char_iterator: for index in distinct_character_count-1..message.len() {
        let mut found: Vec<char> = Vec::with_capacity(distinct_character_count);
        for offset in 0..distinct_character_count {
            let this_char = message[index+offset+1-distinct_character_count];
            if found.contains(&this_char) { continue 'char_iterator; }
            found.push(this_char);
        }
        return Some(index+1);
    }
    None
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    let chars = input.chars().collect();
    let first = find_start_marker(&chars, 4);
    let second = find_start_marker(&chars, 14);
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
        assert_eq!(run(&sample_input), (Some(7), Some(19)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (Some(1702), Some(3559)));
    }
}
