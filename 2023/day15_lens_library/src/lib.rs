use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    LineMalformed(&'a str),
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

enum Operation {
    Dash,
    Equals(usize),
}

impl Operation {
    fn perform<'a>(&self, boxes: &mut [Vec<(&'a str, usize)>], label: &'a str) {
        let box_idx = hash(label);
        match self {
            Operation::Dash => if let Some(idx) = boxes[box_idx].iter().position(|(lb, _focal_length)| lb == &label) {
                    _ = boxes[box_idx].remove(idx)
                },
            Operation::Equals(focal_length) => if let Some((_label, fl)) = boxes[box_idx].iter_mut().find(|(lb, _lens)| lb == &label) {
                    *fl = *focal_length
                } else {
                    boxes[box_idx].push((label, *focal_length))
                },
        };
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let steps: Vec<_> = input.split(',').collect();
    let first = steps.iter().map(|s| hash(s)).sum();
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    for step in steps {
        let (label, operation) = try_parse(step)?;
        operation.perform(&mut boxes, label);
    }
    let second = boxes.iter()
                        .enumerate()
                        .map(|(box_idx, lens_box)| (1+box_idx) * lens_box.iter()
                                                                    .enumerate()
                                                                    .map(|(idx, (_label, focal_length))| (1+idx)*focal_length)
                                                                    .sum::<usize>())
                        .sum();
    Ok((first, second))
}

fn hash(step: &str) -> usize {
    step.bytes().fold(0, |acc, b| ((acc + b as usize) * 17 ) % 256)
}

fn try_parse(step: &str) -> Result<(&str, Operation), ParseError> {
    let len = step.len();
    let bytes = step.as_bytes();

    match bytes[len-1] {
        b'-' => Ok((&step[..len-1], Operation::Dash)),
        n if n.is_ascii_digit() => if bytes[len-2] == b'=' {
                Ok((&step[..len-2], Operation::Equals((n - b'0') as usize)))
            } else {
                Err(ParseError::LineMalformed(step))
            },
        _ => Err(ParseError::LineMalformed(step)),
    }
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
        assert_eq!(run(&sample_input), Ok((1320, 145)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((509167, 259333)));
    }
}
