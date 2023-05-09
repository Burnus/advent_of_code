use core::fmt::Display;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InputMalformed(String),
    InvalidChar(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputMalformed(v) => write!(f, "Input is malformed: {v}"),
            Self::InvalidChar(c) => write!(f, "Tried to construct Amphipod from invalid character {c}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Type {
    fn energy_modifier(&self) -> usize {
        match self {
            Self::Amber  => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }

    fn target_x(&self) -> usize {
        match self {
            Self::Amber  => 2,
            Self::Bronze => 4,
            Self::Copper => 6,
            Self::Desert => 8,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Space {
    Free,
    Occupied(Amphipod),
    Entrance,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Initial,
    Hallway,
    Stopped,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Amphipod {
    amphipod_type: Type,
    amphipod_state: State,
}

impl TryFrom<char> for Amphipod {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let amphipod_type = match value {
            'A' => Ok(Type::Amber),
            'B' => Ok(Type::Bronze),
            'C' => Ok(Type::Copper),
            'D' => Ok(Type::Desert),
            _ => Err(Self::Error::InvalidChar(value)),
        }?;
        let amphipod_state = State::Initial;

        Ok(Self { amphipod_type, amphipod_state, })
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Burrow {
    energy_spent: usize,
    spaces: BTreeMap<(usize, usize), Space>,
}

impl TryFrom<&str> for Burrow {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines: Vec<_> = value.lines().collect();
        if lines.len() != 5 || lines.iter().enumerate().any(|(idx, line)| match (idx, line.len()) {
            (s, 13) if s<3 => false,
            (e, 11) if e>2 => false,
            _ => true,
        }) {
            return Err(Self::Error::InputMalformed(value.to_string()));
        }
        let mut spaces = BTreeMap::new();
        (0..11).for_each(|x| {
            let space = match x {
                2 | 4 | 6 | 8 => Space::Entrance,
                _ => Space::Free,
            };
            spaces.insert((x, 0), space); 
        });
        for y in 1..=2 {
            for x in (2..=8).step_by(2) {
                let amphipod = Amphipod::try_from(lines[1+y].chars().nth(1+x).unwrap())?;
                spaces.insert((x, y), Space::Occupied(amphipod));
            }
        }

        for ((x, _y), s) in spaces.iter_mut().filter(|((_x, y), _s)| *y == 2) {
            if let Space::Occupied(curr) = s {
                if *x == curr.amphipod_type.target_x() {
                    curr.amphipod_state = State::Stopped;
                }
            }
        }

        Ok(Self { energy_spent: 0, spaces, })
    }
}

impl Burrow {
    fn possible_moves(&self, ((x, y), s): (&(usize, usize), &Space)) -> Vec<Self> {
        if let Space::Occupied(amphipod) = s {
            let mut res = Vec::new();
                
            // If possible, move into target.
            if *y < 2 || (1..*y).all(|row| self.spaces.get(&(*x, row)) == Some(&Space::Free)) { 
                let target_x = amphipod.amphipod_type.target_x();
                if !self.spaces.iter().any(|((col, y), s)|
                    // Are there any amphipods that still need to leave this chamber?
                    (*col == target_x && match s {
                        Space::Occupied(a) => a.amphipod_state == State::Initial,
                        _ => false,
                    }) ||
                    // Are any of the hallway spaces between current x and target_x occupied?
                    ((*x.min(&target_x)+1..*x.max(&target_x)).contains(col) && *y == 0 && matches!(s, Space::Occupied(_)) )) {
                        // The new y is the max y of free spaces in this chamber
                        let new_y = self.spaces.iter().filter(|(&(x, _y), &s)| x == target_x && s == Space::Free ).map(|((_x, y), _s)| *y).max().unwrap();

                        // Move the amphipod into the chamber and set its status to Stopped
                        let mut spaces = self.spaces.clone();
                        let s = Space::Occupied(Amphipod { amphipod_type: amphipod.amphipod_type, amphipod_state: State::Stopped });
                        spaces.insert((target_x, new_y), s);
                        spaces.insert((*x, *y), Space::Free);

                        // return the new state early since there can't possibly any better move
                        // for this amphipod.
                        return vec![Self { energy_spent: self.energy_spent + amphipod.amphipod_type.energy_modifier()*(*y+new_y+target_x.abs_diff(*x)), spaces }];
                }

                // Otherwise, try moveing into corridor. This is only allowed if we haven't moved
                // yet.
                if amphipod.amphipod_state == State::Initial {
                    // First look right
                    for dx in 1.. {
                        // skip spaces right outside the chambers and stop if we hit a wall or
                        // another amphipod
                        match self.spaces.get(&(*x+dx, 0)) {
                            Some(Space::Free) => (),
                            Some(Space::Entrance) => continue,
                            _ => break,
                        }
                        // Move the amphipod into the new space and set its status to Hallway
                        let mut spaces = self.spaces.clone();
                        let s = Space::Occupied(Amphipod { amphipod_type: amphipod.amphipod_type, amphipod_state: State::Hallway });
                        spaces.insert((x+dx, 0), s);
                        spaces.insert((*x, *y), Space::Free);
                        let next = Self { energy_spent: self.energy_spent + amphipod.amphipod_type.energy_modifier()*(y+dx), spaces };
                        res.push(next);
                    }
                    // Look left -- same as right, but make sure x doesn't become negative
                    for dx in 1..=*x {
                        match self.spaces.get(&(*x-dx, 0)) {
                            Some(Space::Free) => (),
                            Some(Space::Entrance) => continue,
                            _ => break,
                        }
                        let mut spaces = self.spaces.clone();
                        let s = Space::Occupied(Amphipod { amphipod_type: amphipod.amphipod_type, amphipod_state: State::Hallway });
                        spaces.insert((x-dx, 0), s);
                        spaces.insert((*x, *y), Space::Free);
                        let next = Self { energy_spent: self.energy_spent + amphipod.amphipod_type.energy_modifier()*(y+dx), spaces };
                        res.push(next);
                    }
                }
            }
            // Return whatever we have collected. May be nothing if all paths are blocked.
            res
        } else {
            Vec::new()
        }
    }

    // Dykstra's Algorithm
    fn organize(&mut self) -> usize {
        let mut open_set = BTreeSet::from([self.clone()]);

        // Pick the cheapest move available
        while let Some(current) = open_set.pop_first() {
            // If we are done, return the energy spent so far
            let to_move: Vec<_> = current.spaces.iter().filter(|(_coords, s)| match s {
                Space::Occupied(a) => a.amphipod_state != State::Stopped,
                _ => false,
            }).collect();
            if to_move.is_empty() {
                return current.energy_spent;
            } else {
                // Otherwise find out where each amphipod can go from here
                for a in to_move {
                    for next in current.possible_moves(a) {
                        open_set.insert(next);
                    }
                }
            }
        }
        // return 0 if there was no solution
        0
    }

    fn expand(&self) -> Self {
        let middle = BTreeMap::from([
            ((2, 2), Space::Occupied(Amphipod::try_from('D').unwrap())),
            ((2, 3), Space::Occupied(Amphipod::try_from('D').unwrap())),
            ((4, 2), Space::Occupied(Amphipod::try_from('C').unwrap())),
            ((4, 3), Space::Occupied(Amphipod::try_from('B').unwrap())),
            ((6, 2), Space::Occupied(Amphipod::try_from('B').unwrap())),
            ((6, 3), Space::Occupied(Amphipod::try_from('A').unwrap())),
            ((8, 2), Space::Occupied(Amphipod::try_from('A').unwrap())),
            ((8, 3), Space::Occupied(Amphipod::try_from('C').unwrap())),
        ]);
        let mut spaces = self.spaces.iter()
            .map(|(&(x, y), &s)| ((x, y*y), s))
            .collect::<BTreeMap<_, _>>();
        spaces.extend(middle);

        Self { energy_spent: self.energy_spent, spaces, }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut burrow_1 = Burrow::try_from(input)?;
    let mut burrow_2 = burrow_1.expand();

    let first = burrow_1.organize();
    let second = burrow_2.organize();
    Ok((first, second))
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
        assert_eq!(run(&sample_input), Ok((12521, 44169)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((10526, 41284)));
    }
}
