use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(PartialEq, Eq)]
struct RationalAngle {
    quadrant: u8,
    divident: isize,
    divisor: isize,
}

impl From<(isize, isize)> for RationalAngle {
    fn from((x, y): (isize, isize)) -> Self {
        let quadrant = match (x.signum(), y.signum()) {
            (0, -1) | (1, -1)  => 1,
            (1, 0)  | (1,  1)  => 2,
            (0, 1)  | (-1, 1)  => 3,
            (-1, 0) | (-1, -1) => 4,
            _ => panic!("Unexpected combination of signs: {x}, {y}"),
        };
        let divident = x / gcd(x, y);
        let divisor = y / gcd(x, y);

        Self {
            quadrant,
            divident,
            divisor,
        }
    }
}

impl PartialOrd for RationalAngle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.quadrant.cmp(&(other.quadrant)) {
            Ordering::Equal => Some((other.divident * self.divisor).cmp(&(self.divident * other.divisor))),
            diff => Some(diff),
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
    let angles: Vec<BTreeSet<RationalAngle>> = asteroids.iter().map(|a| asteroids.iter().filter(|other| *other != a).map(|other| RationalAngle::from_vector(*a, *other)).collect()).collect();
    let (idx, first) = angles.iter().enumerate().max_by_key(|(_idx, a)| a.len()).unwrap();
    let angle = first.iter().nth(199).unwrap();
    let second = (1..).map(|i| ((asteroids[idx].0 as isize + i * angle.divident) as usize, (asteroids[idx].1 as isize + i * angle.divisor) as usize))
                      .find(|a| asteroids.contains(a))
                      .unwrap();
    (first.len(), (second.0*100 + second.1))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    #[ignore = "The sample doesn't have enough asteroids to run part 2."]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), (33, 0));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (263, 1110));
    }
}
