pub fn run(input: &str, rows: usize) -> (usize, usize) {
    let  cols = input.len();
    let mut room = vec![vec![false; cols+2]; rows];
    for (idx, c) in input.chars().enumerate() {
        room[0][idx+1] = parse_tile(c);
    }
    (1..rows).for_each(|y| {
        (1..=cols).for_each(|x| {
            room[y][x] = room[y-1][x-1] ^ room[y-1][x+1];
        });
    });
    let first = cols*rows-room.iter().flatten().filter(|&t| *t).count();
    let second = 0;
    (first, second)
}

fn parse_tile(tile: char) -> bool {
    match tile {
        '.' => false,
        '^' => true,
        _ =>  panic!("Unexpected Token: {tile}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {}", name)[..]).trim().to_string()
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input, 3), (6, 0));
    }

    #[test]
    fn test_sample_large() {
        let sample_input = read_file("tests/sample_input_2");
        assert_eq!(run(&sample_input, 10), (38, 0));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input, 40), (1987, 0));
    }

    #[test]
    fn test_challenge_large() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input, 400_000), (19984714, 0));
    }
}
