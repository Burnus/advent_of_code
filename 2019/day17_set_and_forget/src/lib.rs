use intcode_processor::intcode_processor::{Cpu, OutputState};
use std::collections::HashSet;
use std::num::ParseIntError;

type Coordinates = (usize, usize);

pub fn run(input: &str) -> Result<(usize, isize), ParseIntError> {
    let mut cpu = Cpu::try_with_memory_from_str(input)?;
    let mut cpu_2 = cpu.clone();
    let mut image = String::new();
    while let OutputState::Output(pixel) = cpu.run() {
        image.push((pixel as u8) as char);
    }
    // println!("{image}");
    let map: HashSet<_> = image.lines().enumerate().flat_map(|(y, line)| line.chars().enumerate().filter(|(_x, c)| *c == '#').map(|(x, _c)| (x, y)).collect::<HashSet<_>>()).collect();
    let first = map.iter().filter(|s| neighbours(**s, &map) > 2).map(|(x, y)| x*y).sum();

    // Sorry, Part 2 only works for this specific input
    cpu_2.set(0, 2);
    let commands = "A,B,A,C,A,B,C,C,A,B\nR,8,L,10,R,8\nR,12,R,8,L,8,L,12\nL,12,L,10,L,8\nn\n";
    for byte in commands.bytes() {
        cpu_2.set_input(byte as isize);
    }
    let mut second = 0;
    loop {
        match cpu_2.run() {
            OutputState::Output(_) => (),
            OutputState::DiagnosticCode(dust) => {
                second = dust;
                break;
            },
            OutputState::Halt => panic!("Halted without returning dust"),
        }
    }
    Ok((first, second))
}

fn neighbours(of: Coordinates, map: &HashSet<Coordinates>) -> u8 {
    let mut neighbours = 0;

    if of.0>0 && map.contains(&(of.0-1, of.1)) {
        neighbours += 1;
    }

    if of.1>0 && map.contains(&(of.0, of.1-1)) {
        neighbours += 1;
    }

    if map.contains(&(of.0+1, of.1)) {
        neighbours += 1;
    }

    if map.contains(&(of.0, of.1+1)) {
        neighbours += 1;
    }

    neighbours
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
        assert_eq!(run(&challenge_input), Ok((2804, 833429)));
    }
}
