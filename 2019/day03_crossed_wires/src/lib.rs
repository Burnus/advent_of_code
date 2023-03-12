struct Line {
    bottom_left: (isize, isize),
    top_right:   (isize, isize),
}

impl Line {
    fn intersection(&self, other: &Self) -> Option<(isize, isize)> {
        if self.bottom_left.0 <= other.bottom_left.0 && self.top_right.0 >= other.top_right.0 && other.bottom_left.1 <= self.bottom_left.1 && other.top_right.1 >= self.top_right.1 {
            Some((other.bottom_left.0, self.bottom_left.1))
        } else if other.bottom_left.0 <= self.bottom_left.0 && other.top_right.0 >= self.top_right.0 && self.bottom_left.1 <= other.bottom_left.1 && self.top_right.1 >= other.top_right.1 {
            Some((self.bottom_left.0, other.bottom_left.1))
        } else if self.bottom_left == other.top_right {
            Some(self.bottom_left)
        } else if self.top_right == other.bottom_left {
            Some(self.top_right)
        } else {
            None
        }
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let wire1 = lines_from(lines.next().unwrap());
    let intersections = intersections_by_applying(lines.next().unwrap(), &wire1);
    let first = intersections.iter().map(|(coords, _len2)| coords.0.unsigned_abs() + coords.1.unsigned_abs()).min().unwrap();
    let second = intersections.iter().map(|(coords, len2)| len2 + path_len(&wire1, *coords)).min().unwrap();
    (first, second)
}

fn path_len(wire: &[Line], (x, y): (isize, isize)) -> usize {
    let mut len = 0;
    let mut current = (0, 0);
    for segment in wire {
        if (segment.bottom_left.0..=segment.top_right.0).contains(&x) && (segment.bottom_left.1..=segment.top_right.1).contains(&y) {
            return len + x.abs_diff(current.0) + y.abs_diff(current.1);
        } else {
            len += segment.bottom_left.0.abs_diff(segment.top_right.0) + segment.bottom_left.1.abs_diff(segment.top_right.1);
            current = if current == segment.bottom_left { segment.top_right } else { segment.bottom_left };
        }
    }

    len
}

fn intersections_by_applying(val: &str, existing_wire: &[Line]) -> Vec<((isize, isize), usize)> {
    let mut len=0;
    let mut current = (0, 0);
    val.split(',')
        .flat_map(|segment| {
            let (this, dist) = segment_from(segment, &mut current);
            len += dist;
            existing_wire.iter()
                         .filter_map(|other| this.intersection(other))
                         .map(|intersection| (intersection, len - intersection.0.abs_diff(current.0) - intersection.1.abs_diff(current.1)))
                         .collect::<Vec<((isize, isize), usize)>>()
        // Skip the first intersection (at the origin)
        }).skip(1)
        .collect()

}

fn lines_from(val: &str) -> Vec<Line> {
    let mut current = (0, 0);
    val.split(',').map(|segment| segment_from(segment, &mut current).0).collect()
}

fn segment_from(val: &str, current: &mut (isize, isize)) -> (Line, usize) {
        let dir = val.chars().next().unwrap();
        let count: String = val.chars().skip(1).collect();
        let dist = count.parse::<isize>().unwrap();
        let dest = match dir {
            'U' => (current.0,        current.1 - dist),
            'D' => (current.0,        current.1 + dist),
            'L' => (current.0 - dist, current.1),
            'R' => (current.0 + dist, current.1),
            _ => panic!("Unexpected Direction: {dir}"),
        };
        let res = Line {
            bottom_left: (current.0.min(dest.0), current.1.min(dest.1)),
            top_right:   (current.0.max(dest.0), current.1.max(dest.1)),
        };
        *current = dest;
        (res, dist as usize)
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
        assert_eq!(run(&sample_input), (159, 610));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (1225, 107036));
    }
}
