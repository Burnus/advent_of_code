pub fn run(input: &str, amount: usize) -> (usize, usize) {
    let containers: Vec<_> = input.lines().map(|i| i.parse::<usize>().unwrap()).collect();
    let first = get_combinations_for_containers(&containers, amount, usize::MAX);
    let mut second = 0;
    let mut i = 1;
    while second == 0 {
        second = get_combinations_for_containers(&containers, amount, i);
        i += 1;
    }
    (first, second)
}

fn get_combinations_for_containers(containers: &[usize], amount: usize, containers_remaining: usize) -> usize {
    if containers.is_empty() || containers_remaining == 0 {
        match amount {
            0 => 1,
            _ => 0,
        }
    } else {
        let first = containers[0];
        if first > amount {
            get_combinations_for_containers(&containers[1..], amount, containers_remaining)
        } else {
            get_combinations_for_containers(&containers[1..], amount, containers_remaining) +
                get_combinations_for_containers(&containers[1..], amount - first, containers_remaining-1)
        }
    }
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
        assert_eq!(run(&sample_input, 25), (4, 3));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input, 150), (654, 57));
    }
}
