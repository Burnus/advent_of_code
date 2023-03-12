struct Layer {
    depth: usize,
    range: usize,
}

impl Layer {
    fn parse(line: &str) -> Self {
        let (depth_str, range_str) = line.split_once(": ").unwrap();
        
        Self {
            depth: depth_str.parse().unwrap(),
            range: range_str.parse().unwrap(),
        }
    }

    fn position(&self, time: usize) -> usize {
        let range = self.range;
        match time % (2*range-2) {
            fwd if fwd < range => fwd,
            t => 2*range-t-2,
        }
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let firewall: Vec<_> = input.lines().map(Layer::parse).collect();
    let first = firewall.iter()
        .filter(|layer| layer.position(layer.depth) == 0)
        .map(|layer| layer.depth * layer.range)
        .sum();
    let second = (0..).find(|i| 
            firewall.iter().all(|layer| layer.position(layer.depth + i) != 0)
        ).unwrap();
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
        assert_eq!(run(&sample_input), (24, 10));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (1900, 3966414));
    }
}
