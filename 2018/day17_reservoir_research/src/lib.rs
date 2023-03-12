use std::collections::{HashSet, HashMap};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Droplet {
    coordinates: (usize, usize),
}

impl Droplet {
    fn spawn(coordinates: (usize, usize)) -> Self {
        Droplet {
            coordinates,
        }
    }
}

#[derive(Clone, Default)]
struct Water {
    reached: HashSet<(usize, usize)>,
    resting: HashSet<(usize, usize)>,
    previous: HashMap<(usize, usize), (usize, usize)>,
    done: HashSet<(usize, usize)>
}

impl Water {
    fn flow(&mut self, spawn_point: (usize, usize), structures: &HashSet<(usize, usize)>) {
        if self.done.contains(&spawn_point) {
            return;
        }
        let mut droplet = Droplet::spawn(spawn_point);
        self.reached.insert(spawn_point);
        let max_y = *structures.iter().max_by_key(|s| s.1).map(|(_x, y)| y).unwrap();
        // try to fall
        let mut down = (spawn_point.0, spawn_point.1 + 1);
        while !structures.contains(&down) && !self.resting.contains(&down) {
            if down.1 > max_y {
                self.done.insert(spawn_point);
                return;
            }
            self.reached.insert(down);
            down.1 += 1;
        }
        let (mut x, y) = (down.0, down.1-1);
        droplet.coordinates.1 = y;
        let this_height: Vec<_> = structures.iter().filter(|(_col, row)| row == &y).map(|(x, _)| *x).collect();
        let l = this_height.iter().filter(|col| *col < &x).max().unwrap_or(&0);
        let r = this_height.iter().filter(|col| *col > &x).min().unwrap_or(&usize::MAX);

        // try to flow left
        let mut reached_left = false;
        loop {
            if x == *l {
                reached_left = true;
                x = droplet.coordinates.0;
                break;
            }
            if structures.contains(&(x, y+1)) || self.resting.contains(&(x, y+1)) {
                self.reached.insert((x, y));
                x -= 1;
            } else {
                self.previous.insert((x, y), spawn_point);
                let reached_before = self.reached.len();
                self.flow((x, y), structures);
                if self.reached.len() == reached_before {
                    self.done.insert((x, y));
                }
                x = down.0;
                break;
            }
        }

        // try to flow right
        let mut reached_right = false;
        loop {
            if x == *r {
                reached_right = true;
                break;
            }
            if structures.contains(&(x, y+1)) || self.resting.contains(&(x, y+1)) {
                self.reached.insert((x, y));
                x += 1;
            } else {
                self.previous.insert((x, y), spawn_point);
                let reached_before = self.reached.len();
                self.flow((x, y), structures);
                if self.reached.len() == reached_before {
                    self.done.insert((x, y));
                }
                break;
            }
        }

        // fill all if we reached both ends
        if reached_left && reached_right {
            (*l+1..*r).for_each(|x| { self.resting.insert((x, y)); });
            if y > spawn_point.1 {
                self.flow(spawn_point, structures);
            // } else if self.reached.len() == reached_before {
            //     self.done.insert(spawn_point);
            } else {
                // dbg!(spawn_point);
                let reached_before = self.reached.len();
                self.flow(*self.previous.get(&spawn_point).unwrap(), structures);
                if self.reached.len() == reached_before {
                    self.done.insert(*self.previous.get(&spawn_point).unwrap());
                }
            }
        }
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let structures: HashSet<(usize, usize)> = input.lines().flat_map(parse_structures).collect();
    let mut water = Water::default();
    water.flow((500, 0), &structures);
    // dbg!(&water.reached);
    let min_y = structures.iter().min_by_key(|s| s.1).map(|s| s.1).unwrap();
    let max_y = structures.iter().max_by_key(|s| s.1).map(|s| s.1).unwrap();
    let first = water.reached.iter().filter(|(_x, y)| (min_y..=max_y).contains(y)).count();
    let second = water.resting.len();
    (first, second)
}

fn parse_structures(line: &str) -> HashSet<(usize, usize)> {
    let components: Vec<_> = line.split(&['=', ',', ' ', '.']).collect();
    // dbg!(&components);
    assert_eq!(components.len(), 7);
    match (components[0], components[1], components[2], components[3], components[4], components[5], components[6]) {
        ("x", x0, "", "y", y0, "", y1) => (y0.parse().unwrap()..=y1.parse().unwrap()).map(|y| (x0.parse().unwrap(), y)).collect(),
        ("y", y0, "", "x", x0, "", x1) => (x0.parse().unwrap()..=x1.parse().unwrap()).map(|x| (x, y0.parse().unwrap())).collect(),
        _ => panic!("Unexpected pattern: {:?}", components),
    }
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
        assert_eq!(run(&sample_input), (57, 29));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (31641, 26321));
    }
}
