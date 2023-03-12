use std::collections::HashMap;

enum Direction { North, West, South, East }

struct Facility {
    rooms: HashMap<(isize, isize), [bool; 4]>,
    current: (isize, isize),
}

impl From<&str> for Facility {
    fn from(value: &str) -> Self {
        let mut facility = Facility::new();
        facility.follow_pattern(value);
        facility
    }
}

impl Facility {
    fn new() -> Self {
        Self { 
            rooms: HashMap::from([((0, 0), [false; 4])]),
            current: (0, 0),
        }
    }

    fn neighbours(&self, from: (isize, isize)) -> Vec<(isize, isize)> {
        [(0, -1), (-1, 0), (0, 1), (1, 0)].iter().enumerate().filter(|(idx, _)| self.rooms.get(&from).unwrap()[*idx]).map(|(_, coords)| (from.0+coords.0, from.1+coords.1)).collect()
    }

    fn distances_from(&self, from: (isize, isize)) -> HashMap<(isize, isize), usize> {
        let mut distances = HashMap::from([(from, 0)]);
        let mut step = 1;
        let mut new = Vec::from([from]);
        while !new.is_empty() {
            let mut next = Vec::new();
            for current in &new {
                let neighbours = self.neighbours(*current);
                for n in neighbours {
                    if !distances.contains_key(&n) && !next.contains(&n) {
                        next.push(n);
                    }
                }
            }
            for n in &next {
                distances.insert(*n, step);
            }
            step += 1;
            std::mem::swap(&mut new, &mut next);
        }
        distances
    }

    fn make_room(&mut self, direction: Direction) {
        let (dir_idx, dir_offset) = match direction {
            Direction::North => (0, (0, -1)),
            Direction::West  => (1, (-1, 0)),
            Direction::South => (2, (0,  1)),
            Direction::East  => (3, (1,  0)),
        };
        self.rooms.get_mut(&self.current).unwrap()[dir_idx] = true;
        self.current.0 += dir_offset.0;
        self.current.1 += dir_offset.1;
        let new = self.rooms.entry(self.current).or_insert([false; 4]);
        new[(dir_idx+2)%4] = true;
    }

    fn follow_pattern(&mut self, value: &str) {
        let mut chars = value.chars();
        while let Some(c) = chars.next() {
            match c {
                '^' => (),
                '$' => return,
                'N' => self.make_room(Direction::North),
                'W' => self.make_room(Direction::West),
                'S' => self.make_room(Direction::South),
                'E' => self.make_room(Direction::East),
                '(' => {
                        let mut nesting = 1;
                        let mut paths = Vec::new();
                        let mut substr = String::new();
                        let start = self.current;
                        while nesting > 0 {
                            let next = chars.next().unwrap_or_else(|| panic!("Unable to close {substr}"));
                            match next {
                                '(' => {
                                        nesting += 1;
                                        substr.push(next);
                                    },
                                ')' => {
                                        nesting -= 1;
                                        if nesting > 0 {
                                            substr.push(next);
                                        } else {
                                            paths.push(substr);
                                            substr = String::new();
                                        }
                                    },
                                '|' => {
                                        if nesting == 1 {
                                            paths.push(substr);
                                            substr = String::new();
                                        } else {
                                            substr.push('|');
                                        }
                                    }
                                'N' | 'W' | 'S' | 'E' => substr.push(next),
                                _ => panic!("Unexpected direction: {next}"),

                            }
                        }
                        for sub_pattern in paths {
                            self.current = start;
                            self.follow_pattern(&sub_pattern);
                        }
                    },
                _ => panic!("Unexpected direction: {c}"),
            }
        }
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let facility = Facility::from(input);
    let distances = facility.distances_from((0, 0));
    let first = *distances.values().max().unwrap();
    let second = distances.values().filter(|dist| **dist >= 1000).count();
    (first, second)
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
        let expected = [
            (3, 0),
            (10, 0),
            (18, 0),
        ];
        for (idx, input) in sample_input.lines().enumerate() {
            assert_eq!(run(input), expected[idx]);
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (4344, 8809));
    }
}
