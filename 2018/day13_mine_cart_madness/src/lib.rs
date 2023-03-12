#[derive(Clone, Copy)]
enum Direction { Up, Down, Left, Right }

impl Direction {
    fn coming_from(&self, (old_x, old_y): (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up    => (old_x,   old_y-1),
            Self::Down  => (old_x,   old_y+1),
            Self::Left  => (old_x-1, old_y),
            Self::Right => (old_x+1, old_y),
        }
    }

    fn by_turning(&self, next_turn: &Turning) -> Self {
        match (self, next_turn) {
            (d, Turning::Straight) => *d,
            (Self::Up, Turning::Left) | (Self::Down, Turning::Right) => Self::Left,
            (Self::Left, Turning::Left) | (Self::Right, Turning::Right) => Self::Down,
            (Self::Down, Turning::Left) | (Self::Up, Turning::Right) => Self::Right,
            (Self::Right, Turning::Left) | (Self::Left, Turning::Right) => Self::Up,
        }
    }
}

#[derive(Clone, Copy)]
enum Turning { Left, Straight, Right }

impl Turning {
    fn next(&self) -> Self {
        match self {
            Turning::Left => Turning::Straight,
            Turning::Straight => Turning::Right,
            Turning::Right => Turning::Left,
        }
    }
}

enum Track { Horizontal, Vertical, NE, NW, Intersection, None }

#[derive(Clone)]
struct Cart {
    position: (usize, usize),
    direction: Direction,
    next_turn: Turning,
}

impl Cart {
    fn tick(&mut self, carts: &[Cart], track: &[Vec<Track>]) -> Option<((usize, usize), usize)> {
        let (old_x, old_y) = self.position;
        match track[old_y][old_x] {
            Track::Horizontal | Track::Vertical => (),
            Track::Intersection => {
                    let next_turn = &self.next_turn;
                    self.direction = self.direction.by_turning(next_turn);
                    self.next_turn = self.next_turn.next();
                },
            Track::NE => {
                    self.direction = match self.direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                },
            Track::NW => {
                    self.direction = match self.direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                },
            Track::None => panic!("Car is off track at {}, {}", old_x, old_y),
        }
        let new_position = self.direction.coming_from((old_x, old_y));
        self.position = new_position;
        carts.iter().position(|cart| cart.position == new_position).map(|idx| (new_position, idx))
    }
}

pub fn run(input: &str) -> ((usize, usize), (usize, usize)) {
    let (track, mut carts) = parse_track(input);
    let mut first = (0, 0);
    while carts.len() > 1 {
        let mut idx = 0;
        loop {
            if idx == carts.len() {
                break;
            }
            let mut cart = carts[idx].clone();
            if let Some(crash) = cart.tick(&carts, &track) {
                if first == (0, 0) {
                    first = crash.0;
                }
                if crash.1 > idx {
                    carts.remove(crash.1);
                    carts.remove(idx);
                } else {
                    carts.remove(idx);
                    carts.remove(crash.1);
                    idx -= 1;
                }
            } else {
                carts[idx] = cart;
                idx += 1;
            }
        }
        carts.sort_by(|a, b| match a.position.1.cmp(&b.position.1) {
                std::cmp::Ordering::Equal => a.position.0.cmp(&b.position.0),
                diff => diff,
            });
    }
    let second = carts[0].position;
    (first, second)
}

fn parse_track(input: &str) -> (Vec<Vec<Track>>, Vec<Cart>) {
    let mut carts = Vec::new();
    let mut track = Vec::new();
    for (y, row) in input.lines().enumerate() {
        let mut track_row = Vec::new();
        row.chars().enumerate().for_each(|(x, c)| {
            match c {
                ' '  => track_row.push(Track::None),
                '-'  => track_row.push(Track::Horizontal),
                '|'  => track_row.push(Track::Vertical),
                '/'  => track_row.push(Track::NE),
                '\\' => track_row.push(Track::NW),
                '+'  => track_row.push(Track::Intersection),
                '^'  => {
                        track_row.push(Track::Vertical);
                        carts.push(Cart {
                                position: (x, y),
                                direction: Direction::Up,
                                next_turn: Turning::Left,
                            });
                    },
                '>' => {
                        track_row.push(Track::Horizontal);
                        carts.push(Cart {
                                position: (x, y),
                                direction: Direction::Right,
                                next_turn: Turning::Left,
                            });
                    },
                '<' => {
                        track_row.push(Track::Horizontal);
                        carts.push(Cart {
                                position: (x, y),
                                direction: Direction::Left,
                                next_turn: Turning::Left,
                            });
                    },
                'v' => {
                        track_row.push(Track::Vertical);
                        carts.push(Cart {
                                position: (x, y),
                                direction: Direction::Down,
                                next_turn: Turning::Left,
                            });
                    },
               _ => panic!("Unexpeted track token: {c}"), 
            }
        });
        track.push(track_row);
    }

    (track, carts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..])
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), ((2, 0), (6, 4)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), ((119, 41), (45, 136)));
    }
}
