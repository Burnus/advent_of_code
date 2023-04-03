use std::num::ParseIntError;

pub fn run(input: &str) -> Result<(usize, usize), ParseIntError> {
    let mut expenses: Vec<_> = input.lines().map(|line| line.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
    expenses.sort();
    let first = summands_for(2020, &expenses).map(|(a, b)| a*b).unwrap();
    let second = expenses.iter().find_map(|&a| {
        if let Some((b, c)) = summands_for(2020-a, &expenses) {
            if a != b && a != c && b != c {
                Some(a*b*c)
            } else {
                None
            }
        } else {
            None
        }
    }).unwrap();
    Ok((first, second))
}

fn summands_for(target: usize, sorted_list: &[usize]) -> Option<(usize, usize)> {
    sorted_list.iter().find(|&a| sorted_list.binary_search(&(target-a)).is_ok()).map(|&a| (a, (target-a)))
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
        assert_eq!(run(&sample_input), Ok((514579, 241861950)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((964875, 158661360)));
    }
}
