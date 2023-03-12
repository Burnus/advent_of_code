use std::{fs, isize, collections::HashMap};

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

fn read_file(path: &str) -> HashMap<(isize, isize), Tile> {
    fs::read_to_string(path)
        .expect("File not Found")
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars()
            .enumerate()
            .map(move |(x, c)| ((x as isize, y as isize), match c {
                '.' => Tile::Free,
                '#' => Tile::Elf,
                _ => panic!("Unexpected Map Feature: {c} at {x}, {y}"),
            })))
        .collect()
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
        if !elfs.iter().any(|elf| elf.considered != None) {
            return round + 1;
        }
        elfs.iter_mut().for_each(|elf| *elf = elf.reposition(grid));
    }
    unreachable!("The loop always returns");
}

fn main() {
    let mut grid = read_file("input");

    let mut elfs: Vec<Elf> = grid.iter()
        .filter(|((_, _), &tile)| tile==Tile::Elf)
        .map(|((x, y), _)| Elf { x: *x, y: *y, considered: None })
        .collect();

    println!("After 10 Rounds, {} tiles are free.", get_free_tiles(&mut elfs, &mut grid, 10));
    println!("No more movement after round {}.", get_last_round(&mut elfs, &mut grid, 10));
}

#[test]
fn sample_input() {
    let mut grid = read_file("tests/sample_input");

    let mut elfs: Vec<Elf> = grid.iter()
        .filter(|((_, _), &tile)| tile==Tile::Elf)
        .map(|((x, y), _)| Elf { x: *x, y: *y, considered: None })
        .collect();

    assert_eq!(get_free_tiles(&mut elfs, &mut grid, 10), 110);
    assert_eq!(get_last_round(&mut elfs, &mut grid, 10), 20);
}

#[test]
fn challenge_input() {
    let mut grid = read_file("tests/input");

    let mut elfs: Vec<Elf> = grid.iter()
        .filter(|((_, _), &tile)| tile==Tile::Elf)
        .map(|((x, y), _)| Elf { x: *x, y: *y, considered: None })
        .collect();

    assert_eq!(get_free_tiles(&mut elfs, &mut grid, 10), 4068);
    assert_eq!(get_last_round(&mut elfs, &mut grid, 10), 968);
}
