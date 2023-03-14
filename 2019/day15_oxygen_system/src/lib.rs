use std::{num::ParseIntError, collections::{HashMap, HashSet}};
use intcode_processor::intcode_processor::{Cpu, OutputState};

pub fn run(input: &str) -> Result<(usize, usize), ParseIntError> {
    let cpu = Cpu::try_with_memory_from_str(input)?;
    let (first, cpu_2) = find_oxygen(&cpu);
    let second = fill_map(&cpu_2);
    Ok((first, second))
}

fn fill_map(cpu: &Cpu) -> usize {
    let current = (0, 0);
    let mut area = HashSet::from([current]);
    let mut reached_last_step = Vec::from([(current, cpu.clone())]);
    for steps in 0.. {
        let mut reached_this_step = Vec::new();
        for (pos, this_cpu) in &reached_last_step {
            for direction in 1..=4 {
                let new_pos = get_coords(*pos, direction);
                if !area.contains(&new_pos) {
                    let mut new_cpu = this_cpu.clone();
                    new_cpu.set_input(direction);
                    if let OutputState::Output(state) = new_cpu.run() {
                        area.insert(new_pos);
                        match state {
                            0 => (),
                            1 | 2=> reached_this_step.push((new_pos, new_cpu)),
                            _ => panic!("Unexpected state returned: {state}"),
                        }
                    } else {
                        panic!("Cpu did not return an Output");
                    }
                }
            }
        }
        if reached_this_step.is_empty() {
            return steps;
        }
        std::mem::swap(&mut reached_this_step, &mut reached_last_step);
    }
    unreachable!("The loop always runs and can never exit except by returning");
}

fn find_oxygen(cpu: &Cpu) -> (usize, Cpu) {
    let current = (0, 0);
    let mut area = HashMap::from([(current, 1)]);
    let mut reached_last_step = Vec::from([(current, cpu.clone())]);
    for steps in 1.. {
        let mut reached_this_step = Vec::new();
        for (pos, this_cpu) in &reached_last_step {
            for direction in 1..=4 {
                let new_pos = get_coords(*pos, direction);
                if let std::collections::hash_map::Entry::Vacant(e) = area.entry(new_pos) {
                    let mut new_cpu = this_cpu.clone();
                    new_cpu.set_input(direction);
                    if let OutputState::Output(state) = new_cpu.run() {
                        e.insert(state);
                        match state {
                            0 => (),
                            1 => reached_this_step.push((new_pos, new_cpu)),
                            2 => return (steps, new_cpu),
                            _ => panic!("Unexpected state returned: {state}"),
                        }
                    } else {
                        panic!("Cpu did not return an Output");
                    }
                }
            }
        }
        std::mem::swap(&mut reached_this_step, &mut reached_last_step);
    }
    unreachable!("The loop always runs and can never exit except by returning");
}

fn get_coords(old: (isize, isize), direction: isize) -> (isize, isize) {
    match direction {
        1 => (old.0, old.1 - 1), // N
        2 => (old.0, old.1 + 1), // S
        3 => (old.0 - 1, old.1), // W
        4 => (old.0 + 1, old.1), // E
        _ => panic!("Unexpected direction: {direction}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((220, 334)));
    }
}
