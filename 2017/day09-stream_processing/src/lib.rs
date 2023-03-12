pub fn run(stream: &str) -> (usize, usize) {
    let mut garbage = false;
    let mut ignore_next = false;
    let mut score = 0;
    let mut garbage_count = 0;
    let mut current_depth = 0;

    stream.chars().for_each(|c| {
        if ignore_next {
            ignore_next = false;
        } else if garbage {
            match c {
                '!' => ignore_next = true,
                '>' => garbage = false,
                _ => garbage_count += 1,
            }
        } else {
            match c {
                '<' => garbage = true,
                '{' => {
                    current_depth += 1;
                    score += current_depth;
                },
                '}' => current_depth -= 1,
                _ => (),
            }
        }
    });

    (score, garbage_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (8337, 4330));
    }
}
