use core::fmt::Display;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    ParseCharError(char),
    NoStartError,
    LineMalformed(&'a str),
    InvalidStartNeighbours(bool, bool, bool, bool),
    DoubleStartError,
}

impl From<ParseCharError> for ParseError<'_> {
    fn from(value: ParseCharError) -> Self {
        Self::ParseCharError(value.offending_char)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoubleStartError => write!(f, "Start tile encountered more than once"),
            Self::InvalidStartNeighbours(n, s, w, e) => write!(f, "Start tile has connections to more or less than 2 neighbours: North: {n}, South: {s}, West: {w}, East: {e}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::NoStartError => write!(f, "No start tile found"),
            Self::ParseCharError(c) => write!(f, "Unable to parse character: {c}"),
        }
    }
}

struct ParseCharError { offending_char: char, }

#[derive(PartialEq, Clone, Copy)]
#[repr(u8)]
enum Pipe {
    NorthSouth, // |
    EastWest,   // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Ground,     // .
    Start,      // S
}

impl TryFrom<char> for Pipe {
    type Error = ParseCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::NorthSouth),
            '-' => Ok(Self::EastWest),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            '7' => Ok(Self::SouthWest),
            'F' => Ok(Self::SouthEast),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            v => Err(ParseCharError{ offending_char: v }),
        }
    }
}

impl Pipe {
    fn get_neighbours(&self) -> (Pipe, Pipe) {
        match self {
            Self::NorthSouth => (Self::NorthSouth, Self::NorthSouth),
            Self::NorthWest | Self::NorthEast | Self::SouthWest | Self::SouthEast => (Self::NorthSouth, Self::EastWest),
            Self::EastWest => (Self::EastWest, Self::EastWest),
            _ => (Self::Ground, Self::Ground),
        }
    }

    fn get_neighbour_diffs(&self) -> ((usize, usize), (usize, usize)) {
        match self {
            Self::NorthSouth => ((0,1), (2,1)),
            Self::NorthWest => ((0,1), (1,0)),
            Self::NorthEast => ((0,1), (1,2)),
            Self::EastWest => ((1,0), (1,2)),
            Self::SouthWest => ((2,1), (1,0)),
            Self::SouthEast => ((2,1), (1,2)),
            _ => ((1,1), (1,1)),
        }
    }
}

fn try_parse_maze(input: &str) -> Result<(HashMap<(usize, usize), Pipe>, (usize, usize)), ParseError> {
    let mut start = None;
    let mut maze = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pipe = Pipe::try_from(c)?;
            match pipe {
                Pipe::Start => if start.is_none() {
                        start = Some((2*y+1, 2*x+1));
                    } else {
                        return Err(ParseError::DoubleStartError);
                    },
                Pipe::Ground => (),
                p => {
                        maze.insert((2*y+1, 2*x+1), p);
                        if y%2 == x%2 {
                            let ((dy1, dx1), (dy2, dx2)) = p.get_neighbour_diffs();
                            let (n1, n2) = p.get_neighbours();
                            maze.insert((2*y+dy1, 2*x+dx1), n1);
                            maze.insert((2*y+dy2, 2*x+dx2), n2);
                        }
                    },
            }
        }
    }
    match start {
        None => Err(ParseError::NoStartError),
        Some((y, x)) => {
                // Determine start tile
                let (mut n, mut s, mut w, mut e) = (false, false, false, false);
                if y > 1 && [Some(&Pipe::NorthSouth), Some(&Pipe::SouthEast), Some(&Pipe::SouthWest)].contains(&maze.get(&(y-2, x))) { n = true; }
                if [Some(&Pipe::NorthSouth), Some(&Pipe::NorthEast), Some(&Pipe::NorthWest)].contains(&maze.get(&(y+2, x))) { s = true; }
                if x > 1 && [Some(&Pipe::EastWest), Some(&Pipe::SouthEast), Some(&Pipe::NorthEast)].contains(&maze.get(&(y, x-2))) { w = true; }
                if [Some(&Pipe::EastWest), Some(&Pipe::NorthWest), Some(&Pipe::SouthWest)].contains(&maze.get(&(y, x+2))) { e = true; }
                let p = match (n, s, w, e) {
                    (true, true, false, false) => Ok(Pipe::NorthSouth),
                    (true, false, true, false) => Ok(Pipe::NorthWest),
                    (true, false, false, true) => Ok(Pipe::NorthEast),
                    (false, true, true, false) => Ok(Pipe::SouthWest),
                    (false, true, false, true) => Ok(Pipe::SouthEast),
                    (false, false, true, true) => Ok(Pipe::EastWest),
                    _ => Err(ParseError::InvalidStartNeighbours(n, s, w, e)),
                }?;
                maze.insert((y, x), p);
                if y%4 == x%4 {
                    let ((dy1, dx1), (dy2, dx2)) = p.get_neighbour_diffs();
                    let (n1, n2) = p.get_neighbours();
                    maze.insert((y+dy1-1, x+dx1-1), n1);
                    maze.insert((y+dy2-1, x+dx2-1), n2);
                }
                Ok((maze, (y, x)))
            },
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let (mut maze, start) = try_parse_maze(input)?;
    let first = filter_and_return_max_dist(&mut maze, start);
    let second = count_enclosed(&maze, start);
    Ok((first, second))
}

