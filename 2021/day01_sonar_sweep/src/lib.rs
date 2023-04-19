use std::num::ParseIntError;

pub fn run(input: &str) -> Result<(usize, usize), ParseIntError> {
    let measurements: Vec<_> = input.lines().map(|i| i.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
    let first = measurements.windows(2).filter(|w| w[1] > w[0]).count();
    let second = measurements.windows(3).map(|w| w.iter().sum::<usize>()).collect::<Vec<_>>().windows(2).filter(|w| w[1] > w[0]).count();
    Ok((first, second))
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
        assert_eq!(run(&sample_input), Ok((7, 5)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((1832, 1858)));
    }
}
