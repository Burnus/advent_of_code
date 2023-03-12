enum Direction { Up, Down, Left, Right }

struct Position {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Position {
    fn mov(&mut self) {
        (self.x, self.y) = match self.dir {
            Direction::Up => (self.x, self.y-1),
            Direction::Down => (self.x, self.y+1),
            Direction::Left => (self.x-1, self.y),
            Direction::Right => (self.x+1, self.y),
        }
    }
}

pub fn run(input: &str) -> (String, usize) {
    let mut current = Position {
        x: input.lines().next().unwrap().bytes().position(|b| b == b'|').unwrap(),
        y: 0,
        dir: Direction::Down,
    };
    let mut first = String::new();
    for second in 1.. {
        current.mov();
        match input.lines().nth(current.y).unwrap().as_bytes().get(current.x).unwrap() {
            b' ' => return(first, second),
            b'|' | b'-' => (),
            c @ b'A'..=b'Z' => first.push(*c as char),
            b'+' => match current.dir {
                    Direction::Up | Direction::Down => if current.x == 0 || input.lines().nth(current.y).unwrap().as_bytes().get(current.x-1) == Some(&b' ') {
                            current.dir = Direction::Right;
                        } else {
                            current.dir = Direction::Left;
                        },
                    Direction::Left | Direction::Right => if current.y == 0 || input.lines().nth(current.y-1).unwrap().as_bytes().get(current.x) == Some(&b' ') {
                            current.dir = Direction::Down;
                        } else {
                            current.dir = Direction::Up;
                        },
                },
            u => panic!("Reached an unrecognized input character: {}", u),
        }
    }
    unreachable!("The loop always executes");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..])
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), ("ABCDEF".to_string(), 38));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), ("LXWCKGRAOY".to_string(), 17302));
    }
}
