use core::fmt::Display;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedMapFeature(char, usize, usize),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedMapFeature(c, x, y) => write!(f, "Trying to parse unexpected map feature {c} at x={x}, y={y}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile { Free, Elf, ProposedOnce, ProposedMultiple }

#[derive(PartialEq)]
enum Direction {
    N,
    S,
    W,
    E,
    NE,
    NW,
    SW,
    SE,
}

impl Direction {
    fn get_considered_directions(index: usize) -> (Self, Self, Self) {
        match index {
            0 => (Self::N, Self::NW, Self::NE),
            1 => (Self::S, Self::SE, Self::SW),
            2 => (Self::W, Self::SW, Self::NW),
            3 => (Self::E, Self::NE, Self::SE),
            _ => panic!("Unexpected Direction Index: {index}"),
        }
    }
    fn get_offset(&self) -> (isize, isize) {
        match self {
            Self::N  => ( 0,-1),
            Self::S  => ( 0, 1),
            Self::W  => (-1, 0),
            Self::E  => ( 1, 0),
            Self::NE => ( 1,-1),
            Self::NW => (-1,-1),
            Self::SW => (-1, 1),
            Self::SE => ( 1, 1),
        } 
    }
}

#[derive(Clone, Copy)]
struct Elf {
    x: isize,
    y: isize,
    considered: Option<(isize, isize)>,
}

impl Elf {
    fn consider(mut self, round: usize, grid: &mut HashMap<(isize, isize), Tile>) -> Self {
        if self.is_alone(grid) { return self; }
        'next_direction: for direction_index in round..round+4 {
            let (considered_direction, diagonal1, diagonal2) = Direction::get_considered_directions(direction_index%4);
            for direction in [&considered_direction, &diagonal1, &diagonal2] {
                let offset = direction.get_offset();
                if let Some(tile) = grid.get(&(self.x + offset.0, self.y + offset.1)) {
                    if *tile == Tile::Elf {
                        continue 'next_direction;
                    } 
                }
            }
            let proposed_offset = considered_direction.get_offset();
            let proposed_coordinates = (self.x + proposed_offset.0, self.y + proposed_offset.1);
            let proposed_tile = grid.get(&proposed_coordinates);
            match proposed_tile {
                None | Some(&Tile::Free) => { 
                        grid.insert(proposed_coordinates, Tile::ProposedOnce);
                        self.considered = Some(proposed_coordinates);
                        break 'next_direction;
                    },
                Some(&Tile::ProposedOnce) => {
                        grid.insert(proposed_coordinates, Tile::ProposedMultiple);
                        // self.considered = None;
                        break 'next_direction;
                    }, 
                _ => { break 'next_direction; },
            }
        }
        self
    }
    fn reposition(mut self, grid: &mut HashMap<(isize, isize), Tile>) -> Self {
        if let Some(considered_coordinates) = self.considered {
            if grid.get(&considered_coordinates) == Some(&Tile::ProposedOnce) {
                grid.insert((self.x, self.y), Tile::Free);
                grid.insert(considered_coordinates, Tile::Elf);
                (self.x, self.y) = considered_coordinates;
            } else {
                grid.insert(considered_coordinates, Tile::Free);
            }
        }
        self.considered = None;
        self
    }

    fn is_alone(&self, grid: &mut HashMap<(isize, isize), Tile>) -> bool {
        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                if x_offset == 0 && y_offset == 0 { continue; }
                if grid.get(&(self.x + x_offset, self.y + y_offset)) == Some(&Tile::Elf) {
                    return false;
                }
            }
        }
        true
    }
}

fn get_free_tiles(elfs: &mut [Elf], grid: &mut HashMap<(isize, isize), Tile>, rounds: usize) -> usize {
    for round in 0..rounds {
        elfs.iter_mut().for_each(|elf| *elf = elf.consider(round, grid));
        elfs.iter_mut().for_each(|elf| *elf = elf.reposition(grid));
        
    }

    let min_x = elfs.iter().map(|elf| elf.x).min().unwrap();
    let max_x = elfs.iter().map(|elf| elf.x).max().unwrap();
    let min_y = elfs.iter().map(|elf| elf.y).min().unwrap();
    let max_y = elfs.iter().map(|elf| elf.y).max().unwrap();

    let width = max_x+1-min_x;
    let height = max_y+1-min_y;

    (width*height) as usize - elfs.len()
}

fn get_last_round(elfs: &mut [Elf], grid: &mut HashMap<(isize, isize), Tile>, starting_round: usize) -> usize {
    for round in starting_round.. {
        elfs.iter_mut().for_each(|elf| *elf = elf.consider(round, grid));
        if !elfs.iter().any(|elf| elf.considered.is_some()) {
            return round + 1;
        }
        elfs.iter_mut().for_each(|elf| *elf = elf.reposition(grid));
    }
    unreachable!("The loop always returns");
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut grid: HashMap<_, _> = input.lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars()
            .enumerate()
            .map(move |(x, c)| match c {
                '.' => Ok(((x as isize, y as isize), Tile::Free)),
                '#' => Ok(((x as isize, y as isize), Tile::Elf)),
                _ => Err(ParseError::UnexpectedMapFeature(c, x, y)),
            }))
        .collect::<Result<HashMap<_, _>, _>>()?;
    let mut elfs: Vec<Elf> = grid.iter()
        .filter(|((_, _), &tile)| tile==Tile::Elf)
        .map(|((x, y), _)| Elf { x: *x, y: *y, considered: None })
        .collect();

    let first = get_free_tiles(&mut elfs, &mut grid, 10);
    let second = get_last_round(&mut elfs, &mut grid, 10);
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
        assert_eq!(run(&sample_input), Ok((110, 20)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((4068, 968)));
    }
}
