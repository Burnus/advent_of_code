use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidDirection(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDirection(c) => write!(f, "Trying to parse invalid character {c} into Direction"),
        }
    }
}

#[derive(PartialEq)]
enum Shape { 
    Minus,
    Plus, 
    L, 
    Pipe, 
    Square,
}

impl <T: Into<usize>> From <T> for Shape {
    fn from(number: T) -> Self {
        match number.into() % 5 {
            0 => Shape::Minus,
            1 => Shape::Plus,
            2 => Shape::L,
            3 => Shape::Pipe,
            4 => Shape::Square,
            _ => unreachable!("number%5 can only ever be one of the values above"),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Direction { Left, Right, Down }

impl TryFrom<char> for Direction {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            c => Err(Self::Error::InvalidDirection(c)),
        }
    }
}

#[derive(PartialEq)]
enum State { Falling, Resting }

#[derive(PartialEq, Copy, Clone, Debug)]
struct Position {
    x: u8,
    y: usize,
}

struct Block {
    shape: Shape,
    bl_position: Position,
}

impl Block {
    fn spawn(shape: Shape, arena: &mut PlayArea) -> Self {
        let block = Self {
            shape,
            bl_position: Position {
                x: 2,
                y: (arena.max_y + 4) as usize,
            },
        };
        for position in &block.all_positions() {
            arena.occupy(position);
        }
        arena.max_y += 4;
        block
    }

    fn old_positions_by_shifting(&self, direction: &Direction) -> Vec<Position> {
        let bl = self.bl_position;
        match direction {
            Direction::Down => match self.shape {
                    Shape::Minus => (0..4).map(|x_shift| Position {
                                                        x: bl.x + x_shift,
                                                        y: bl.y, })
                                        .collect(),
                    Shape::Pipe => vec![Position { x: bl.x, y: bl.y + 3 }],
                    Shape::L => vec![
                                        bl,
                                        Position { x: bl.x + 1, y: bl.y },
                                        Position { x: bl.x + 2, y: bl.y + 2 },
                                    ],
                    Shape::Square => (0..2).map(|x_shift| Position {
                                                            x: bl.x + x_shift,
                                                            y: bl.y + 1, })
                                        .collect(),
                    Shape::Plus => vec![
                                        Position { x: bl.x, y: bl.y + 1 },
                                        Position { x: bl.x + 1, y: bl.y + 2, },
                                        Position { x: bl.x + 2, y: bl.y + 1 },
                                    ],
                },
            Direction::Left => match self.shape {
                    Shape::Minus => vec![Position { x: bl.x + 3, y: bl.y }],
                    Shape::Pipe => (0..4).map(|y_shift| Position {
                                                            x: bl.x,
                                                            y: bl.y + y_shift, })
                                        .collect(),
                    Shape::L => (0..3).map(|y_shift| Position {
                                                        x: bl.x + 2,
                                                        y: bl.y + y_shift, })
                                        .collect(),
                    Shape::Square => (0..2).map(|y_shift| Position {
                                                            x: bl.x + 1,
                                                            y: bl.y + y_shift, })
                                            .collect(),
                    Shape::Plus => vec![
                                        Position { x: bl.x + 1, y: bl.y },
                                        Position { x: bl.x + 2, y: bl.y + 1 },
                                        Position { x: bl.x + 1, y: bl.y + 2 },
                                    ],
                },
            Direction::Right => match self.shape {
                    Shape::Minus => vec![bl],
                    Shape::Pipe => (0..4).map(|y_shift| Position {
                                                            x: bl.x,
                                                            y: bl.y + y_shift, })
                                        .collect(),
                    Shape::L => vec![
                                    bl,
                                    Position { x: bl.x + 2, y: bl.y + 1 },
                                    Position { x: bl.x + 2, y: bl.y + 2 },
                                ],
                    Shape::Square => (0..2).map(|y_shift| Position {
                                                            x: bl.x,
                                                            y: bl.y + y_shift, })
                                            .collect(),
                    Shape::Plus => vec![
                                        Position { x: bl.x + 1, y: bl.y, },
                                        Position { x: bl.x, y: bl.y + 1 },
                                        Position { x: bl.x + 1, y: bl.y + 2 },
                                    ],
                }
        }
    }

