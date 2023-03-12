use std::{collections::HashMap, ops::{Add, AddAssign}};

#[derive(PartialEq, Eq, Clone)]
enum State { Weakened, Infected, Flagged }

#[derive(Clone)]
enum Direction { Up, Down, Left, Right }

impl Direction {
    fn turn_left(&mut self) {
        *self = match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        };
    }

    fn turn_right(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        };
    }

    fn reverse(&mut self) {
        *self = match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn position_offset(&self) -> Coordinates {
        match self {
            Self::Up => Coordinates(-1, 0),
            Self::Down => Coordinates(1, 0),
            Self::Left => Coordinates(0, -1),
            Self::Right => Coordinates(0, 1),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinates(isize, isize);

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Coordinates {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[derive(Clone)]
struct Carrier {
    position: Coordinates,
    direction: Direction,
    infections: usize
}

impl Carrier {
    fn burst(&mut self, grid: &mut HashMap<Coordinates, State>, version: u8) {
        match grid.get(&self.position) {
            None => {
                    self.direction.turn_left();
                    match version {
                        1 => { grid.insert(self.position, State::Infected); self.infections += 1; },
                        _ => { grid.insert(self.position, State::Weakened); },
                    };
                },
            Some(State::Weakened) => {
                    grid.insert(self.position, State::Infected);
                    self.infections += 1;
                },
            Some(State::Infected) => {
                    self.direction.turn_right();
                    match version { 
                        1 => grid.remove(&self.position),
                        _ => grid.insert(self.position, State::Flagged),
                    };
                },
            Some(State::Flagged) => {
                    self.direction.reverse();
                    grid.remove(&self.position);
                },
        }

        self.position += self.direction.position_offset();
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let mut infected: HashMap<Coordinates, State> = input.lines().enumerate().flat_map(|(y, line)|
                                                    line.chars().enumerate().filter(|(_x, c)| *c == '#').map(|(x, _)| (Coordinates(y as isize, x as isize), State::Infected)).collect::<HashMap<Coordinates, State>>()
                                                ).collect();
    let mut carrier = Carrier {
        position: Coordinates( (input.lines().count()/2) as isize, (input.lines().next().unwrap().len()/2) as isize ),
        direction: Direction::Up,
        infections: 0,
    };
    let mut infected_v2 = infected.clone();
    let mut carrier_v2 = carrier.clone();

    for _ in 0..10_000 {
        carrier.burst(&mut infected, 1);
    }
    for _ in 0..10_000_000 {
        carrier_v2.burst(&mut infected_v2, 2);
    }
    let first = carrier.infections;
    let second = carrier_v2.infections;
    (first, second)
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
        assert_eq!(run(&sample_input), (5587, 2511944));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (5261, 2511927));
    }
}
