use rayon::prelude::*;

pub fn run(input: &str) -> ((usize, usize), (usize, usize, usize)) {
    let grid_serial = input.parse::<i32>().unwrap();
    let grid: Vec<Vec<i32>> = (1..=300).map(|y|
                                    (1..=300).map(|x| cell_charge(x, y, grid_serial))
                                             .collect()
                                         ).collect();
    let first = get_max_charge(&grid, 3).0;
    let second = get_total_max_charge(&grid);
    ((first.0 + 1, first.1 + 1), (second.0.0 + 1, second.0.1 + 1, second.1))
}

fn get_total_max_charge(grid: &[Vec<i32>]) -> ((usize, usize), usize) {
    let mut best = ((0, 0), 1);
    let mut best_val = i32::MIN;
    let mut side_length = 1;
    loop {
        let this = get_max_charge(grid, side_length);
        if this.1 > best_val {
            best_val = this.1;
            best = (this.0, side_length);
            side_length += 1;
        } else if this.1 > 0 {
            side_length += 1;
        } else {
            return best;
        }
    }
}

fn get_max_charge(grid: &[Vec<i32>], side_length: usize) -> ((usize, usize), i32) {
    (0..=300-side_length).into_par_iter()
                        .map(|y| 
                            (0..=300-side_length).map(|x| ((x, y), grid_charge(grid, (x, y), side_length)))
                            .max_by_key(|charge| charge.1).unwrap())
                        .max_by_key(|charge| charge.1)
                        .unwrap()
}

fn grid_charge(grid: &[Vec<i32>], top_left: (usize, usize), side_length: usize) -> i32 {
    (0..side_length).map(|y_offset|
                    (0..side_length).map(|x_offset| grid[top_left.1 + y_offset][top_left.0 + x_offset])
                    .sum::<i32>()
                    ).sum()
}

fn cell_charge(x: i32, y: i32, grid_serial: i32) -> i32 {
    ((((x + 10) * y + grid_serial) * (x + 10) ) % 1000 ) / 100 - 5
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
        assert_eq!(run(&sample_input), ((33, 45), (90, 269, 16)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), ((235, 85), (233, 40, 13)));
    }
}