fn filter_and_return_max_dist(maze: &mut HashMap<(usize, usize), Pipe>, start: (usize, usize)) -> usize {
    let mut visited: HashSet<_> = HashSet::from([start]);
    let mut open_set = VecDeque::from([(0, start)]);
    while let Some((dist, (y, x))) = open_set.pop_front() {
        let curr_tile = maze.get(&(y, x)).unwrap();
        let ((dy1, dx1), (dy2, dx2)) = curr_tile.get_neighbour_diffs();
        if visited.contains(&(y+dy1-1, x+dx1-1)) {
            if visited.contains(&(y+dy2-1, x+dx2-1)) {
                let mut new_maze = visited.iter().map(|(y, x)| ((*y, *x), *maze.get(&(*y, *x)).unwrap())).collect();
                std::mem::swap(&mut new_maze, maze);
                return *open_set.iter().map(|(dist, _pos)| dist).max().unwrap() / 2;
            } else {
                visited.insert((y+dy2-1, x+dx2-1));
                open_set.push_back((dist+1, (y+dy2-1, x+dx2-1)));
            }
        } else {
            visited.insert((y+dy1-1, x+dx1-1));
            open_set.push_back((dist+1, (y+dy1-1, x+dx1-1)));
            if !visited.contains(&(y+dy2-1, x+dx2-1)) {
                visited.insert((y+dy2-1, x+dx2-1));
                open_set.push_back((dist+1, (y+dy2-1, x+dx2-1)));
            }
        }
    }
    unreachable!()
} 

fn count_enclosed(maze: &HashMap<(usize, usize), Pipe>, start: (usize, usize)) -> usize {
    let mut starting: HashSet<_> = (start.0-1..start.0+2).flat_map(|y| (start.1-1..start.1+2).filter(|x| maze.get(&(y, *x)).is_none()).map(|x| (y, x)).collect::<HashSet<(usize, usize)>>()).collect();
    let y_max = maze.iter().map(|((y, _x), _p)| *y).max().unwrap();
    let x_max = maze.iter().map(|((_y, x), _p)| *x).max().unwrap();
    'starting_tile: while let Some(ground) = starting.iter().next() {
        let mut new_ground = 0;
        let mut open_set = VecDeque::from([*ground]);
        let mut visited = HashSet::from([*ground]);
        while let Some((y, x)) = open_set.pop_front() {
            starting.remove(&(y, x));
            if x==0 || y==0 {
                continue 'starting_tile;
            }
            let neighbours = [(y-1, x), (y+1, x), (y, x-1), (y, x+1)];
            for neighbour in neighbours {
                if !maze.contains_key(&neighbour) {
                    if y > y_max || x > x_max {
                        continue 'starting_tile; 
                    } else if !visited.contains(&neighbour) {
                        visited.insert(neighbour);
                        open_set.push_back(neighbour);
                        if neighbour.0%2==1 && neighbour.1%2==1 {
                            new_ground += 1;
                        }
                    }
                }
            }
        }
        return new_ground;
    }
    0
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
        assert_eq!(run(&sample_input), Ok((80, 10)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((6697, 423)));
    }
}
