enum Operation {
    SwapPos(usize, usize),
    SwapLtr(char, char),
    RotateL(usize),
    RotateR(usize),
    RotateIdx(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Operation {
    fn parse(line: &str) -> Self {
        let components: Vec<_> = line.split(' ').collect();
        match (components[0], components[1]) {
            ("swap", "position") => Self::SwapPos(components[2].parse().unwrap(), components[5].parse().unwrap()),
            ("swap", "letter") => Self::SwapLtr(components[2].parse().unwrap(), components[5].parse().unwrap()),
            ("rotate", "left") => Self::RotateL(components[2].parse().unwrap()),
            ("rotate", "right") => Self::RotateR(components[2].parse().unwrap()),
            ("rotate", "based") => Self::RotateIdx(components[6].parse().unwrap()),
            ("reverse", _) => Self::Reverse(components[2].parse().unwrap(), components[4].parse().unwrap()),
            ("move", _) => Self::Move(components[2].parse().unwrap(), components[5].parse().unwrap()),
            _ => panic!("Operation not recognized: {line}"),
        }
    }

    fn perform(&self, input: &mut [u8]) {
        match self {
            Self::SwapPos(x, y) => input.swap(*x, *y),
            Self::SwapLtr(x, y) => {
                    let xs: Vec<_> = input.iter().enumerate().filter(|(_, c)| **c == *x as u8).map(|(idx, _)| idx).collect();
                    let ys: Vec<_> = input.iter().enumerate().filter(|(_, c)| **c == *y as u8).map(|(idx, _)| idx).collect();
                    for idx in xs {
                        input[idx] = *y as u8;
                    }
                    for idx in ys {
                        input[idx] = *x as u8;
                    }
                },
            Self::RotateL(x) => input.rotate_left(*x),
            Self::RotateR(x) => input.rotate_right(*x),
            Self::RotateIdx(x) => {
                    let mut mid = 1 + input.iter().position(|c| *c == *x as u8).unwrap();
                    if mid > 4 { mid += 1; }
                    mid %= input.len();
                    input.rotate_right(mid);
                },
            Self::Reverse(x, y) => input[*x..=*y].reverse(),
            Self::Move(x, y) => {
                    if x<y {
                        input[*x..=*y].rotate_left(1);
                    } else {
                        input[*y..=*x].rotate_right(1);
                    }
                },
        }
    }
}

fn get_permutations(rest: &str) -> Vec<String> {
    if rest.len() == 1 {
        return Vec::from([rest.to_string()]);
    }
    let mut res = Vec::new();
    for (idx, c) in rest.chars().enumerate() {
        let mut new_rest = rest.to_string();
        new_rest.remove(idx);
        for this_result in get_permutations(&new_rest) {
            res.push(format!("{c}{this_result}"));
        }
    }
    res
}

pub fn unscramble(input: &str, goal: &str) -> String {
    let permutations = get_permutations(goal);
    for permutation in permutations {
        if scramble(input, &permutation) == *goal.to_string() {
            return permutation;
        }
    }
    "No suitable permutation found.".to_string()
}

pub fn scramble(input: &str, starting: &str) -> String {
    let operations: Vec<_> = input.lines().map(Operation::parse).collect();
    let mut bytes = starting.as_bytes().to_vec();
    for op in operations {
        op.perform(&mut bytes);
    }
    String::from_utf8(bytes).unwrap()
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
        assert_eq!(scramble(&sample_input, "abcde"), "decab".to_string());
        assert_eq!(scramble(&sample_input, "deabc"), "decab".to_string());
        assert_eq!(unscramble(&sample_input, "decab"), "deabc".to_string());
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(scramble(&challenge_input, "abcdefgh"), "bdfhgeca".to_string());
        assert_eq!(unscramble(&challenge_input, "fbgdceah"), "gdfcabeh".to_string());
    }
}
