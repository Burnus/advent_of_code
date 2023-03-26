use core::fmt::Display;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidCharacter(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCharacter(v) => write!(f, "Unexpected Character found: {v}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    LabelComponent(u16),
}

impl TryFrom<char> for Tile {
    type Error = ParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Open),
            '#' | ' ' => Ok(Self::Wall),
            l @ 'A'..='Z' => Ok(Self::LabelComponent(l as u16 - b'A' as u16)),
            e => Err(Self::Error::InvalidCharacter(e))
        }
    }
}

struct Maze {
    tiles: Vec<Vec<Tile>>,
    portals: HashMap<(usize, usize), (usize, usize)>
}

impl From<Vec<Vec<Tile>>> for Maze {
    fn from(value: Vec<Vec<Tile>>) -> Self {
        let mut tiles = value.to_vec();
        let mut labels = Vec::new();
        for row in 2..value.len()-2 {
            for col in 2..value[row].len()-2 {
                match value[row][col] {
                    Tile::LabelComponent(_) => tiles[row][col] = Tile::Wall,
                    Tile::Open => match ( (value[row-1][col], value[row-2][col]), (value[row][col-1], value[row][col-2]), (value[row][col+1], value[row][col+2]), (value[row+1][col], value[row+2][col]), ) {
                        ((Tile::LabelComponent(b), Tile::LabelComponent(a)), _, _, _) | (_, (Tile::LabelComponent(b), Tile::LabelComponent(a)), _, _) | (_, _, (Tile::LabelComponent(a), Tile::LabelComponent(b)), _) | (_, _, _, (Tile::LabelComponent(a), Tile::LabelComponent(b))) => {
                            labels.push((a*26+b, (row, col)));
                        },
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
        let mut portals = HashMap::new();
        labels.sort_by_key(|(label, _coords)| *label);
        labels.windows(2).for_each(|labels| {
            let a = labels[0];
            let b = labels[1];
            if a.0 == b.0 {
                portals.insert(a.1, b.1);
                portals.insert(b.1, a.1);
            }
        });
        portals.insert((0, 0), labels[0].1);
        portals.insert((1, 1), labels[labels.len()-1].1);

        Self { tiles, portals, }
    }
}

impl Maze {
    fn neighbours(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
            if let Some(dest) = self.portals.get(&(position.0, position.1)) {
                neighbours.push(*dest);
            }
        for offset in [(1, 0), (0, 1), (2, 1), (1, 2)] {
            let new_position = (position.0+offset.0-1, position.1+offset.1-1);
            if self.tiles[new_position.0][new_position.1] == Tile::Open {
                neighbours.push(new_position);
            }
        }

        neighbours
    }

    fn recursive_neighbours(&self, position: (usize, usize), level: usize) -> Vec<((usize, usize), usize)> {
        let mut neighbours = Vec::new();
            if let Some(dest) = self.portals.get(&(position.0, position.1)) {
                if position.0 == 2 || position.1 == 2 || position.0 == self.tiles.len()-3 || position.1 == self.tiles[position.0].len()-3 {
                    if level > 0 {
                        neighbours.push((*dest, level-1));
                    }
                } else {
                    neighbours.push((*dest, level+1));
                }
            }
        for offset in [(1, 0), (0, 1), (2, 1), (1, 2)] {
            let new_position = (position.0+offset.0-1, position.1+offset.1-1);
            if self.tiles[new_position.0][new_position.1] == Tile::Open {
                neighbours.push((new_position, level));
            }
        }

        neighbours
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let tiles: Vec<_> = input.lines().map(|line| line.chars().map(Tile::try_from).collect::<Result<Vec<_>, _>>()).collect::<Result<Vec<Vec<Tile>>, _>>()?;
    let maze = Maze::from(tiles);
    let start = maze.portals.get(&(0, 0)).unwrap();
    let goal = maze.portals.get(&(1, 1)).unwrap();
    let first = get_shortest_path(&maze, *start, *goal);
    let second = get_shortest_path_recursive(&maze, *start, *goal);
    Ok((first, second))
}

fn get_shortest_path(maze: &Maze, start: (usize, usize), goal: (usize, usize)) -> usize {
    let mut visited = HashSet::from([start]);
    let mut open_set = VecDeque::from([(start, 0)]);
    while let Some((position, dist)) = open_set.pop_front() {
        if position == goal {
            return dist;
        }
        for neighbour in maze.neighbours(position) {
            if !visited.contains(&neighbour) {
                open_set.push_back((neighbour, dist+1));
                visited.insert(neighbour);
            }
        }
    }
    panic!("All ways exhausted, but no solution found")
}

fn get_shortest_path_recursive(maze: &Maze, start: (usize, usize), goal: (usize, usize)) -> usize {
    let mut visited = HashSet::from([(start, 0)]);
    let mut open_set = VecDeque::from([((start, 0), 0)]);
    while let Some(((position, level), dist)) = open_set.pop_front() {
        if position == goal && level == 0 {
            return dist;
        }
        for neighbour in maze.recursive_neighbours(position, level) {
            if !visited.contains(&neighbour) {
                open_set.push_back((neighbour, dist+1));
                visited.insert(neighbour);
            }
        }
    }
    panic!("All ways exhausted, but no solution found")
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
        assert_eq!(run(&sample_input), Ok((77, 396)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((400, 4986)));
    }
}
