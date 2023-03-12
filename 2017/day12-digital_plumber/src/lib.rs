pub fn run(input: &str) -> (usize, usize) {
    let mut network: Vec<_> = input.lines().map(parse_connections).collect();
    let root_network = get_clique(&mut network, 0);
    let first = root_network.len();
    let mut second = 1;
    while let Some(idx) = network.iter().position(|v| !v.is_empty()) {
        second += 1;
        _ = get_clique(&mut network, idx);
    }
    (first, second)
}

fn parse_connections(line: &str) -> Vec<usize> {
    let (_id, rest) = line.split_once(" <-> ").unwrap();
    rest.split(", ").map(|i| i.parse::<usize>().unwrap()).collect()
}

fn get_clique(network: &mut [Vec<usize>], start: usize) -> Vec<usize> {
    let mut clique = Vec::from([start]);
    let mut last_step = Vec::from([start]);
    while !last_step.is_empty() {
        let mut new_this_step = Vec::new();
        for id in &last_step {
            for neighbour in network[*id].iter() {
                if !new_this_step.contains(neighbour) && clique.binary_search(neighbour).is_err() {
                    new_this_step.push(*neighbour);
                }
            }
            network[*id] = Vec::new();
        }
        clique.append(&mut new_this_step.to_vec());
        clique.sort();
        std::mem::swap(&mut new_this_step, &mut last_step);
    }

    clique
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
        assert_eq!(run(&sample_input), (6, 2));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (169, 179));
    }
}
