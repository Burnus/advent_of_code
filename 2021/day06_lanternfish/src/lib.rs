use std::num::ParseIntError;

pub fn run(input: &str) -> Result<(usize, usize), ParseIntError> {
    let fish: Vec<_> = input.split(',').map(|i| i.parse::<u8>()).collect::<Result<Vec<_>, _>>()?;
    let mut ages: Vec<usize> = (0..=8).map(|age| fish.iter().filter(|fish_age| **fish_age == age).count()).collect();
    for _ in 0..80 {
        let new = ages[0];
        (0..8).for_each(|age| ages[age] = ages[age+1]);
        ages[6] += new;
        ages[8] = new;
    }
    let first = ages.iter().sum();
    for _ in 80..256 {
        let new = ages[0];
        (0..8).for_each(|age| ages[age] = ages[age+1]);
        ages[6] += new;
        ages[8] = new;
    }
    let second = ages.iter().sum();
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
        assert_eq!(run(&sample_input), Ok((5934, 26984457539)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((366057, 1653559299811)));
    }
}
