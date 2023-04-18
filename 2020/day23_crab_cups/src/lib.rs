use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ParseIntError(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let arrangement: Vec<_> = input.chars().map(|c| c.to_digit(10).ok_or(ParseError::ParseIntError(c)).map(|i| i as usize)).collect::<Result<Vec<_>, _>>()?;
    let mut cups = (1..=9).map(|i| arrangement[(arrangement.iter().position(|cup| *cup == i).unwrap()+1) % arrangement.len()]).collect::<Vec<_>>();
    let mut next = arrangement[0];
    let mut cups_2 = cups.clone();
    cups_2[arrangement[arrangement.len()-1]-1] = cups.len()+1;
    cups_2.append(&mut (cups.len()+2..=1_000_000).collect());
    cups_2.push(next);

    for _ in 0..100 {
        play(&mut cups, &mut next);
    }

    let mut first = 0;
    let mut next_digit = cups[0];
    while next_digit != 1 {
        first *= 10;
        first += next_digit;
        next_digit = cups[next_digit-1];
    }

    next = arrangement[0];
    for _ in 0..10_000_000 {
        play(&mut cups_2, &mut next);
    }

    let second = cups_2[0] * cups_2[cups_2[0]-1];
    Ok((first, second))
}

fn play(cups: &mut Vec<usize>, current: &mut usize) {
    let moved = [
        cups[*current-1],
        cups[cups[*current-1]-1],
        cups[cups[cups[*current-1]-1]-1]
    ];
    let mut target = *current - 1;
    loop {
        if target < 1 {
            target = cups.len();
        } else if moved.contains(&target) {
            target -= 1;
        } else {
            break;
        }
    }
    cups[*current-1] = cups[moved[2]-1];
    cups[moved[2]-1] = cups[target-1];
    cups[target-1] = moved[0];

    *current = cups[*current-1];
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
        assert_eq!(run(&sample_input), Ok((67384529, 149245887792)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((82934675, 474600314018)));
    }
}
