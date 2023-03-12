use md5::{Md5, Digest};

pub fn run(input: &str) -> (String, usize) {
    let first = get_shortest_path((0, 0), (3, 3), input);
    let second = get_longest_path((0, 0), (3, 3), input);
    (first, second)
}

fn get_longest_path(start: (usize, usize), destination: (usize, usize), seed: &str) -> usize {
    let mut solutions = Vec::from([(start, String::from(seed))]);
    
    let mut longest = 0;
    loop {
        if solutions.is_empty() {
            break;
        }
        let mut next_solutions = Vec::new();
        for solution in solutions {
            let pos = solution.0;
            let key = solution.1;
            if pos == destination {
                longest = key.len() - seed.len();
            } else {
                let mut directions = Vec::new();
                let open = get_doors(&key);
                if pos.1 > 0 && open.0 { directions.push('U'); }
                if pos.1 < destination.1 && open.1 { directions.push('D'); }
                if pos.0 > 0 && open.2 { directions.push('L'); }
                if pos.0 < destination.0 && open.3 { directions.push('R'); }

                for direction in directions {
                    let mut this_key = key.to_string();
                    this_key.push(direction);
                    next_solutions.push((go(pos, direction), this_key));
                }
            }
        }
        solutions = next_solutions;
    }
    longest
}

fn get_shortest_path(start: (usize, usize), destination: (usize, usize), seed: &str) -> String {
    let mut solutions = Vec::from([(start, String::from(seed))]);
    
    loop {
        let mut next_solutions = Vec::new();
        for solution in solutions {
            let pos = solution.0;
            let key = solution.1;
            if pos == destination {
                return key[seed.len()..].to_string();
            }
            let mut directions = Vec::new();
            let open = get_doors(&key);
            if pos.1 > 0 && open.0 { directions.push('U'); }
            if pos.1 < destination.1 && open.1 { directions.push('D'); }
            if pos.0 > 0 && open.2 { directions.push('L'); }
            if pos.0 < destination.0 && open.3 { directions.push('R'); }

            for direction in directions {
                let mut this_key = key.to_string();
                this_key.push(direction);
                next_solutions.push((go(pos, direction), this_key));
            }
        }
        solutions = next_solutions;
    }
}

fn go((x, y): (usize, usize), direction: char) -> (usize, usize) {
    match direction {
        'U' => (x, y-1),
        'D' => (x, y+1),
        'L' => (x-1, y),
        'R' => (x+1, y),
        _ => panic!("Unexpected Direction: {direction}"),
    }
}

/// Returns wether doors are open in directions (U, D, L, R)
fn get_doors(key: &str) -> (bool, bool, bool, bool) {
    let mut hasher = Md5::new();
    hasher.update(key);
    let hash = hasher.finalize();
    ( hash[0] / 16 > 10, hash[0] % 16 > 10, hash[1] / 16 > 10, hash[1] % 16 > 10 )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        let expected = [ 
                ("DDRRRD", 370),
                ("DDUDRLRRUDRD", 492),
                ("DRURDRUDDLLDLUURRDULRLDUUDDDRR", 830)
            ];
        for (idx, input) in sample_input.lines().enumerate() {
            assert_eq!(run(input), (expected[idx].0.to_string(), expected[idx].1));
        }
    }

    #[test]
    fn test_challenge() {
        let mut challenge_input = read_file("tests/challenge_input");
        challenge_input = challenge_input.trim().to_string();
        assert_eq!(run(&challenge_input), ("RDURRDDLRD".to_string(), 526));
    }
}
