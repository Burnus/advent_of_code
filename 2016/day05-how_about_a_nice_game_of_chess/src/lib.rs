use md5::{Digest, Md5};

pub fn run(input: &str) -> (String, String) {
    let mut first = String::new();
    let mut second = String::from("        ");
    let prefix = input.trim();
    for i in 0.. {
        let mut hasher = Md5::new();
        hasher.update(prefix.to_owned() + &(i.to_string()));
        let hash = hasher.finalize();
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            let sixth = char::from_digit(hash[2].into(), 16).unwrap(); 
            if first.len() < 8 {
                first += &sixth.to_string();
            }
            if (b'0'..b'8').contains(&(sixth as u8)) {
                let pos = (sixth as u8 - b'0') as usize;
                let seventh = char::from_digit((hash[3]/16).into(), 16).unwrap();  
                if second.chars().nth(pos) == Some(' ') {
                    second.replace_range(pos..=pos, &seventh.to_string());
                }
            }
            if !second.contains(' ') {
                break;
            }
        }
    }
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
        assert_eq!(run(&sample_input), ("18f47a30".to_string(), "05ace8e3".to_string()));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), ("2414bc77".to_string(), "437e60fc".to_string()));
    }
}
