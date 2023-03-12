use std::fs;
use std::collections::HashSet;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn perform_motion(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.x-=1,
            Direction::Right => self.x+=1,
            Direction::Up => self.y+=1,
            Direction::Down => self.y-=1,
        }
    }

    fn follow(&self, head: &Position) -> Self {
        let (dx, dy) = (head.x-self.x, head.y-self.y);
        match (dx, dy) {
            (2,0) => Self { x: self.x+1, y: self.y },
            (0,2) => Self { x: self.x, y: self.y+1 },
            (-2,0) => Self { x: self.x-1, y: self.y },
            (0,-2) => Self { x: self.x, y: self.y-1},
            (2,2)| (2,1) | (1,2) => Self { x: self.x+1, y: self.y+1 },
            (2,-2) | (2,-1) | (1,-2) => Self { x: self.x+1, y: self.y-1 },
            (-2,-2) | (-2,-1) | (-1,-2) => Self { x: self.x-1, y: self.y-1 },
            (-2,2) | (-2,1) |(-1,2) => Self { x: self.x-1, y: self.y+1 },
            _ => *self,
        }
    }
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn parse_head_movement(instruction: &str) -> (Direction, i32) {
    let direction = match &instruction[0..=0] {
        "L" => Direction::Left,
        "R" => Direction::Right,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => panic!("Unknown Direction"),
    };

    let count = instruction[2..].parse().unwrap();
    (direction, count)
}

fn get_visited(head_movements: &str, rope_length: usize) -> HashSet<Position> {
    let mut positions = vec![Position { x:0, y: 0 }; rope_length];
    let mut visited = HashSet::new();
    visited.insert(positions[0]);
    for instruction in head_movements.lines() {
        let (head_direction, count) = parse_head_movement(instruction);
        for _ in 0..count {
            positions[0].perform_motion(&head_direction);
            for i in 1..rope_length {
                positions[i] = positions[i].follow(&positions[i-1]);
            }
            visited.insert(positions[rope_length-1]);
        }
    }
    visited
}

fn main() {
    //let head_movements = read_file("sample_input");
    let head_movements = read_file("input");

    let visited_fixed = get_visited(&head_movements, 2);
    let visited_loose = get_visited(&head_movements, 10);

    println!("The fixed tail visited a total of {} positions.", visited_fixed.len());
    println!("The loose tail visited a total of {} positions.", visited_loose.len());
}

#[test]
fn sample_input() {
    let head_movements = read_file("tests/sample_input");
    assert_eq!(get_visited(&head_movements, 2).len(), 13);
    assert_eq!(get_visited(&head_movements, 10).len(), 1);
}

#[test]
fn challenge_input() {
    let head_movements = read_file("tests/input");
    assert_eq!(get_visited(&head_movements, 2).len(), 6376);
    assert_eq!(get_visited(&head_movements, 10).len(), 2607);
}
