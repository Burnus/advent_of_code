enum Direction { North, NorthEast, SouthEast, South, SouthWest, NorthWest }

impl Direction {
    fn parse(string_repr: &str) -> Self {
        match string_repr {
            "n" => Self::North,
            "ne" => Self::NorthEast,
            "se" => Self::SouthEast,
            "s" => Self::South,
            "sw" => Self::SouthWest,
            "nw" => Self::NorthWest,
            _ => panic!("Unknown Direction: {string_repr}"),
        }
    }

    fn movement_components(&self) -> (isize, isize) {
        match self {
            Self::North => (2, 0),
            Self::NorthEast => (1, 1),
            Self::SouthEast => (-1, 1),
            Self::South => (-2, 0),
            Self::SouthWest => (-1, -1),
            Self::NorthWest => (1, -1),
        }
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let (last, max) = input.split(',')
        .map(Direction::parse)
        .fold(((0, 0), 0), |(current, max_distance), direction| {
            let new = go_direction(current, &direction);
            (new, max_distance.max(distance_from_origin(new)))
        });
    (distance_from_origin(last), max)
}

fn go_direction(current: (isize, isize), direction: &Direction) -> (isize, isize) {
    let movement = direction.movement_components();
    (current.0 + movement.0, current.1 + movement.1)
}

fn distance_from_origin(position: (isize, isize)) -> usize {
    let ew_distance = position.1.unsigned_abs();
    let ns_distance = position.0.unsigned_abs();
    ew_distance + ns_distance.saturating_sub(ew_distance)/2
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
        let sample_inputs = read_file("tests/sample_input");
        let sample_results = [
            (3, 3),
            (0, 2),
            (2, 2),
            (3, 3),
        ];
        for (idx, line) in sample_inputs.lines().enumerate() {
            assert_eq!(run(line.trim()), sample_results[idx]);
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (796, 1585));
    }
}
