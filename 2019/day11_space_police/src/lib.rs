use std::{collections::HashMap, isize};
use intcode_processor::intcode_processor::{Cpu, OutputState};

enum Direction { Up, Left, Down, Right }

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }

    fn x(&self) -> isize {
        match self {
            Self::Left => -1,
            Self::Right => 1,
            _ => 0,
        }
    }

    fn y(&self) -> isize {
        match self {
            Self::Up => -1,
            Self::Down => 1,
            _ => 0,
        }
    }
}

pub fn run(input: &str) -> (usize, String) {
    let mut cpu_1 = Cpu::try_with_memory_from_str(input).unwrap();
    let mut cpu_2 = cpu_1.clone();
    let mut panels_1 = HashMap::new();
    let mut panels_2 = HashMap::from([((0, 0), 1)]);
    paint(&mut cpu_1, &mut panels_1);
    paint(&mut cpu_2, &mut panels_2);
    let first = panels_1.len();
    let second = print(&panels_2);
    (first, second)
}

fn paint(cpu: &mut Cpu, panels: &mut HashMap<(isize, isize), isize>) {
    let mut position = (0, 0);
    let mut direction = Direction::Up;
    loop {
        cpu.set_input(*panels.get(&position).unwrap_or(&0));
        if let OutputState::Output(colour) = cpu.run() {
            panels.insert(position, colour);
        } else {
            return;
        }
        match cpu.run() {
            OutputState::Output(0) => direction = direction.turn_left(),
            OutputState::Output(1) => direction = direction.turn_right(),
            _ => return,
        }
        position.0 += direction.x();
        position.1 += direction.y();
    }
}

fn print(panels: &HashMap<(isize, isize), isize>) -> String {
    let x_min = *panels.iter().map(|((x, _y), _colour)| x).min().unwrap();
    let x_max = *panels.iter().map(|((x, _y), _colour)| x).max().unwrap();
    let y_min = *panels.iter().map(|((_x, y), _colour)| y).min().unwrap();
    let y_max = *panels.iter().map(|((_x, y), _colour)| y).max().unwrap();

    (y_min..=y_max).map(|y| (x_min..=x_max).map(|x| match panels.get(&(x, y)) { Some(1) => '#', _ => ' ', }).chain(['\n'].into_iter()).collect::<String>()).collect()
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
        let expected = r#"
 #### #    #### ###  #  #   ## ###   ##    
    # #    #    #  # # #     # #  # #  #   
   #  #    ###  ###  ##      # #  # #  #   
  #   #    #    #  # # #     # ###  ####   
 #    #    #    #  # # #  #  # # #  #  #   
 #### #### #### ###  #  #  ##  #  # #  #   
"#;
        assert_eq!(run(&challenge_input), (2539, expected[1..].to_string()));
    }
}
