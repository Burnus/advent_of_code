use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    InputMalformed(&'a str),
    InvalidChar(char),
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputMalformed(v) => write!(f, "Unable to split Input into Map and Instructions. Input should consist of 2 parts, separated by an empty line:\n{v}"),
            Self::InvalidChar(c) => write!(f, "Invalid Character {c} encountered"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction { 
    E = 0,
    S = 1,
    W = 2,
    N = 3,
}

#[derive(Clone, Copy, PartialEq)]
enum Wrapping { Flat, Cube(isize) }

#[derive(PartialEq)]
enum Walkability { Free, Obstructed, Void }

enum Instruction { Go(usize), Turn(char) }

#[derive(Clone, Copy)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    fn next(&self, original_direction: Direction, map: &[Vec<Walkability>], wrapping: Wrapping) -> Option<(Coordinate, Direction)> {
        let mut new_col = self.col as isize;
        let mut new_row = self.row as isize;
        let mut direction = original_direction;
        loop {
            let (row_diff, col_diff) = match direction {
                Direction::N => (-1, 0),
                Direction::S => (1, 0),
                Direction::W => (0, -1),
                Direction::E => (0, 1),
            };
            let max_row = map.len() as isize;
            let max_col = map[new_row as usize].len() as isize;
            let mut next_row = match (new_row, row_diff) {
                (0, -1) => max_row - 1,
                (max, 1) if max == max_row - 1 => 0,
                _ => new_row + row_diff,
                // _ => (new_row + row_diff) % max_row,
            };
            let mut next_col = match (new_col, col_diff) {
                (0, -1) => max_col - 1,
                (max, 1) if max == max_col - 1 => 0,
                _ => new_col + col_diff,
                //_ => (new_col + col_diff) % max_col,
            };
            if let Wrapping::Cube(side_length) = wrapping {
                match (next_row, next_col, direction) {
                    (r, c, d) if r == 4*side_length-1 && (side_length..2*side_length).contains(&c) && d == Direction::N => {
                        // 1 => 6
                        next_row = 2*side_length + c;
                        next_col = 0;
                        direction = Direction::E;
                    },
                    (r, c, d) if r == 4*side_length-1 && (2*side_length..3*side_length).contains(&c) && d == Direction::N => {
                        // 2 => 6
                        //next_row = 4*side_length-1; // already set
                        next_col = c - 2*side_length;
                        //direction = Direction::N; // already set
                    },
                    (r, c, d) if r == side_length && (2*side_length..3*side_length).contains(&c) && d == Direction::S => {
                        // 2 => 3
                        next_row = c - side_length;
                        next_col = 2*side_length-1;
                        direction = Direction::W;
                    },
                    (r, c, d) if r == 3*side_length && (side_length..2*side_length).contains(&c) && d == Direction::S => {
                        // 5 => 6
                        next_row = c + 2*side_length;
                        next_col = side_length-1;
                        direction = Direction::W;
                    },
                    (r, c, d) if (0..side_length).contains(&r) && c == 0 && d == Direction::E => {
                        // 2 => 5
                        next_row = 3*side_length - (r+1);
                        next_col = 2*side_length-1;
                        direction = Direction::W;
                    },
                    (r, c, d) if (0..side_length).contains(&r) && c == side_length-1 && d == Direction::W => {
                        // 1 => 4
                        next_row = 3*side_length - (r+1);
                        next_col = 0;
                        direction = Direction::E;
                    },
                    (r, c, d) if (side_length..2*side_length).contains(&r) && c == side_length-1 && d == Direction::W => {
                        // 3 => 4
                        next_row = 2*side_length;
                        next_col = r - side_length;
                        direction = Direction::S;
                    },
                    (r, c, d) if (side_length..2*side_length).contains(&r) && c == 0 && d == Direction::E => {
                        // 3 => 2
                        next_row = side_length-1;
                        next_col = r + side_length;
                        direction = Direction::N;
                    },
                    (r, c, d) if r == 2*side_length-1 && (0..side_length).contains(&c) && d == Direction::N => {
                        // 4 => 3
                        next_row = side_length + c;
                        next_col = side_length;
                        direction = Direction::E;
                    },
                    (r, c, d) if (2*side_length..3*side_length).contains(&r) && c == 2*side_length-1 && d == Direction::W => {
                        // 4 => 1
                        next_row = 3*side_length - (r+1);
                        next_col = side_length;
                        direction = Direction::E;
                    },
                    (r, c, d) if (2*side_length..3*side_length).contains(&r) && c == 0 && d == Direction::E => {
                        // 5 => 2
                        next_row = 3*side_length - (r+1);
                        next_col = 3*side_length-1;
                        direction = Direction::W;
                    },
                    (r, c, d) if (3*side_length..4*side_length).contains(&r) && c == side_length-1 && d == Direction::W => {
                        // 6 => 1
                        next_row = 0;
                        next_col = r - 2*side_length;
                        direction = Direction::S;
                    },
                    (r, c, d) if (3*side_length..4*side_length).contains(&r) && c == 0 && d == Direction::E => {
                        // 6 => 5
                        next_row = 3*side_length-1;
                        next_col = r - 2*side_length;
                        direction = Direction::N;
                    },
                    (r, c, d) if r == 0 && (0..side_length).contains(&c) && d == Direction::S => {
                        // 6 => 2
                        next_row = 0; // already set
                        next_col = c + 2*side_length;
                        //direction = Direction::S; // already set
                    },
                    _ => (),
                }
            } else if next_col as usize >= map[next_row as usize].len() {
                new_row = next_row;
                continue;
            }
            match map[next_row as usize][next_col as usize] {
                Walkability::Obstructed => { return None },
                Walkability::Free => { return Some((Coordinate { row: next_row as usize, col: next_col as usize }, direction)); },
                Walkability::Void => { new_col = next_col; new_row = next_row; },
            } 
        }
    }
}

