use std::collections::HashMap;

struct Instruction(HashMap<char, Vec<char>>);

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut instructions: HashMap<char, Vec<char>> = HashMap::new();
        value.lines().for_each(|line| {
            let words: Vec<_> = line.split_whitespace().collect();
            assert_eq!(words.len(), 10);
            let prerequisite = words[1].chars().next().unwrap();
            let this = words[7].chars().next().unwrap();
            instructions.entry(this).and_modify(|step| step.push(prerequisite)).or_insert(Vec::from([prerequisite]));
            if instructions.get(&prerequisite).is_none() {
                instructions.insert(prerequisite, Vec::new());
            }
        });
        Instruction(instructions)
    }
}
pub fn run(input: &str, workers: usize, additional_time: u8) -> (String, usize) {
    let instructions = input.into();
    let first = assembly_order(&instructions);
    let second = par_assambly_time(&instructions, workers, additional_time);
    (first, second)
}

fn par_assambly_time(instructions: &Instruction, workers: usize, additional_time: u8) -> usize {
    let mut remaining = instructions.0.clone();
    let mut steps = String::new();
    let mut workers = vec![(0, None); workers];
    let mut todo = remaining.len();
    let mut time_passed = 0;
    while todo > 0 {
        time_passed += 1;
        for w in workers.iter_mut() {
            match w {
                (0, None) => (),
                (1, Some(x)) => {
                        steps.push(*x);
                        todo -= 1;
                        *w = (0, None);
                    },
                _ => w.0 -= 1,
            }
        }
        let mut available: Vec<_> = remaining.iter().filter(|(_step, prereq)| prereq.iter().all(|c| steps.contains(*c))).map(|(step, _prereq)| *step).collect();
        available.sort_by(|a, b|b.cmp(a));
        for w in workers.iter_mut().filter(|w| **w == (0, None)) {
            if let Some(next) = available.pop() {
                let time = additional_time + next as u8 - b'@';
                *w = (time, Some(next)); 
                remaining.remove(&next);
            }
        }
    }
    time_passed - 1
}

fn assembly_order(instructions: &Instruction) -> String {
    let mut remaining = instructions.0.clone();
    let mut steps = String::new();
    while !remaining.is_empty() {
        let next = remaining.iter().filter(|(_step, prereq)| prereq.iter().all(|c| steps.contains(*c))).map(|(step, _prereq)| *step).min().unwrap();
        steps.push(next);
        remaining.remove(&next);
    }

    steps
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
        assert_eq!(run(&sample_input, 2, 0), ("CABDFE".to_string(), 15));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input, 5, 60), ("JNOIKSYABEQRUVWXGTZFDMHLPC".to_string(), 1099));
    }
}
