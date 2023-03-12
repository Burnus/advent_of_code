use std::{fs, collections::HashSet};

#[derive(PartialEq)]
enum Status { Resting, Falling, Blocked }

#[derive(PartialEq)]
enum Mode { EndlessVoid, WithFloor }

#[derive(PartialEq, Eq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn from(string: &str) -> Self {
        let components = string.split(',').collect::<Vec<_>>().iter().map(|i| i.parse().unwrap()).collect::<Vec<usize>>();
        if !components.len() == 2 { 
            panic!("unable to parse {string} into Position"); 
        }

        Self {
            x: components[0],
            y: components[1],
        }
    }
}

struct Sand {
    position: Position,
    ymax: usize,
}

const ORIGIN: Position = Position {
    x: 500,
    y: 0,
};

impl Sand {
    //fn fall(&mut self, cave: &Vec<Position>, other_sand: &mut Vec<Position>, mode: &Mode) -> Status {
    fn fall(&mut self, cave: &HashSet<Position>, other_sand: &mut HashSet<Position>, mode: &Mode) -> Status {
        // return if we fall below all structures
        if *mode == Mode::EndlessVoid && self.position.y >= self.ymax {
            return Status::Falling;
        }
        // or we reached the floor. 
        if *mode == Mode::WithFloor && self.position.y > self.ymax {
            other_sand.insert(self.position.clone());
            return Status::Resting;
        }
        // Fall down if possible
        if !cave.contains(&Position{ x: self.position.x, y: self.position.y+1 }) && !other_sand.contains(&Position { x: self.position.x, y: self.position.y+1 }) {
            self.position.y += 1;
            return self.fall(cave, other_sand, mode);
        }
        // Next try falling left
        if !cave.contains(&Position{ x: self.position.x-1, y: self.position.y+1 }) && !other_sand.contains(&Position { x: self.position.x-1, y: self.position.y+1 }) {
            self.position.x -= 1;
            self.position.y += 1;
            return self.fall(cave, other_sand, mode);
        }
        // Next try falling right
        if !cave.contains(&Position{ x: self.position.x+1, y: self.position.y+1 }) && !other_sand.contains(&Position { x: self.position.x+1, y: self.position.y+1 }) {
            self.position.x += 1;
            self.position.y += 1;
            return self.fall(cave, other_sand, mode);
        }
        // Else we can't fall any more.
        other_sand.insert(self.position.clone());
        if self.position == ORIGIN {
            Status::Blocked
        } else {
            Status::Resting
        }
    }

    fn spawn(cave: &HashSet<Position>, ymax: usize, mode: &Mode) -> HashSet<Position> {
        let mut other_sand = HashSet::new();
        loop {
            let mut new_unit = Sand {
                position: ORIGIN,
                ymax,
            };
            let new_status = new_unit.fall(cave, &mut other_sand, mode);
            if new_status != Status::Resting {
                break;
            }
        }
        other_sand
    }
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn positions_of_formation(formation: &str) -> Vec<Position> {
    let mut blocked = Vec::new();
    let corners = formation.split(" -> ")
                    .map(Position::from)
                    .collect::<Vec<Position>>();
    if corners.len() == 1 {
        return corners;
    }
    for pair in corners.windows(2).collect::<Vec<&[Position]>>() {
        let minx = pair[0].x.min(pair[1].x);
        let maxx = pair[0].x.max(pair[1].x);
        let miny = pair[0].y.min(pair[1].y);
        let maxy = pair[0].y.max(pair[1].y);

        for x in minx..=maxx {
            for y in miny..=maxy {
                blocked.push(Position{ x, y });
            }
        }
    }
    blocked
}

fn get_cave(scan: &str) -> (HashSet<Position>, usize){
    let cave = scan.lines()
        .flat_map(|formation| positions_of_formation(formation).iter()
             .cloned()
             .collect::<HashSet<_>>())
        .collect::<HashSet<_>>();
    let ymax = cave.iter()
                    .map(|pos| pos.y)
                    .max()
                    .unwrap();
    (cave, ymax)
}

fn main() {
    let scan = read_file("input");

    let (cave, ymax) = get_cave(&scan);

    let endless_sand = Sand::spawn(&cave, ymax, &Mode::EndlessVoid);
    println!("In Case of an endless void, {} units of sand will come to a rest", endless_sand.len());

    let sand_with_floor = Sand::spawn(&cave, ymax, &Mode::WithFloor);
    println!("In Case of a floor, {} units of sand will be spawned", sand_with_floor.len());
}

#[test]
fn sample_input() {
    let scan = read_file("tests/sample_input");
    let (cave, ymax) = get_cave(&scan);
    assert_eq!(Sand::spawn(&cave, ymax, &Mode::EndlessVoid).len(), 24);
    assert_eq!(Sand::spawn(&cave, ymax, &Mode::WithFloor).len(), 93);
}

#[test]
fn challenge_input() {
    let scan = read_file("tests/input");
    let (cave, ymax) = get_cave(&scan);
    assert_eq!(Sand::spawn(&cave, ymax, &Mode::EndlessVoid).len(), 979);
    assert_eq!(Sand::spawn(&cave, ymax, &Mode::WithFloor).len(), 29044);
}
