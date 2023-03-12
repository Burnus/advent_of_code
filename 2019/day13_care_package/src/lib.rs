use std::collections::HashMap;

use intcode_processor::intcode_processor::{Cpu, OutputState};

pub fn run(input: &str) -> (usize, isize) {
    let mut cpu = Cpu::try_with_memory_from_str(input).unwrap();
    let mut cpu_2 = cpu.clone();
    let mut screen = HashMap::new();
    let mut screen_2 = HashMap::new();
    while let OutputState::Output(x) = cpu.run() {
        if let OutputState::Output(y) = cpu.run() {
            if let OutputState::Output(tile) = cpu.run() {
                screen.insert((x, y), tile);
            }
        }
    }
    let first = screen.iter().filter(|(_coords, tile)| **tile == 2).count();

    cpu_2.set(0, 2);
    let second = play(&mut cpu_2, &mut screen_2);
    (first, second)
}

fn play(cpu: &mut Cpu, screen: &mut HashMap<(isize, isize), isize>) -> isize {
    let mut res = 0;
    let mut ball_pos = 0;
    let mut pad_pos = 0;
    while let OutputState::Output(x) = cpu.run() {
        if let OutputState::Output(y) = cpu.run() {
            if let OutputState::Output(tile) = cpu.run() {
                if x == -1 && y == 0 {
                    res = tile;
                } else {
                    if tile == 4 {
                        ball_pos = x;
                    } else if tile == 3 {
                        pad_pos = x;
                    }
                    screen.insert((x, y), tile);
                    cpu.reset_input((ball_pos - pad_pos).signum());
                }
            }
        }
    }
    res
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
        assert_eq!(run(&challenge_input), (333, 16539));
    }
}
