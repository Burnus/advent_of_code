use core::fmt::Display;
use std::{num::ParseIntError, collections::BTreeSet};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ImageCorrupted(String),
    ParseIntError(std::num::ParseIntError),
    LineMalformed(String),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ImageCorrupted(s) => write!(f, "Image data seem to be corrupted: {s}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

const SIDE_LENGTH: usize = 10;
const MONSTER_PATTERN: &str =
"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";
const MONSTER_SIZE: usize = 15;

#[derive(Clone)]
struct Tile {
    id: usize,
    pixels: [[bool; SIDE_LENGTH]; SIDE_LENGTH],
    neighbours: [Option<usize>; 4],
}

impl TryFrom<&str> for Tile {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines: Vec<_> = value.lines().collect();
        if lines.len() != 11 {
            return Err(Self::Error::ImageCorrupted(format!("Image contains {} lines instead of 11.", lines.len())));
        }
        if let Some(id_str) = lines[0].split_whitespace().last() {
            let id = id_str[..id_str.len()-1].parse()?;
            let mut pixels = [[false; SIDE_LENGTH]; SIDE_LENGTH];

            for (y, line) in lines.iter().skip(1).enumerate() {
                if line.len() != SIDE_LENGTH {
                    return Err(Self::Error::LineMalformed(line.to_string()));
                }
                for (x, c) in line.chars().enumerate() {
                    match c {
                        '.' => (),
                        '#' => pixels[y][x] = true,
                        _ => return Err(Self::Error::LineMalformed(line.to_string())),
                    }
                }
            }
            let neighbours = [None; 4];
            Ok(Self { id, pixels, neighbours})
        } else {
            Err(Self::Error::LineMalformed(lines[0].to_string()))
        }
    }
}

impl Tile {
    fn flip_horizontally(&mut self) {
        let old_pixels = self.pixels.clone(); 
        self.pixels.iter_mut().enumerate().for_each(|(y, row)| row.iter_mut().enumerate().for_each(|(x, p)| *p = old_pixels[y][SIDE_LENGTH-x-1]));
    }

    fn rotate_left(&mut self) {
        let old_pixels = self.pixels.clone(); 
        self.pixels.iter_mut().enumerate().for_each(|(y, row)| row.iter_mut().enumerate().for_each(|(x, p)| *p = old_pixels[x][SIDE_LENGTH-y-1]));
    }

    fn rotate_180(&mut self) {
        let old_pixels = self.pixels.clone(); 
        self.pixels.iter_mut().enumerate().for_each(|(y, row)| row.iter_mut().enumerate().for_each(|(x, p)| *p = old_pixels[SIDE_LENGTH-y-1][SIDE_LENGTH-x-1]));
    }

    fn borders(&self) -> [[bool; SIDE_LENGTH]; 4] {
        let mut copied = self.clone();
        copied.rotate_left();
        [
            self.pixels[0],                 // top
            copied.pixels[SIDE_LENGTH-1],   // left
            self.pixels[SIDE_LENGTH-1],     // bottom
            copied.pixels[0],               // right
        ]
    }

