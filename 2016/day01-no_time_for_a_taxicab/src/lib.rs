use std::collections::HashSet;

enum Instruction {
    Left(isize),
    Right(isize),
}

impl Instruction {
    fn parse(input: &str) -> Self {
        match input.chars().next().unwrap() {
            'L' => Self::Left(input[1..].parse().unwrap()),
            'R' => Self::Right(input[1..].parse().unwrap()),
            _ => panic!("Unexpected instruction: {input}"),
        }
    }
}

enum Facing {
    N,
    E,
    S,
    W,
}

struct Position {
    x: isize,
    y: isize,
    direction: Facing,
}

impl Position {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Facing::N,
        }
    }

    fn turn_and_get_distance(&mut self, instruction: &Instruction) -> isize {
        self.direction = match instruction {
            Instruction::Left(_) => match self.direction {
                Facing::N => Facing::W,
                Facing::W => Facing::S,
                Facing::S => Facing::E,
                Facing::E => Facing::N,
            },
            Instruction::Right(_) => match self.direction {
                Facing::N => Facing::E,
                Facing::E => Facing::S,
                Facing::S => Facing::W,
                Facing::W => Facing::N,
            },
        };
        match instruction {
            Instruction::Left(d) | Instruction::Right(d) => *d,
        }

    }

    fn go(&mut self, distance: isize) {
        (self.x, self.y) = match self.direction {
            Facing::N => (self.x, self.y+distance),
            Facing::W => (self.x-distance, self.y),
            Facing::S => (self.x, self.y-distance),
            Facing::E => (self.x+distance, self.y),
        };
    }

    fn get_first_double(&mut self, instruction: &Instruction, visited: &mut HashSet<(isize, isize)>) -> Option<(isize, isize)> {
        let distance = self.turn_and_get_distance(instruction);
        for _ in 0..distance {
            self.go(1);
            if visited.contains(&(self.x, self.y)) {
                return Some((self.x, self.y));
            }
            visited.insert((self.x, self.y));
        }
        None
    }

    fn follow(&mut self, instruction: &Instruction) {
        let distance = self.turn_and_get_distance(instruction);
        self.go(distance);
    }
}

pub fn run(input: &str) -> (isize, isize) {
    let instructions: Vec<_> = input.trim().split(", ").map(Instruction::parse).collect();
    let mut me = Position::new();
    instructions.iter().for_each(|instruction| me.follow(instruction));
    let mut snd = Position::new();
    let mut visited = HashSet::from([(0, 0)]);
    let mut second = None;
    instructions.iter().for_each(|instruction| if second.is_none() {
        second = snd.get_first_double(instruction, &mut visited);
    });
    let first = me.x.abs() + me.y.abs();
    (first, second.unwrap().0.abs() + second.unwrap().1.abs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), (8, 4));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (246, 124));
    }
}
