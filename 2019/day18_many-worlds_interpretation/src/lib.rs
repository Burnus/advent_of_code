use core::fmt::Display;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    CharMalformed(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CharMalformed(v) => write!(f, "Unexpected Character: {v}"),
        }
    }
}

type Coordinates = (usize, usize);

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Open,
    Wall,
    Entrance,
    Door(usize),
    Key(usize),
}

impl Tile {
    fn print(self) -> char {
        match self {
            Self::Open => '.',
            Self::Wall => '#',
            Self::Entrance => '@',
            Self::Door(i) => (b'A' + i.ilog2() as u8) as char,
            Self::Key(i) => (b'a' + i.ilog2() as u8) as char,
        }
    }
}

struct Vault {
    tiles: Vec<Vec<Tile>>,
}

impl TryFrom<&str> for Vault {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tiles = value.lines()
            .map(|line| line.chars()
                .map(|c| {
                    match c {
                        '.' => Ok(Tile::Open),
                        '#' => Ok(Tile::Wall),
                        '@' => Ok(Tile::Entrance),
                        d if d.is_uppercase() => Ok(Tile::Door(2_usize.pow(d as u32 - b'A' as u32))),
                        k if k.is_lowercase() => Ok(Tile::Key(2_usize.pow(k as u32 - b'a' as u32))),
                        _ => Err(ParseError::CharMalformed(c)),
                    }
                }).collect::<Result<Vec<_>, _>>()
            ).collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            tiles,
        })
    }
}

impl Vault {
    fn split(&self, entrance: (usize, usize)) -> Self {
        let mut tiles = self.tiles.to_vec();

        for dx in 0..3 {
            for dy in 0..3 {
                tiles[entrance.1+dy-1][entrance.0+dx-1] = if dx % 2 == 0 && dy % 2 == 0 { Tile::Entrance } else { Tile::Wall };
            }
        }
        
        Self {
            tiles,
        }
    }

