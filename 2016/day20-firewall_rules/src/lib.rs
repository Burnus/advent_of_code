pub fn run(input: &str, max: usize) -> (usize, usize) {
    let mut blocked: Vec<_> = input.lines().map(parse_range).collect();
    blocked.sort();
    //let first = (0..).find(|i| !blocked.iter().any(|(lower, upper)| (lower..=upper).contains(&i))).unwrap();
    //let second = (first..=max).filter(|i| !blocked.iter().any(|(lower, upper)|(lower..=upper).contains(&i))).count();
    let first = find_first(&blocked);
    let second = count_all(blocked, first, max);
    (first, second)
}

fn count_all(mut ranges: Vec<(usize, usize)>, first: usize, max: usize) -> usize {
    let mut count = 0;
    let mut current = first;
    while current < max {
        count += 1;
        current += 1;
        while let Some((_, upper)) = ranges.iter().find(|(l, u)| (l..=u).contains(&&current))  {
            current = upper + 1;
            let mut new_ranges: Vec<_> = ranges.iter().filter(|(_, u)| u > &current).cloned().collect();
            std::mem::swap(&mut ranges, &mut new_ranges);
        }
    }

    count
}

fn find_first(ranges: &[(usize, usize)]) -> usize {
    let mut current = 0;
    while let Some((_, upper)) = ranges.iter().find(|(l, u)| (l..=u).contains(&&current)) {
        current = upper + 1;
    }
    current
}

fn parse_range(line: &str) -> (usize, usize) {
    let (lower, upper) = line.split_once('-').unwrap();
    (lower.parse().unwrap(), upper.parse().unwrap())
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
        assert_eq!(run(&sample_input, 9), (3, 2));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input, u32::MAX as usize), (23923783, 125));
    }
}
