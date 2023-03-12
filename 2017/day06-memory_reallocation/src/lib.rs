use std::collections::HashSet;

pub fn run(input: &str) -> (usize, usize) {
    let mut memory_banks: Vec<_> = input.split_whitespace().map(|i| i.parse::<isize>().unwrap()).collect();
    let mut known_states = HashSet::from([memory_banks.clone()]);
    let mut first = 1;
    redistribute(&mut memory_banks);
    while !known_states.contains(&memory_banks) {
        known_states.insert(memory_banks.clone());
        first += 1;
        redistribute(&mut memory_banks);
        // eprintln!("{:#?}", &memory_banks);
    }
    let target_state = memory_banks.clone();
    let mut second = 1;
    redistribute(&mut memory_banks);
    while memory_banks != target_state {
        redistribute(&mut memory_banks);
        second += 1; 
    }
    (first, second)
}

fn redistribute(memory_banks: &mut [isize]) {
    let (max_idx, max_val) = memory_banks.iter().cloned().enumerate().max_by(|a, b| match a.1.cmp(&b.1) {
                                                                                    std::cmp::Ordering::Equal => b.0.cmp(&a.0),
                                                                                    o => o,
                                                                                }).unwrap();
    memory_banks[max_idx] = 0;
    (1..=max_val as usize).for_each(|idx| memory_banks[(max_idx + idx) % memory_banks.len()] += 1);
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
        assert_eq!(run(&sample_input), (5, 4));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (3156, 1610));
    }
}
