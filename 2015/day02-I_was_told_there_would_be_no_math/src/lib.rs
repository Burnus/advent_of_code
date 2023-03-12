use std::fs::read_to_string;

fn read_file(name: &str) -> String {
    read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
}

fn min_2(a: usize, b: usize, c: usize) -> (usize, usize) {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => (a, b.min(c)),
        std::cmp::Ordering::Equal => (a, b.min(c)),
        std::cmp::Ordering::Greater => (b, a.min(c)),
    }
}

fn get_dimensions(line: &str) -> (usize, usize, usize) { 
    let sides: Vec<&str> = line.split('x').collect();
    assert_eq!(sides.len(), 3);
    (sides[0].parse::<usize>().unwrap(), sides[1].parse::<usize>().unwrap(), sides[2].parse::<usize>().unwrap())
}

fn get_wrapping_paper((l, w, h): (usize, usize, usize)) -> usize {
    let (short, mid) = min_2(l, w, h);

    2*l*w + 2*w*h + 2*h*l + short*mid
}

fn get_ribbon((l, w, h): (usize, usize, usize)) -> usize {
    let (short, mid) = min_2(l, w, h);

    2*(short+mid) + l*w*h
}

pub fn run(input: &str) -> (usize, usize ) {
    let dimensions: Vec<_> = input.lines().map(get_dimensions).collect();
    let first = dimensions.iter().cloned().map(get_wrapping_paper).sum();
    let second = dimensions.iter().cloned().map(get_ribbon).sum();
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), (101, 48));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (1598415, 3812909));
    }
}
