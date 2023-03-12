pub fn run(input: &str) -> (usize, usize) {
    let first = input.lines().filter(|l| supports_tls(l)).count();
    let second = input.lines().filter(|l| supports_ssl(l)).count();
    (first, second)
}

fn supports_ssl(address: &str) -> bool {
    let components: Vec<_> = address.split(&['[', ']']).collect();
    components.iter().step_by(2).any(|component| {
        (0..=component.len()-3).any(|char_idx| {
            let a = component.chars().nth(char_idx).unwrap();
            let b = component.chars().nth(char_idx+1).unwrap();
            component.chars().nth(char_idx+2).unwrap() == a && 
                b != a && 
                components.iter().skip(1).step_by(2).any(|comp_2| comp_2.contains(&(b.to_string() + &a.to_string() + &b.to_string())))
        })
    })
}

fn supports_tls(address: &str) -> bool {
    let components: Vec<_> = address.split(&['[', ']']).collect();
    !components.iter().skip(1).step_by(2).any(|comp| contains_abba(comp)) &&
        components.iter().step_by(2).any(|comp| contains_abba(comp))
}

fn contains_abba(component: &str) -> bool {
    component.len() >= 4 && (0..=component.len()-4).any(|char_idx| 
        component.chars().nth(char_idx) == component.chars().nth(char_idx+3) &&
            component.chars().nth(char_idx+1) == component.chars().nth(char_idx+2) &&
            component.chars().nth(char_idx) != component.chars().nth(char_idx+1))
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
        for (idx, line) in sample_input.lines().enumerate() {
            assert_eq!(supports_tls(line), [0, 3].contains(&idx));
            assert_eq!(supports_ssl(line), [4, 6, 7].contains(&idx));
        }
        assert_eq!(run(&sample_input), (2, 3));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (115, 231));
    }
}
