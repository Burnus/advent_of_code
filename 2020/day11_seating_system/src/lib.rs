use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidChar(char)
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChar(c) => write!(f, "Invalid Character encountered: {c}"),
        }
    }
}

#[derive(PartialEq, Clone)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

#[derive(Clone)]
struct Ferry {
    seats: Vec<Vec<Seat>>,
}

impl TryFrom<&str> for Ferry {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self{ seats: value.lines().map(|row|
                                          row.chars()
                                          .map(|c| match c {
                                              '.' => Ok(Seat::Floor),
                                              'L' => Ok(Seat::Empty),
                                              '#' => Ok(Seat::Occupied),
                                              _ => Err(Self::Error::InvalidChar(c)),
                                          }).collect::<Result<Vec<_>, _>>())
                                    .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Ferry {
    fn occupied_neighbours(&self, (x, y): (usize, usize)) -> usize {
        (0..=2).map(|dx| (0..=2)
                    .filter(|&dy| !(dx==1 && dy==1) && 
                                  (1..=self.seats.len()).contains(&(y+dy)) && 
                                  (1..=self.seats[0].len()).contains(&(x+dx)) && 
                                  self.seats[y+dy-1][x+dx-1] == Seat::Occupied).count())
                    .sum()
    }

    fn round(&mut self) -> bool {
        let mut next = self.seats.clone();
        let mut changed = false;
        next.iter_mut().enumerate().for_each(|(y, row)|
            row.iter_mut().enumerate().for_each(|(x, s)| {
           match (&s, self.occupied_neighbours((x, y))) {
                (Seat::Empty, 0) => {
                    *s = Seat::Occupied;
                    changed = true;
                },
                (Seat::Occupied, m) if m>3 => {
                    *s = Seat::Empty;
                    changed = true;
                },
                _ => (),
           } 
        }));
        if changed {
            std::mem::swap(&mut next, &mut self.seats);
            true
        } else {
            false
        }
    }

    fn occupied_in_view(&self, (x, y): (usize, usize)) -> usize {
        let mut res = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let mut new_x = x as isize + dx;
                let mut new_y = y as isize + dy;
                while new_x >= 0 && new_y >= 0 && new_y < self.seats.len() as isize && new_x < self.seats[new_y as usize].len() as isize {
                    match self.seats[new_y as usize][new_x as usize] {
                        Seat::Floor => {
                            new_x += dx;
                            new_y += dy;
                        },
                        Seat::Occupied => {
                            res += 1;
                            break;
                        },
                        Seat::Empty => break,
                    }
                }
            }
        }
        res
    }

    fn round_v2(&mut self) -> bool {
        let mut next = self.seats.clone();
        let mut changed = false;
        next.iter_mut().enumerate().for_each(|(y, row)|
            row.iter_mut().enumerate().for_each(|(x, s)| {
           match (&s, self.occupied_in_view((x, y))) {
                (Seat::Empty, 0) => {
                    *s = Seat::Occupied;
                    changed = true;
                },
                (Seat::Occupied, m) if m>4 => {
                    *s = Seat::Empty;
                    changed = true;
                },
                _ => (),
           } 
        }));
        if changed {
            std::mem::swap(&mut next, &mut self.seats);
            true
        } else {
            false
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut ferry_1 = Ferry::try_from(input)?;
    let mut ferry_2 = ferry_1.clone();
    while ferry_1.round() {}
    while ferry_2.round_v2() {}
    let first = ferry_1.seats.iter().map(|row| row.iter().filter(|v| **v == Seat::Occupied).count()).sum();
    let second = ferry_2.seats.iter().map(|row| row.iter().filter(|v| **v == Seat::Occupied).count()).sum();
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
        assert_eq!(run(&sample_input), Ok((37, 26)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((2441, 2190)));
    }
}
