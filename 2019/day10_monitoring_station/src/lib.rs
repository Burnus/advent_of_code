use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(PartialEq, Eq)]
struct RationalAngle {
    upper_half: bool,
    divident: isize,
    divisor: usize,
}

impl From<(isize, isize)> for RationalAngle {
    fn from((x, y): (isize, isize)) -> Self {
        let upper_half = x < 0;
        let divident = y.signum() * x / gcd(x, y);
        let divisor = (y / gcd(x, y)).unsigned_abs();

        Self {
            upper_half,
            divident,
            divisor,
        }
    }
}

impl PartialOrd for RationalAngle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.upper_half, other.upper_half) {
            (true, false) => Some(Ordering::Greater),
            (false, true) => Some(Ordering::Less),
            _ => match (self. divident as f64 / self.divisor as f64) - (other.divident as f64 / other.divisor as f64) {
                n if n < 0.0 => Some(Ordering::Greater),
                p if p > 0.0 => Some(Ordering::Less),
                _ => Some(Ordering::Equal),
            }
        }
    }
}

impl Ord for RationalAngle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl RationalAngle {
    fn from_vector(source: (usize, usize), dest: (usize, usize)) -> Self {
        let x = dest.0 as isize - source.0 as isize;
        let y = dest.1 as isize - source.1 as isize;
        Self::from((x, y))
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let asteroids: Vec<(usize, usize)> = input.lines().enumerate().flat_map(|(y, line)| line.chars().enumerate().filter(|(_x, c)| *c == '#').map(|(x, _c)| (x, y)).collect::<Vec<_>>()).collect();
    // let mut angles: Vec<Vec<(isize, isize)>> = asteroids.iter().map(|a| asteroids.iter().filter(|other| *other != a).map(|other| reduced_angle(*a, *other)).collect()).collect();
    let mut angles: Vec<BTreeSet<RationalAngle>> = asteroids.iter().map(|a| asteroids.iter().filter(|other| *other != a).map(|other| RationalAngle::from_vector(*a, *other)).collect()).collect();
    // for asteroid in angles.iter_mut() { 
    //     asteroid.sort_by(rotation_sort);
    //     asteroid.dedup();
    // }
    let (idx, first) = angles.iter().enumerate().max_by_key(|(_idx, a)| a.len()).unwrap();
    let angle = first.iter().nth(199).unwrap();
    let second = (0, 0);
    // let second = (1..).map(|i| ((asteroids[idx].0 as isize + i * angle.divident) as usize, (asteroids[idx].1 as isize + i * angle.divisor as isize) as usize))
    //                   .find(|a| asteroids.contains(a))
    //                   .unwrap();
    (first.len(), (second.0*100 + second.1))
}

fn rotation_sort(lhs: &(isize, isize), rhs: &(isize, isize)) -> Ordering {
    if lhs == rhs {
        return  Ordering::Equal;
    }
    match (lhs.0.signum(), rhs.0.signum()) {
        (-1, 1) => Ordering::Greater,
        (1, -1) => Ordering::Less,
        (0, -1) =>Ordering::Less,
        (-1, 0) => Ordering::Greater,
        (0, 1) => if lhs.1.signum() == 1 { Ordering::Less } else { Ordering::Greater },
        (1, 0) => if rhs.1.signum() == 1 { Ordering::Greater } else { Ordering::Less },
        (0, 0) => match (lhs.1.signum(), rhs.1.signum()) {
            (1, 1) | (-1, -1) => Ordering::Equal,
            (1, -1) => Ordering::Less,
            (-1, 1) => Ordering::Greater,
            _ => panic!("Unable to sort {lhs:?} and {rhs:?}"),
        },
        _ => match (lhs.1.signum(), rhs.1.signum()) {
            (1, 1) | (-1, -1) => match (lhs.0 as f64 / lhs.1 as f64) - (rhs.0 as f64 / rhs.1 as f64) {
                n if n < 0.0 => Ordering::Greater,
                p if p > 0.0 => Ordering::Less,
                _ => panic!("Unexpected sorting of {lhs:?} and {rhs:?}: Equal"),
            },
            _ => (lhs.0.signum()*lhs.1.signum()).cmp(&(lhs.0.signum()*rhs.1.signum()))
        },
    }
}

fn gcd(lhs: isize, rhs: isize) -> isize {
    if lhs == 0 {
        return rhs.abs();
    } else if rhs == 0 {
        return lhs.abs();
    }

    let a = lhs.abs();
    let b = rhs.abs();

    (1..=a.min(b)).rev().find(|i| a%i == 0 && b%i == 0).unwrap()
}

fn reduced_angle(lhs: (usize, usize), rhs: (usize, usize)) -> (isize, isize) {
    let x = rhs.0 as isize - lhs.0 as isize;
    let y = rhs.1 as isize - lhs.1 as isize;
    
    (x/gcd(x,y), y/gcd(x,y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    // #[test]
    // fn test_sample() {
    //     let sample_input = read_file("tests/sample_input");
    //     assert_eq!(run(&sample_input), (33, 0));
    // }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (263, 1110));
    }
}
