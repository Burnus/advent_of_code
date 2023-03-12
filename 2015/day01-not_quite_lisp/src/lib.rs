use std::fs::read_to_string;

fn read_file(name: &str) -> String {
    read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
}

fn char_to_floor(c: char) -> isize {
    match c { 
        '(' => 1, 
        ')' => -1, 
        _ => 0,
    }
}

pub fn final_floor(input: &str) -> isize {
    input.chars().map(char_to_floor).sum()
}

pub fn first_basement_pos(input: &str) -> usize {
    let mut floors = input.chars().scan(0, |curr_floor, c| { *curr_floor += char_to_floor(c);Some(*curr_floor) } );
    floors.position(|i| i==-1).expect("Never reached Floor -1") + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        let expected = [0, 0, 3, 3, 3, -1, -1, -3, -3];
        for (line_number, line) in sample_input.lines().enumerate() {
            assert_eq!(final_floor(line), expected[line_number]);
        }
        let samples_2 = [")", "()())"];
        let expected_2 = [1, 5];
        for (sample_number, sample) in samples_2.iter().enumerate() {
            assert_eq!(first_basement_pos(sample), expected_2[sample_number]);
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(final_floor(&challenge_input), 232);
        assert_eq!(first_basement_pos(&challenge_input), 1783);
    }
}
