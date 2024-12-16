use core::fmt::Display;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InputMalformed,
    InvalidMapChar(char),
    InvalidSequenceChar(char),
    NoRobot
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputMalformed => write!(f, "Input must consist of the map, followed by an empty line, and the robot's movement sequence"),
            Self::InvalidMapChar(e) => write!(f, "Found invalid map tile {e}. Only '#', '.', 'O', and '@' are allowed."),
            Self::InvalidSequenceChar(e) => write!(f, "Found invalid movement sequence {e}. Only '<', 'v', '^', and '>' are allowed."),
            Self::NoRobot => write!(f, "No robot ('@') was found in the map"),
        }
    }
}

type Coordinates = (isize, isize);

struct Warehouse {
    walls: HashSet<Coordinates>,
    boxes: HashSet<Coordinates>,
    robot: Coordinates,
    widened: bool,
}

impl TryFrom<&str> for Warehouse {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut robot = None;

        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => _ = walls.insert((x as isize, y as isize)),
                    'O' => _ = boxes.insert((x as isize, y as isize)),
                    '@' => robot = Some((x as isize, y as isize)),
                    '.' => (),
                    e => return Err(Self::Error::InvalidMapChar(e)),
                }
            }
        }

        if let Some(robot) = robot {
            Ok(Self {
                walls,
                boxes,
                robot,
                widened: false,
            })
        } else {
            Err(Self::Error::NoRobot)
        }
    }
}

impl Warehouse {
    fn perform_sequence(&mut self, sequence: &[Coordinates]) {
        for &step in sequence {
            let next = (self.robot.0 + step.0, self.robot.1 + step.1);
            if self.walls.contains(&next) {
                continue;
            }
            if !self.boxes.contains(&next) && (!self.widened || !self.boxes.contains(&(next.0-1, next.1))) {
                self.robot = next;
                continue;
            }
            let to_push = if self.boxes.contains(&next) {
                next
            } else {
                (next.0-1, next.1)
            };
            if self.try_push(&[to_push], step) {
                self.robot = next;
            }
        }
    }

    fn try_push(&mut self, from: &[Coordinates], direction: Coordinates) -> bool {
        let mut pushed = Vec::new();
        for &start in from {
            let next = (start.0+direction.0, start.1 + direction.1);
            let next_l = if self.widened { (next.0-1, next.1 ) } else { next };
            let next_r = if self.widened { (next.0+1, next.1 ) } else { next };
            if self.walls.contains(&next) || self.walls.contains(&next_r) {
                return false;
            }
            [next_l, next, next_r].into_iter().filter_map(|b| self.boxes.get(&b)).for_each(|&b| 
                if !from.contains(&b) {
                    pushed.push(b)
                });
        }
        if pushed.is_empty() {
            from.iter().for_each(|&start| {
                self.boxes.remove(&start);
                self.boxes.insert((start.0 + direction.0, start.1 + direction.1));
            });
            true
        } else {
            // pushed.append(&mut from.to_vec());
            pushed.sort();
            pushed.dedup();
            if self.try_push(&pushed, direction) {
                from.iter().for_each(|&start| {
                    self.boxes.remove(&start);
                    self.boxes.insert((start.0 + direction.0, start.1 + direction.1));
                });
                true
            } else {
                false
            }
        }
    } 

    fn box_positions(&self) -> usize {
        self.boxes.iter().map(|(x, y)| (100 * y + x) as usize).sum()
    }

    fn widen(&self) -> Self {
        Self {
            walls: self.walls.iter().copied().flat_map(|(x, y)| [(2*x, y), (2*x+1, y)]).collect(),
            boxes: self.boxes.iter().copied().map(|(x, y)| (2*x, y)).collect(),
            robot: (2 * self.robot.0, self.robot.1),
            widened: true,
        }
    }
}

fn try_sequence_from(seq: &str) -> Result<Vec<Coordinates>, ParseError> {
    seq.lines().flat_map(|l| l.chars().map(|c| {
        match c {
            '<' => Ok((-1, 0)),
            'v' => Ok((0, 1)),
            '^' => Ok((0, -1)),
            '>' => Ok((1, 0)),
            e => Err(ParseError::InvalidSequenceChar(e)),
        }
    })).collect::<Result<Vec<Coordinates>, _>>()
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    if let Some((map, seq)) = input.split_once("\n\n") {
        let mut warehouse_1 = Warehouse::try_from(map)?;
        let sequence = try_sequence_from(seq)?;
        let mut warehouse_2 = warehouse_1.widen();
        warehouse_1.perform_sequence(&sequence);
        warehouse_2.perform_sequence(&sequence);
        let first = warehouse_1.box_positions();
        let second = warehouse_2.box_positions();
        Ok((first, second))
    } else {
        Err(ParseError::InputMalformed)
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
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), Ok((10092, 9021)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((1456590, 1489116)));
    }
}
