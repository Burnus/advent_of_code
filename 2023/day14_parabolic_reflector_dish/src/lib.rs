use core::fmt::Display;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidChar(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChar(c) => write!(f, "Encountered \'{c}\', which is not a valid item"),
        }
    }
}

#[derive(PartialEq)]
enum Direction { North, South, West, East }

struct Platform {
    rounded_rocks: Vec<(usize, usize)>,
    cuboid_rocks: Vec<(usize, usize)>,
}

impl TryFrom<&str> for Platform {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let y_max = value.lines().count() + 1;
        let mut x_max = 2;
        let mut rounded_rocks = Vec::new();
        let mut cuboid_rocks = Vec::from([(0, 0)]);
        for (line_idx, line) in value.lines().enumerate() {
            x_max = x_max.max(line.len() + 1);
            for (char_idx, c) in line.chars().enumerate() {
                match c {
                    'O' => rounded_rocks.push((line_idx+1, char_idx+1)),
                    '#' => cuboid_rocks.push((line_idx+1, char_idx+1)),
                    '.' => (),
                    e => return Err(Self::Error::InvalidChar(e)),
                }
            }
        }
        cuboid_rocks.push((y_max, x_max));

        Ok(Platform { rounded_rocks, cuboid_rocks })
    }
}

impl Platform {
    fn tilt(&mut self, direction: Direction) {
        // sort the rocks such that colliding rocks are adjacent in the Vec
        let sort_order = match direction {
            Direction::North => |(y1, x1): &(usize, usize), (y2, x2): &(usize, usize)| x1.cmp(x2).then_with(|| y1.cmp(y2)),
            Direction::South => |(y1, x1): &(usize, usize), (y2, x2): &(usize, usize)| x1.cmp(x2).then_with(|| y2.cmp(y1)),
            Direction::West => |(y1, x1): &(usize, usize), (y2, x2): &(usize, usize)| y1.cmp(y2).then_with(|| x1.cmp(x2)),
            Direction::East => |(y1, x1): &(usize, usize), (y2, x2): &(usize, usize)| y1.cmp(y2).then_with(|| x2.cmp(x1)),
        };
        self.rounded_rocks.sort_by(sort_order);
        self.cuboid_rocks.sort_by(sort_order);

        // Convenience closure that gives us the coordinates in order: (perpendicular to tilt
        // direction, in tilt direction), so that only objects with the same major coordinate can
        // collide.
        let major_minor = |(y, x): (usize, usize)| -> (usize, usize) {
            match direction {
                Direction::North | Direction::South => (x, y),
                Direction::West | Direction::East => (y, x),
            }
        };
        // Convenience closure that tells us wether the cuboid rock would be before this rounded
        // rock if it were in the same Vec, that is: Its major coordinate is smaller, so it's in a
        // path we already covered, or they are the same and its minor coordinate is such that the
        // rounded rock will be tilted towards it.
        let is_before = |(cuboid_major, cuboid_minor): (usize, usize), major: usize, minor: usize| -> bool {
            cuboid_major < major || (major == cuboid_major && match direction {
                Direction::North | Direction::West => minor > cuboid_minor,
                Direction::South | Direction::East => minor < cuboid_minor,
            })
        };
        let before_edge = match direction {
            Direction::North | Direction::West => 1,
            Direction::South | Direction::East => major_minor(*self.cuboid_rocks.last().unwrap()).1 - 1,
        };

        let mut last_major = 0;
        let mut last_minor = 0;
        let mut cuboid_idx = 0;
        for rounded_rock in self.rounded_rocks.iter_mut() {
            let (major, minor) = major_minor(*rounded_rock);
            if last_major == major {
                // The last rock we moved was in the same path as this one. Move just before it, if
                // no cube shaped rock is in the way.
                
                // Find the first cube shaped rock after this rounded rock (see is_before
                // for what that means) and then go one step back, so that we have the last
                // cube shaped rock before this rounded rock. This is the only one that could
                // potentially be relevant for us.
                while is_before(major_minor(self.cuboid_rocks[cuboid_idx+1]), major, minor) {
                    cuboid_idx += 1;
                }
                let (cuboid_major, cuboid_minor) = major_minor(self.cuboid_rocks[cuboid_idx]);

                if cuboid_major == major {
                    // It is relevant. Move before it or the last rounded rock, whichever comes
                    // first.
                    last_minor = match direction {
                        Direction::North | Direction::West => last_minor.max(cuboid_minor) + 1,
                        Direction::South | Direction::East => last_minor.min(cuboid_minor) - 1,
                    };
                } else {
                    // It isn't relevant. Move before the last rounded rock.
                    last_minor = match direction {
                        Direction::North | Direction::West => last_minor + 1,
                        Direction::South | Direction::East => last_minor - 1,
                    };
                }
            } else {
                // The last rock was in a different path. Move until the edge, if no cube shaped
                // rock is in the way.

                // Again, find the last cube shaped rock before this rounded rock, as above.
                while is_before(major_minor(self.cuboid_rocks[cuboid_idx+1]), major, minor) {
                    cuboid_idx += 1;
                }
                let (cuboid_major, cuboid_minor) = major_minor(self.cuboid_rocks[cuboid_idx]);

                if cuboid_major == major {
                    // It is relevant. Move just before it.
                    last_minor = match direction {
                        Direction::North | Direction::West => cuboid_minor + 1,
                        Direction::South | Direction::East => cuboid_minor - 1,
                    };
                } else {
                    // It isn't relevant. Move just before the edge.
                    last_minor = before_edge;
                }
                last_major = major;
            }
            // Update the minor dimension of this rock in our Vec.
            match direction {
                Direction::North | Direction::South => rounded_rock.0 = last_minor,
                Direction::West | Direction::East => rounded_rock.1 = last_minor,
            };
        }
    }

    fn spin(&mut self, cycle_count: usize) {
        let mut mem: HashMap<Vec<(usize, usize)>, usize> = HashMap::new();
        for idx in 1..=cycle_count {
            self.tilt(Direction::North);
            self.tilt(Direction::West);
            self.tilt(Direction::South);
            self.tilt(Direction::East);
            if let Some(prev) = mem.get(&self.rounded_rocks) {
                // We have already seen this arrangement, meaning we must be in a cycle with period
                // (idx-prev). So we must have seen our target arrangement already, after prev+n
                // steps, where 
                // (prev+n)%period == cycle_count%period. So:
                // n = (cycle_count-prev)%period   // n is equal to n%period, otherwise we wouldn't
                //                                    have encountered it yet. Such an n always
                //                                    exists.
                // prev+n = prev + (cycle_count-prev)%period
                self.rounded_rocks = mem.iter().find(|(_rounded_rocks, after)| **after == prev + (cycle_count-prev)%(idx-prev)).unwrap().0.to_vec();
                break;
            }
            else {
                // This arrangement is new. Save it for later.
                mem.insert(self.rounded_rocks.to_vec(), idx);
            }
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut platform = Platform::try_from(input)?;
    let y_max = platform.cuboid_rocks.last().unwrap().1;
    platform.tilt(Direction::North);
    let first = platform.rounded_rocks.iter().map(|(y, _x)| y_max - y).sum();
    // It doesn't matter that we already tilted north, since tilting twice in the same direction
    // doesn't change anything.
    platform.spin(1000000000);
    let second = platform.rounded_rocks.iter().map(|(y, _x)| y_max - y).sum();
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
        assert_eq!(run(&sample_input), Ok((136, 64)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((108935, 100876)));
    }
}
