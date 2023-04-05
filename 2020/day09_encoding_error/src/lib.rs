use std::{num::ParseIntError, collections::VecDeque};

pub fn run(input: &str, preamble_length: usize) -> Result<(usize, usize), ParseIntError> {
    let numbers: Vec<_> = input.lines().map(|n| n.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
    let mut current = VecDeque::from(numbers[..preamble_length].to_vec());
    let mut first = 0;
    for n in numbers.iter().skip(preamble_length) {
        if contains_pair(&current, *n) {
            current.pop_front();
            current.push_back(*n);
        } else {
            first = *n;
            break;
        }
    }
    let second = get_min_max_sum_of_contiguous(&numbers, first);
    Ok((first, second))
}

fn contains_pair(list: &VecDeque<usize>, target: usize) -> bool {
    list.iter().enumerate().any(|(idx, x)| list.iter().skip(idx+1).any(|y| x+y == target))
}

fn get_min_max_sum_of_contiguous(list: &[usize], target: usize) -> usize {
    for first_idx in 0..list.len() {
        let mut current_sum = 0;
        let mut current_summands = Vec::new();
        let mut last_idx = first_idx;
        while current_sum < target && last_idx < list.len() {
            let summand = list[last_idx];
            current_sum += summand;
            current_summands.push(summand);
            last_idx += 1;
        }
        if current_sum == target {
            return current_summands.iter().min().unwrap() + current_summands.iter().max().unwrap();
        }
    }
    0
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
        assert_eq!(run(&sample_input, 5), Ok((127, 62)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input, 25), Ok((1504371145, 183278487)));
    }
}