    fn new_positions_by_shifting(&self, direction: &Direction) -> Vec<Position> {
        let bl = self.bl_position;
        match direction {
            Direction::Down => match self.shape {
                    Shape::Minus => (0..4).map(|x_shift| Position {
                                                        x: bl.x + x_shift,
                                                        y: bl.y - 1, })
                                        .collect(),
                    Shape::Pipe => vec![Position { x: bl.x, y: bl.y - 1 }],
                    Shape::L => (0..3).map(|x_shift| Position {
                                                        x: bl.x + x_shift,
                                                        y: bl.y - 1, })
                                        .collect(),
                    Shape::Square => (0..2).map(|x_shift| Position {
                                                            x: bl.x + x_shift,
                                                            y: bl.y - 1, })
                                        .collect(),
                    Shape::Plus => vec![
                                        bl,
                                        Position { x: bl.x+1, y: bl.y-1, },
                                        Position { x: bl.x+2, y: bl.y },
                                    ],
                },
            Direction::Left => match self.shape {
                    Shape::Minus => vec![Position { x: bl.x-1, y: bl.y }],
                    Shape::Pipe => (0..4).map(|y_shift| Position {
                                                            x: bl.x - 1,
                                                            y: bl.y + y_shift, })
                                        .collect(),
                    Shape::L => vec![
                                    Position { x: bl.x - 1, y: bl.y },
                                    Position { x: bl.x + 1, y: bl.y + 1 },
                                    Position { x: bl.x + 1, y: bl.y + 2 },
                                ],
                    Shape::Square => (0..2).map(|y_shift| Position {
                                                            x: bl.x - 1,
                                                            y: bl.y + y_shift, })
                                            .collect(),
                    Shape::Plus => vec![
                                        bl,
                                        Position { x: bl.x - 1, y: bl.y + 1 },
                                        Position { x: bl.x, y: bl.y + 2 },
                                    ],
                },
            Direction::Right => match self.shape {
                    Shape::Minus => vec![Position { x: bl.x + 4, y: bl.y }],
                    Shape::Pipe => (0..4).map(|y_shift| Position {
                                                            x: bl.x + 1,
                                                            y: bl.y + y_shift, })
                                        .collect(),
                    Shape::L => (0..3).map(|y_shift| Position {
                                                        x: bl.x + 3,
                                                        y: bl.y + y_shift, })
                                        .collect(),
                    Shape::Square => (0..2).map(|y_shift| Position {
                                                            x: bl.x + 2,
                                                            y: bl.y + y_shift, })
                                            .collect(),
                    Shape::Plus => vec![
                                        Position { x: bl.x + 2, y: bl.y, },
                                        Position { x: bl.x + 3, y: bl.y + 1 },
                                        Position { x: bl.x + 2, y: bl.y + 2 },
                                    ],
                }

        }
    }

    fn all_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        match self.shape {
            Shape::Minus => {
                    for i in 0..4 {
                        positions.push(Position {
                            x: self.bl_position.x + i,
                            y: self.bl_position.y,
                        });
                    }
                },
            Shape::Pipe => {
                    for i in 0..4 {
                        positions.push(Position {
                            x: self.bl_position.x,
                            y: self.bl_position.y + i,
                        });
                    }
                },
            Shape::Square => {
                    for i in 0..2 {
                        for j in 0..2 {
                            positions.push(Position{
                                x: self.bl_position.x + i,
                                y: self.bl_position.y + j,
                            });
                        }
                    }
                },
            Shape::L => {
                    for i in 0..3 {
                        positions.push(Position { 
                            x: self.bl_position.x + i, 
                            y: self.bl_position.y,
                        });
                    }
                    for j in 1..3 {
                        positions.push(Position { 
                            x: self.bl_position.x + 2, 
                            y: self.bl_position.y + j,
                        });
                    }
                },
            Shape::Plus => {
                    positions.push(Position {
                        x: self.bl_position.x + 1,
                        y: self.bl_position.y,
                    });
                    for i in 0..3 {
                        positions.push(Position {
                            x: self.bl_position.x + i,
                            y: self.bl_position.y + 1,
                        });
                    }
                    positions.push(Position {
                        x: self.bl_position.x + 1,
                        y: self.bl_position.y + 2,
                    });
                }
        }
        positions
    }

