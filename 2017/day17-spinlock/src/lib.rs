pub fn run(step: usize) -> (usize, usize) {
    let mut buffer = Vec::from([0]);
    let mut current_position = 0;
    for iteration in 1..=2017 {
        current_position = (current_position + step) % iteration + 1;
        buffer.insert(current_position, iteration);
    }
    let first = buffer[(current_position+1) % 2018];
    let mut second = buffer[1];
    for iteration in 2018..=50_000_000 {
        current_position = (current_position + step) % iteration + 1;
        if current_position == 1 {
            second = iteration
        }
    }
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = 3;
        assert_eq!(run(sample_input), (638, 1222153));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = 370;
        assert_eq!(run(challenge_input), (1244, 11162912));
    }
}