    fn neighbours_count(&self) -> usize {
        self.neighbours.iter().filter(|neighbour| neighbour.is_some()).count()
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut tiles: Vec<_> = input.split("\n\n").map(Tile::try_from).collect::<Result<Vec<_>, _>>()?;
    find_neighbours(&mut tiles);

    let first = tiles.iter().filter(|tile| tile.neighbours_count() == 2).map(|tile| tile.id).product();

    let image = assemble(&tiles);
    let monsters = count_monsters(&image);

    let second = image.iter().map(|row| row.iter().filter(|b| **b).count()).sum::<usize>() - monsters * MONSTER_SIZE;
    Ok((first, second))
}

fn find_neighbours(tiles: &mut [Tile]) {
    let mut open_set = BTreeSet::from([0]);
    let mut todo: Vec<_> = (0..tiles.len()).collect();
    while let Some(target_idx) = open_set.pop_first() {
        todo.remove(todo.iter().position(|t| *t == target_idx).unwrap());
        let mut target = tiles[target_idx].clone();
        let target_borders = target.borders();

        todo.iter().for_each(|other_idx| {
            let mut other = &mut tiles[*other_idx];
            for _ in 0..4 {
                for (side, other_border) in other.borders().into_iter().enumerate() {
                    let mut rev = other_border;
                    rev.reverse();
                    if target_borders[(side+2)%4] == other_border {
                        other.neighbours[side] = Some(target_idx);
                        target.neighbours[(side+2)%4] = Some(*other_idx);
                        open_set.insert(*other_idx);
                        return;
                    } else if target_borders[(side+2)%4] == rev {
                        other.flip_horizontally();
                        if side % 2 == 1 {
                            other.rotate_180();
                        }
                        open_set.insert(*other_idx);
                        other.neighbours[side] = Some(target_idx);
                        target.neighbours[(side+2)%4] = Some(*other_idx);
                        return;
                    }
                }
                other.rotate_left();
            }
        });
        tiles[target_idx] = target;
    }
}

fn assemble(tiles: &[Tile]) -> Vec<Vec<bool>> {
    let offset_factor = SIDE_LENGTH-2;
    let first_idx = tiles.iter().position(|t| t.neighbours[0].is_none() && t.neighbours[1].is_none()).unwrap();

    let mut tile_ids = vec![vec![first_idx]];
    let mut current_idx = first_idx;
    while let Some(next) = tiles[current_idx].neighbours[2] {
        tile_ids.push(vec![next]);
        current_idx = next;
    }

    for row in tile_ids.iter_mut() {
        current_idx = row[0];
        while let Some(next) = tiles[current_idx].neighbours[3] {
            row.push(next);
            current_idx = next;
        }

    }

    let mut res = vec![vec![false; offset_factor*tile_ids[0].len()]; offset_factor*tile_ids.len()];

    tile_ids.iter().enumerate().for_each(|(tile_y, tile_row)| {
        tile_row.iter().enumerate().for_each(|(tile_x, tile_idx)| {
            tiles[*tile_idx].pixels.iter().skip(1).take(offset_factor).enumerate().for_each(|(y, row)| {
                row.iter().skip(1).take(offset_factor).enumerate().for_each(|(x, b)| {
                    if *b {
                        res[tile_y*offset_factor+y][tile_x*offset_factor+x] = true;
                    }
                });
            });
        });
    });
    res
}

fn count_monsters(image: &[Vec<bool>]) -> usize {
    let mut monster: Vec<Vec<bool>> = MONSTER_PATTERN.lines().map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>()).collect();
    let mut monsters = Vec::new();
    for _ in 0..4 {
        let old_width = monster[0].len();

        let mut flipped_monster = monster.clone();
        flipped_monster.iter_mut().enumerate().for_each(|(y, row)| row.iter_mut().enumerate().for_each(|(x, p)| *p = monster[y][old_width-x-1]));
        monsters.push(flipped_monster);

        let mut rotated_monster = vec![vec![false; monster.len()]; old_width];
        rotated_monster.iter_mut().enumerate().for_each(|(y, row)| row.iter_mut().enumerate().for_each(|(x, p)| *p = monster[x][old_width-y-1]));
        monsters.push(rotated_monster.clone());
        std::mem::swap(&mut monster, &mut rotated_monster);
    }

    let mut monster_count = 0;

    for monster in monsters {
        let height = monster.len();
        let width = monster[0].len();
        for y_offset in 0..(image.len()-height) {
            'pos: for x_offset in 0..(image[0].len()-width) {
                for (row_idx, row) in image.iter().skip(y_offset).take(height).enumerate() {
                    for (col_idx, pix) in row.iter().skip(x_offset).take(width).enumerate() {
                        if monster[row_idx][col_idx] && !pix {
                            continue 'pos;
                        }
                    }
                }
                monster_count += 1;
            }
        }
    }

    monster_count
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
        assert_eq!(run(&sample_input), Ok((20899048083289, 273)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((7901522557967, 2476)));
    }
}