    fn print(&self) -> String {
        self.tiles.iter()
                  .flat_map(|row| row.iter()
                                .map(|t| t.print())
                                .chain(['\n'].into_iter()))
                  .collect()
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct CollectionState {
    positions: Vec<Coordinates>,
    keys_left: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct TraversalState {
    position: Coordinates,
    keys_required: usize,
}

impl TraversalState {
    fn get_neighbours(&self) -> [Self; 4] {
        [
            Self { position: (self.position.0-1, self.position.1), keys_required: self.keys_required },
            Self { position: (self.position.0+1, self.position.1), keys_required: self.keys_required },
            Self { position: (self.position.0, self.position.1-1), keys_required: self.keys_required },
            Self { position: (self.position.0, self.position.1+1), keys_required: self.keys_required },
        ]
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let vault = Vault::try_from(input)?;
    // println!("{}", vault.print());
    let keys: Vec<_> = vault.tiles.iter().enumerate().flat_map(|(y, row)| row.iter().enumerate().filter(|(_x, tile)| matches!(tile, Tile::Key(_))).map(|(x, key)| match key { Tile::Key(k) => (x, y, *k), _ => unreachable!(), }).collect::<Vec<_>>()).collect();
    let entrance = vault.tiles.iter().enumerate().find(|(_y, row)| row.iter().any(|tile| matches!(tile, Tile::Entrance))).map(|(y, row)| (row.iter().position(|tile| matches!(tile, Tile::Entrance)).unwrap(), y)).unwrap();
    let graph = get_graph(&vault, &keys, &[entrance]);
    let first = find_shortest(&graph, &keys, &[entrance]);
    let vault_2 = &vault.split(entrance);
    // println!("{}", vault_2.print());
    let entrances_2 = [(entrance.0-1, entrance.1-1), (entrance.0+1, entrance.1-1), (entrance.0-1, entrance.1+1), (entrance.0+1, entrance.1+1)];
    let graph_2 = get_graph(vault_2, &keys, &entrances_2);
    // dbg!(&graph_2);
    let second = find_shortest(&graph_2, &keys, &entrances_2);
    // let second = 0;
    Ok((first, second))
}

fn find_shortest(graph: &HashMap<((usize, usize), (usize, usize)), Vec<(usize, usize)>>, keys: &[(usize, usize, usize)], entrances: &[(usize, usize)]) -> usize {
    let starting = CollectionState { positions: entrances.to_vec(), keys_left: keys.iter().map(|(_x, _y, k)| k).sum() };
    let mut open_set = HashSet::from([starting.clone()]);
    let mut costs = HashMap::from([(starting, 0)]);
    while !open_set.is_empty() {
        let current = open_set.iter().min_by_key(|s| costs.get(s).unwrap()).unwrap().clone();
        let old_costs = *costs.get(&current).unwrap();
        if current.keys_left == 0 {
            return old_costs;
        }
        open_set.remove(&current);
        for (x, y, key) in keys.iter().filter(|(_x, _y, k)| k & current.keys_left > 0) {
            for cursor in 0..current.positions.len() {
                let mut new = current.clone();
                new.positions[cursor] = (*x, *y);
                new.keys_left -= *key;

                let paths = graph.get(&(current.positions[cursor], new.positions[cursor])).unwrap();
                let shortest_path = paths.iter().find(|(_dist, keys_required)| keys_required & current.keys_left == 0).map(|(dist, _keys_required)| *dist).unwrap_or(usize::MAX);
                let new_costs = old_costs.saturating_add(shortest_path);
                if new_costs < *costs.get(&new).unwrap_or(&usize::MAX) {
                    open_set.insert(new.clone());
                    costs.insert(new.clone(), new_costs);
                }
            }
        }
    }
    panic!("Exhausted all ways but found no solution");
} 

fn get_graph(vault: &Vault, keys: &[(usize, usize, usize)], entrances: &[(usize, usize)]) -> HashMap<((usize, usize), (usize, usize)), Vec<(usize, usize)>> {
    let mut res = HashMap::new();
    for dest in keys {
        let dest = (dest.0, dest.1);
        for starting in entrances {
            res.insert((*starting, dest), get_paths(*starting, dest, vault));
        }
        for start in keys {
            let start = (start.0, start.1);
            let paths = if let Some(rev) = res.get(&(dest, start)) {
                rev.to_vec()
            } else {
                get_paths(start, dest, vault)
            };
            res.insert((start, dest), paths);
        }
    }
    res
}

fn get_paths(start: Coordinates, dest: Coordinates, vault: &Vault) -> Vec<(usize, usize)> {
    let mut open_set = VecDeque::from([TraversalState { position: start, keys_required: 0 }]);
    let mut distances = HashMap::from([(TraversalState { position: start, keys_required: 0 }, 0)]);
    let mut res = Vec::new();
    while let Some(current) = open_set.pop_front() {
        let dist = *distances.get(&current).unwrap();
        if distances.iter().any(|(other, d)| other.position == current.position && d <= &dist && other.keys_required | current.keys_required == current.keys_required && other.keys_required < current.keys_required) {
            continue;
        }
        if res.iter().any(|(d, keys)| d <= &dist && current.keys_required | keys == current.keys_required) {
            continue;
        }
        if current.position == dest {
            res.push((dist, current.keys_required));
            if current.keys_required == 0 {
                res.sort_by_key(|(dist, _keys_required)| *dist);
                return res;
            }
        } else {
            for neighbour in current.get_neighbours().iter_mut() {
                match vault.tiles[neighbour.position.1][neighbour.position.0] {
                    Tile::Wall => continue,
                    Tile::Door(i) => neighbour.keys_required |= i,
                    // Tile::Key(_) if neighbour.position != dest => continue,
                    _ => ()
                }
                let new_dist = dist + 1;
                if distances.get(neighbour).unwrap_or(&usize::MAX) > &new_dist {
                    open_set.push_back(*neighbour);
                    distances.insert(*neighbour, new_dist);
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    // #[ignore]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), Ok((128, 82)));
        // assert_eq!(run(&sample_input), Ok((114, 72)));
    }

    #[test]
    #[ignore]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((5182, 2154)));
    }
}
