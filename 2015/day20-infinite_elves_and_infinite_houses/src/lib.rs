pub fn run(input: usize) -> (usize, usize) {
    let first = presents_count_for_house(input, 10, None);
    let second = presents_count_for_house(input, 11, Some(50));
    (first, second)
}

fn presents_count_for_house(number: usize, multiplier: usize, max: Option<usize> ) -> usize {
    let max = max.unwrap_or(number/multiplier);
    let size = number/multiplier+1;
    let mut counts = vec![0; size];
    (1..=size).for_each(|n| {
        (n..=size.min(max*n)).step_by(n).for_each(|i| {
            counts[i-1] += multiplier*n;
        });
    });
    counts.iter().position(|i| *i >= number).unwrap_or(0) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let exptected = [
            (9, (1, 1)),
            (10, (1, 1)),
            (11, (2, 1)),
            (30, (2, 2)),
            (40, (3, 3)),
            (70, (4, 4)),
            (60, (4, 4)),
            (120, (6, 6)),
            (80, (6, 6)),
            (150, (8, 8)),
            (130, (8, 6)),
        ];
        for (input, output) in exptected {
            assert_eq!(run(input), output);
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = 29_000_000;
        assert_eq!(run(challenge_input), (665280, 705600));
    }
}
