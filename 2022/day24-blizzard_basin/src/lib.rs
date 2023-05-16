use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    StartNotFound,
    UnexpectedChar(char)
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StartNotFound => write!(f, "Unable to find a starting blizzard in row 0"),
            Self::UnexpectedChar(c) => write!(f, "Trying to parse unexpected character {c} into map"),
        }
    }
}

#[derive(PartialEq)]
enum Direction { Up, Down, Left, Right }

impl Direction {
    fn offset(&self) -> (i8, i8) {
        match self {
            Direction::Up =>    (0,-1),
            Direction::Down =>  (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(PartialEq)]
enum Tile {
    Wall,
    Blizards(u8),
}

impl Tile {
    fn increase_blizzards(&self) -> Self {
        if let Tile::Blizards(count) = self {
            Tile::Blizards(*count + 1)
        } else {
            panic!("Increase Blizards was called on a wall.");
        }
    }

    fn decrease_blizzards(&self) -> Self {
        if let Tile::Blizards(count) = self {
            Tile::Blizards(*count - 1)
        } else {
            panic!("Decrease Blizards was called on a wall.");
        }
    }
}

struct Blizard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Blizard {
    fn go(&mut self, map: &mut [Vec<Tile>], max_x: usize, max_y: usize) {
        let offset = self.direction.offset();
        map[self.y][self.x] = map[self.y][self.x].decrease_blizzards();
        let mut new_x = (self.x as i8 + offset.0) as usize;
        let mut new_y = (self.y as i8 + offset.1) as usize;
        if new_x == 0 {
            new_x = max_x;
        } else if new_x > max_x {
            new_x = 1;
        } else if new_y == 0 {
            new_y = max_y;
        } else if new_y > max_y {
            new_y = 1;
        }
        self.x = new_x;
        self.y = new_y;
        map[self.y][self.x] = map[new_y][new_x].increase_blizzards();
    }
}

fn try_parse(input: &str) -> Result<(Vec<Vec<Tile>>, Vec<Blizard>), ParseError> {
    let mut map = Vec::new();
    let mut blizzards = Vec::new();
    for (y, line) in input.lines().enumerate() {
            let mut this_line = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match &c {
                    '.' => this_line.push(Tile::Blizards(0)),
                    '#' => this_line.push(Tile::Wall),
                    '<' => {
                            this_line.push(Tile::Blizards(1));
                            blizzards.push(Blizard { x, y, direction: Direction::Left });
                        },
                    '>' => {
                            this_line.push(Tile::Blizards(1));
                            blizzards.push(Blizard { x, y, direction: Direction::Right });
                        },
                    '^' => {
                            this_line.push(Tile::Blizards(1));
                            blizzards.push(Blizard { x, y, direction: Direction::Up });
                        },
                    'v' => {
                            this_line.push(Tile::Blizards(1));
                            blizzards.push(Blizard { x, y, direction: Direction::Down });
                        },
                    c => return Err(ParseError::UnexpectedChar(*c)),
                }
            }
            map.push(this_line);
        }
    Ok((map, blizzards))
}

fn get_neighbours((x, y): (usize, usize), max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![(x, y)];
    if x > 1 {
        neighbours.push((x-1, y));
    }
    if x < max_x {
        neighbours.push((x+1, y));
    }
    if y > 0 {
        neighbours.push((x, y-1));
    }
    // We need to include the last row, so we can go to the destination.
    if y <= max_y {
        neighbours.push((x, y+1))
    }

    neighbours
}

fn get_rounds_from_to(start: (usize, usize), destination: (usize, usize), map: &mut [Vec<Tile>], blizzards: &mut [Blizard]) -> usize {
    let max_x = map[0].len()-2;
    let max_y = map.len()-2;
    let mut positions_last_step = vec![start];
    let mut rounds = 0;

    'out: loop {
        rounds += 1;
        let mut positions_this_step = Vec::new();
        for blizzard in &mut *blizzards {
            blizzard.go(map, max_x, max_y);
        }
        for position in &positions_last_step {
            let next = get_neighbours(*position, max_x, max_y);
            for neighbour in next {
                if neighbour == destination {
                    break 'out;
                }
                if map[neighbour.1][neighbour.0] == Tile::Blizards(0) && !positions_this_step.contains(&neighbour) {
                    positions_this_step.push(neighbour);
                }
            }
        }
        positions_last_step = positions_this_step;
    }
    rounds
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let (mut map, mut blizzards) = try_parse(input)?;
    let start = (map[0].iter().position(|tile| *tile == Tile::Blizards(0)).ok_or(ParseError::StartNotFound)?, 0);
    let destination = (map[map.len()-1].iter().position(|tile| *tile == Tile::Blizards(0)).unwrap(), map.len()-1);

    let mut second = get_rounds_from_to(start, destination, &mut map, &mut blizzards);
    let first = second;
    second += get_rounds_from_to(destination, start, &mut map, &mut blizzards);
    second += get_rounds_from_to(start, destination, &mut map, &mut blizzards);
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
        assert_eq!(run(&sample_input), Ok((18, 54)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((277, 877)));
    }
}