struct Position {
    coordinate: Coordinate,
    facing: Direction,
}

impl Position {
    fn turn(&mut self, turning: char) {
        self.facing = match turning {
            'L' => match self.facing {
                Direction::N => Direction::W,
                Direction::W => Direction::S,
                Direction::S => Direction::E,
                Direction::E => Direction::N,
            },
            'R' => match self.facing {
                Direction::N => Direction::E,
                Direction::W => Direction::N,
                Direction::S => Direction::W,
                Direction::E => Direction::S,
            },
            _ => panic!("Unknown Turning Instruction: {turning}"),
        };
    }

    fn go(&mut self, steps: usize, map: &[Vec<Walkability>], wrapping: Wrapping) {
        for _ in 0..steps {
            if let Some((new_coordinate, new_direction)) = self.coordinate.next(self.facing, map, wrapping) {
                self.coordinate = new_coordinate;
                self.facing = new_direction;
            } else {
                break;
            }
        }
    }

    fn follow_instruction(&mut self, instruction: &Instruction, map: &[Vec<Walkability>], wrapping: Wrapping) {
        match instruction {
            Instruction::Go(distance) => self.go(*distance, map, wrapping),
            Instruction::Turn(to) => self.turn(*to),
        }
    }
}

fn try_parse_map(string: &str) -> Result<Vec<Vec<Walkability>>, ParseError> {
    string.lines()
        .map(|line| line.chars()
                .map(|c| match c {
                        ' ' => Ok(Walkability::Void),
                        '.' => Ok(Walkability::Free),
                        '#' => Ok(Walkability::Obstructed),
                        _ => Err(ParseError::InvalidChar(c)),
                    })
                .collect())
        .collect::<Result<Vec<_>, _>>()
}

fn try_parse_instructions(line: &str) -> Result<Vec<Instruction>, ParseError> {
    let mut instructions = Vec::new();
    let mut distance = 0_usize;
    for c in line.chars() {
        if let Some(d) = c.to_digit(10) {
            distance *= 10;
            distance += d as usize;
        } else if ['L', 'R'].contains(&c) {
            if distance > 0 {
                instructions.push(Instruction::Go(distance));
                distance = 0;
            }
            instructions.push(Instruction::Turn(c));
        } else if c == '\n' {
            continue;
        } else {
            return Err(ParseError::InvalidChar(c));
        }
    }
    if distance > 0 {
        instructions.push(Instruction::Go(distance));
    }
    Ok(instructions)
}

// fn read_file(input: &str) -> (Vec<Vec<Walkability>>, Vec<Instruction>) {
//     let (map_str, instructions_str) = components.split_once("\n\n").unwrap();
//     (parse_map(map_str), parse_instructions(instructions_str))
// }

fn get_password(map: &[Vec<Walkability>], instructions: &[Instruction], wrapping: Wrapping) -> usize {
    let mut position = Position {
        coordinate: Coordinate {
            row: 0,
            col: map[0].iter().position(|w| *w == Walkability::Free).unwrap(),
        },
        facing: Direction::E,
    };

    for instruction in instructions {
        position.follow_instruction(instruction, map, wrapping);
    }

    (position.coordinate.row + 1) * 1000 + (position.coordinate.col + 1) * 4 + position.facing as usize
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let (map, instructions) = input.split_once("\n\n").ok_or(ParseError::InputMalformed(input))?;
    let map = try_parse_map(map)?;
    let instructions = try_parse_instructions(instructions)?;
    let first = get_password(&map, &instructions, Wrapping::Flat);
    let side_length = (map.iter().map(|i| i.iter().filter(|&w| *w != Walkability::Void).count()).sum::<usize>() as f64 / 6.0).sqrt() as isize;
    let second = get_password(&map, &instructions, Wrapping::Cube(side_length));
    Ok((first, second))
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
        let (map, instructions) = &sample_input[..].split_once("\n\n").unwrap();
        let map = try_parse_map(map).unwrap();
        let instructions = try_parse_instructions(instructions).unwrap();
        assert_eq!(get_password(&map, &instructions, Wrapping::Flat), 6032);
        assert_eq!((map.iter().map(|i| i.iter().filter(|&w| *w != Walkability::Void).count()).sum::<usize>() as f64 / 6.0).sqrt() as isize, 4);
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((58248, 179091)));
    }
}
