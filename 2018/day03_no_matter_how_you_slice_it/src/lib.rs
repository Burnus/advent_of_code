use std::collections::HashSet;

#[derive(Debug)]
struct Claim {
    id: usize,
    offset: (usize, usize),
    size: (usize, usize),
}

impl From<&str> for Claim {
    fn from(value: &str) -> Self {
        let components: Vec<_> = value.split_whitespace().collect();
        assert_eq!(components.len(), 4);
        let id = components[0][1..].parse().unwrap();
        let off = components[2][..components[2].len()-1].split_once(',').unwrap();
        let dim = components[3].split_once('x').unwrap();

        Self {
            id,
            offset: (off.0.parse().unwrap(), off.1.parse().unwrap()),
            size: (dim.0.parse().unwrap(), dim.1.parse().unwrap()),
        }
    }
}

impl Claim {
    fn overlap(&self, rhs: &Self) -> HashSet<(usize, usize)> {
        let left = self.offset.0.max(rhs.offset.0);
        let right = (self.offset.0 + self.size.0).min(rhs.offset.0 + rhs.size.0);
        let top = self.offset.1.max(rhs.offset.1);
        let bottom = (self.offset.1 + self.size.1).min(rhs.offset.1 + rhs.size.1);
        if left < right && top < bottom {
            (left..right).flat_map(|x| (top..bottom).map(|y| (x, y)).collect::<HashSet<(usize, usize)>>()).collect()
        } else {
            HashSet::new()
        }
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let claims: Vec<_> = input.lines().map(Claim::from).collect();

    let first = claims.iter().enumerate().skip(1).flat_map(|(idx, claim)| 
                    claims.iter().take(idx).flat_map(|other| claim.overlap(other)).collect::<HashSet<(usize, usize)>>()).collect::<HashSet<(usize, usize)>>().len();
    let second = claims.iter().find(|claim| 
                    claims.iter().all(|other| claim.id == other.id || claim.overlap(other).is_empty())).map(|claim| claim.id).unwrap();
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
        assert_eq!(run(&sample_input), (4, 3));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (97218, 717));
    }
}
