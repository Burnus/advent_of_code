use std::collections::{HashSet, HashMap};

#[derive(PartialEq, Eq, Hash)]
enum Tile{ Wall, Open, Loc(u8) }

impl Tile {
    fn parse(byte: u8) -> Self {
        match byte {
            b'#' => Self::Wall,
            b'.' => Self::Open,
            n @ b'0'..=b'9' => Self::Loc(n-b'0'),
            _ => panic!("Unexpected Byte: {byte}"),
        }
    }
}

type Coordinates = (usize, usize);
type Route = (Coordinates, Coordinates);

pub fn run(input: &str) -> (usize, usize) {
    let maze: Vec<Vec<Tile>> = input.lines().map(|l| l.bytes().map(Tile::parse).collect()).collect();
    let to_visit: Vec<_> = maze.iter().enumerate().flat_map(|(row, tiles)|tiles.iter().enumerate().filter(|(_, t)| matches!(t, Tile::Loc(_)) && **t != Tile::Loc(0)).map(move |(col, _)| (col, row))).collect();
    let start = maze.iter()
                            .enumerate()
                            .find(|(_, tiles)| tiles.contains(&Tile::Loc(0)))
                            .map(|(row, tiles)| (tiles.iter()
                                                    .position(|t| t == &Tile::Loc(0))
                                                    .unwrap(), row)).unwrap();
    let distances = get_distance_network(start, &to_visit, &maze);
    let first = salesman(start, &to_visit, &distances, false);
    let second = salesman(start, &to_visit, &distances, true);
    (first, second)
}

fn salesman(start: Coordinates, to_visit: &[Coordinates], distances: &HashMap<Route, usize>, returning: bool) -> usize {
    let initial = (start, to_visit.to_vec());
    let mut open_set = HashSet::from([initial.clone()]);
    let mut costs = HashMap::from([(initial.clone(), 0)]);
    let mut estimated_cost = HashMap::from([(initial, to_visit.len() * to_visit.iter().map(|dest| distances.get(&(start, *dest)).unwrap()).min().unwrap())]);
    while !open_set.is_empty() {
        let current = open_set.iter().min_by_key(|state| estimated_cost.get(state).unwrap()).unwrap().clone();
        let current_costs = *costs.get(&current).unwrap();
        open_set.remove(&current);
        if current.1.is_empty() {
            if returning {
                return current_costs + distances.get(&(start, current.0)).unwrap();
            } else {
                return current_costs;
            }
        }
        for next in &current.1 {
            let new_costs = current_costs + distances.get(&(current.0, *next)).unwrap();
            let min_return_cost = current.1.iter().map(|node| distances.get(&(*node, start)).unwrap()).min().unwrap();
            let mut next_to_visit = current.1.clone();
            next_to_visit.remove(next_to_visit.iter().position(|d| d==next).unwrap());
            if new_costs < *costs.get(&(*next, next_to_visit.clone())).unwrap_or(&usize::MAX) {
                let mut next_estimated = if next_to_visit.is_empty() { 0 } else {
                    *next_to_visit.iter().map(|dest| distances.get(&(*next, *dest)).unwrap()).max().unwrap()
                };
                if returning {
                    next_estimated += min_return_cost;
                }
                open_set.insert((*next, next_to_visit.to_vec()));
                costs.insert((*next, next_to_visit.to_vec()), new_costs);
                estimated_cost.insert((*next, next_to_visit.to_vec()), new_costs + next_estimated);
            }
        }
    }
    panic!("Exhausted all ways but found no solution")
} 

fn get_distance_network(start: Coordinates, other: &[Coordinates], maze: &[Vec<Tile>]) -> HashMap<Route, usize> {
    let mut network = HashMap::new();
    let mut combined = other.to_vec();
    combined.push(start);
    for dest in get_network_from(start, &combined, maze) {
        network.insert((start, dest.0), dest.1);
        network.insert((dest.0, start), dest.1);
    }
    for node in other {
        let other_network = get_network_from(*node, other, maze);
        for dest in other_network {
            network.insert((*node, dest.0), dest.1);
        }
    }
    network
}

fn get_network_from(start: Coordinates, other: &[Coordinates], maze: &[Vec<Tile>]) -> HashMap<Coordinates, usize> {
    let mut network = HashMap::new();
    let mut visited = HashSet::from([start]);
    let mut visited_last_step = HashSet::from([start]);
    let mut depth = 1;
    while network.len() < other.len()-1 {
        let mut visited_this_step = HashSet::new();
        for node in &visited_last_step {
            for neighbour in neighbours(*node, maze) {
                if !visited.contains(&neighbour) {
                    visited.insert(neighbour);
                    visited_this_step.insert(neighbour);
                    if other.contains(&neighbour) {
                        network.insert(neighbour, depth);
                    }
                }
            }
        }
        depth += 1;
        std::mem::swap(&mut visited_last_step, &mut visited_this_step);
    }
    network
}

fn neighbours((x, y): Coordinates, maze: &[Vec<Tile>]) -> Vec<Coordinates> {
    let mut res = Vec::new();
    for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let coords =((x as isize + direction.0) as usize, (y as isize + direction.1) as usize);
        if maze[coords.1][coords.0] != Tile::Wall {
            res.push(coords);
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
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), (14, 20));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (462, 676));
    }
}
