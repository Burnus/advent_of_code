use std::fs;
use day12_hill_climbing_algorithm::*;

/// Reads the file at the given path from the file system and returns its contents as a String.
fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn get_length<F>(dest_network: &[Vec<Position>], start_condition: F) -> usize where
    F: Fn(&Position) -> bool + Copy {
    dest_network.iter()
        .enumerate()
        .find(|(_length, positions)| positions.iter().any(start_condition))
        .unwrap()
        .0
}

fn main() {
    //let map = read_file("sample_input");
    let map = read_file("input");

    let (grid, start, end, max) = parse(&map);

    let end_position = Position::from(25, end, max);
    let dest_network = get_network_to(end_position, &grid);

    let start_finish_length = get_length(&dest_network, |position| position.coordinate() == start);
    println!("We can get from start to end in {} steps.", start_finish_length);

    let shortest_scenic = get_length(&dest_network, |position| position.height() == 0);
    println!("The shortest scenic route is {} steps long.", shortest_scenic);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_input_matches_challenge() {
        let map = read_file("tests/sample_input");
        let (grid, start, end, max) = parse(&map);

        let end_position = Position::from(25, end, max);
        let dest_network = get_network_to(end_position, &grid);

        let start_finish_length = get_length(&dest_network, |position| position.coordinate() == start);
        assert_eq!(start_finish_length, 31);

        let shortest_scenic = get_length(&dest_network, |position| position.height() == 0);
        assert_eq!(shortest_scenic, 29)
    }

    #[test]
    fn challenge_input_matches_solution() {
        let map = read_file("tests/input");
        let (grid, start, end, max) = parse(&map);

        let end_position = Position::from(25, end, max);
        let dest_network = get_network_to(end_position, &grid);

        let start_finish_length = get_length(&dest_network, |position| position.coordinate() == start);
        assert_eq!(start_finish_length, 425);

        let shortest_scenic = get_length(&dest_network, |position| position.height() == 0);
        assert_eq!(shortest_scenic, 418)
    }
}