    fn fall(&mut self, arena: &mut PlayArea) -> State {
        if self.bl_position.y == 0 {
            return State::Resting;
        }
        let new_positions = self.new_positions_by_shifting(&Direction::Down);
        if new_positions.iter().any(|pos| arena.is_occupied(pos)) {
            return State::Resting;
        }
        let old_positions = self.old_positions_by_shifting(&Direction::Down);
        self.bl_position.y -= 1;
        for position in &old_positions {
            arena.free(position);
        }
        for position in &new_positions {
            arena.occupy(position);
        }

        arena.max_y = (self.bl_position.y..=arena.max_y as usize)
            .filter(|&idx| arena.blocked_tiles[idx].iter().any(|&b| b)).max().unwrap() as isize;

        State::Falling
        
    }

    fn push(&mut self, arena: &mut PlayArea, direction: &Direction) {
        if *direction == Direction::Left && self.bl_position.x == 0 {
            return;
        }
        let new_positions = self.new_positions_by_shifting(direction);
        if new_positions.iter().any(|pos| pos.x > arena.max_x) {
            return;
        }

        if !new_positions.iter().any(|pos| arena.is_occupied(pos)) {
            let old_positions = self.old_positions_by_shifting(direction);
            self.bl_position.x = match direction {
                Direction::Left => self.bl_position.x - 1,
                Direction::Right => self.bl_position.x + 1,
                Direction::Down => panic!("Didn't expect to be pushed down"),
            };
            for position in &old_positions {
                arena.free(position);
            }
            for position in &new_positions {
                arena.occupy(position);
            }
        }
    }
}

struct PlayArea {
    blocked_tiles: [[bool; 7]; 7_000],
    max_x: u8,
    max_y: isize,
}

impl PlayArea {
    fn is_occupied(&self, position: &Position) -> bool {
        self.blocked_tiles[position.y][position.x as usize]
    }

    fn occupy(&mut self, coordinates: &Position) {
        self.blocked_tiles[coordinates.y][coordinates.x as usize] = true;
    }

    fn free(&mut self, coordinates: &Position) {
        self.blocked_tiles[coordinates.y][coordinates.x as usize] = false;
    }

    fn new() -> Self {
        Self {
            blocked_tiles: [[false; 7]; 7_000],
            max_x: 6,
            max_y: -1,
        }
    }
}


fn solve_with_pattern(target: usize, directions: &[Direction]) -> usize {
    let mut results: Vec<(usize, usize, usize)> = Vec::new();
    let mut arena = PlayArea::new();
    let mut direction_index = 0;

    for i in 0_usize.. {
        let mut block = Block::spawn(Shape::from(i % 5), &mut arena);
        loop {
            block.push(&mut arena, &directions[direction_index]);
            direction_index += 1;
            direction_index %= directions.len();
            if block.fall(&mut arena) == State::Resting {
                if i + 1 == target {
                    return arena.max_y as usize + 1;
                }
                let state = (i, direction_index, arena.max_y as usize);
                let old_results: Vec<(usize, usize, usize)> = results.iter().filter(|(old_i, old_direction, _)| old_i % 5 == i % 5 && *old_direction == direction_index).cloned().collect();
                if old_results.len() > 1 {
                    let period = i - old_results[1].0;
                    let period_growth = arena.max_y as usize - old_results[1].2;
                    let offset = results[target % period].2;
                    return (target/period) * period_growth + offset;
                } else {
                    results.push(state);
                    break;
                }
            }
        }
    }
    0
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let directions = input.chars().map(Direction::try_from).collect::<Result<Vec<_>, _>>()?;
    let first = solve_with_pattern(2022, &directions);
    let second = solve_with_pattern(1_000_000_000_000, &directions);
    Ok((first, second))
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
        assert_eq!(run(&sample_input), Ok((3069, 1514285714288)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((3219, 1582758620701)));
    }
}
