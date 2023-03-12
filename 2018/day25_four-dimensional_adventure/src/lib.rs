#[derive(Clone, Copy)]
struct Point4D {
    coordinates: (isize, isize, isize, isize)
}

impl From<&str> for Point4D {
    fn from(value: &str) -> Self {
        let components: Vec<_> = value.split(',').collect();
        assert_eq!(components.len(), 4);
        Self {
            coordinates: ( components[0].parse().unwrap(), components[1].parse().unwrap(), components[2].parse().unwrap(), components[3].parse().unwrap() ),
        }
    }
}

impl Point4D {
    fn dist(&self, other: &Point4D) -> usize {
        self.coordinates.0.abs_diff(other.coordinates.0) +
            self.coordinates.1.abs_diff(other.coordinates.1) +
            self.coordinates.2.abs_diff(other.coordinates.2) +
            self.coordinates.3.abs_diff(other.coordinates.3)
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let mut constellations: Vec<Vec<Point4D>> = Vec::new();
    input.lines().for_each(|line| {
        let point = Point4D::from(line);
        let connected: Vec<usize> = constellations.iter().enumerate().filter(|(_idx, constell)| constell.iter().any(|star| point.dist(star) <= 3)).map(|(idx, _constell)| idx).collect();
        match connected.len() {
            0 => constellations.push(vec![point]),
            1 => constellations[connected[0]].push(point),
            _ => {
                let first_idx = connected[0];
                constellations[first_idx].push(point);
                for idx in connected.iter().skip(1).rev() {
                    assert!(idx > &first_idx);
                    let mut other = constellations.remove(*idx);
                    constellations[first_idx].append(&mut other);
                }
                constellations.retain(|v| !v.is_empty());
            }
        }
    });
    let first = constellations.len();
    let second = 0;
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
        assert_eq!(run(&sample_input), (2, 0));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (324, 0));
    }
}
