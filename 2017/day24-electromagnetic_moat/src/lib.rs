#[derive(Clone)]
struct Part {
    port_a: usize,
    port_b: usize,
}

impl Part {
    fn strength(&self) -> usize {
        self.port_a + self.port_b
    }
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let (a, b) = value.split_once('/').unwrap();
        Self {
            port_a: a.parse().unwrap(),
            port_b: b.parse().unwrap(),
        }
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let bridge_strength = |bridge: &Vec<Part>| -> usize { bridge.iter().map(|part| part.strength()).sum() };
    let parts: Vec<_> = input.lines().map(Part::from).collect();
    let all_bridges = all_bridges(&parts, 0);
    let first = all_bridges.iter().map(bridge_strength).max().unwrap();
    let longest = all_bridges.iter().max_by(|a, b| match a.len().cmp(&b.len()) {
            std::cmp::Ordering::Equal => bridge_strength(a).cmp(&bridge_strength(b)),
            different => different,
        }).unwrap();
    let second = longest.iter().map(|part| part.strength()).sum();
    (first, second)
}

fn all_bridges(parts_remaining: &[Part], current: usize) -> Vec<Vec<Part>> {
    let mut remaining_bridges = Vec::new();
    for (idx, next) in parts_remaining.iter().enumerate().filter(|(_idx, p)| p.port_a == current || p.port_b == current) {
        let this_strength = next.strength();
        let next_current = this_strength - current;
        let mut new_remaining = parts_remaining.to_vec();
        new_remaining.remove(idx);
        all_bridges(&new_remaining, next_current).iter_mut().for_each(|bridge| {
            bridge.push(next.clone());
            remaining_bridges.push(bridge.clone());
        });
        remaining_bridges.push(Vec::from([next.clone()]));
    }
    remaining_bridges
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
        assert_eq!(run(&sample_input), (31, 19));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (1906, 1824));
    }
}
