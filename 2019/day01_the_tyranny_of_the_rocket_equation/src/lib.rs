use std::num::ParseIntError;

pub fn run(input: &str) -> Result<(usize, usize), ParseIntError> {
    let first = input.lines().map(|line| line.parse::<usize>().map(|i| i/3-2)).sum::<Result<usize, _>>()?;
    let second = input.lines().map(|line| {
            let parsed = line.parse::<usize>();
            if let Ok(weight) = parsed  {
                let mut total = (weight/3).saturating_sub(2);
                let mut next = (total/3).saturating_sub(2);
                while next > 0 {
                    total += next;
                    next = (next/3).saturating_sub(2);
                }
                Ok(total)
            } else {
                parsed
            }
        }).sum::<Result<usize, _>>()?;
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
        assert_eq!(run(&sample_input), Ok((34241, 51316)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((3363033, 5041680)));
    }
}
