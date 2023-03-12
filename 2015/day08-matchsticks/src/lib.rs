pub fn run(input: &str) -> (usize, usize) {
    let first = input.lines().map(get_decoding_overhead).sum();
    let second = input.lines().map(get_encoding_overhead).sum();
    (first, second)
}

fn get_decoding_overhead(line: &str) -> usize {
    let mut rendered_len = 0;
    let mut literal = true;
    for c in line.chars() {
        match c {
            '"' => {
                    if !literal {
                        rendered_len += 1;
                        literal = true;
                    }
                },
            '\\' => {
                    if !literal {
                        rendered_len += 1;
                    }
                    literal = !literal;
                },
            'x' => {
                    if literal {
                        rendered_len += 1;
                    } else {
                        rendered_len -= 1;
                        literal = true;
                    }
                },
            _ => rendered_len += 1
        };
    }
    line.chars().count() - rendered_len as usize
}

fn get_encoding_overhead(line: &str) -> usize {
    2 + line.matches('"').count() + line.matches('\\').count()
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
        assert_eq!(run(&sample_input), (12, 19));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (1333, 2046));
    }
}
