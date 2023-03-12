use std::{f64::consts::PI, collections::HashMap};

pub fn run(input: usize) -> (usize, usize) {
    let coords = spiral_location(input);
    let first = coords.0.unsigned_abs() + coords.1.unsigned_abs();
    let mut second_data = HashMap::from([((0, 0), 1)]);
    let mut old_coords = (0, 0);
    for index in 2.. {
        let mut to_coords = (old_coords.0 +(((((4*index-7) as f64).sqrt()-0.5).round() * PI/2.0).sin().round() as isize), old_coords.1 + ((((4*index-7) as f64).sqrt() - 0.5).round() * PI/2.0).cos().round() as isize);
        let sum = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)].iter()
            .map(|direction| second_data.get(&(to_coords.0 + direction.0, to_coords.1 + direction.1)).unwrap_or(&0))
            .sum();
        if sum > input {
            return (first, sum);
        }
        second_data.insert(to_coords, sum);
        std::mem::swap(&mut old_coords, &mut to_coords);
    }
    unreachable!("The loop always executes");
}

fn spiral_location(number: usize) -> (isize, isize) {
    (get_x_coord(number), get_y_coord(number))
}

fn get_x_coord(number: usize) -> isize {
    (2..=number).map(|i| (((4.0 * i as f64 - 7.0).sqrt() - 0.5).round() * PI/2.0).sin())
        .sum::<f64>()
        .round() as isize
}

fn get_y_coord(number: usize) -> isize {
    (2..=number).map(|i| (((4.0 * i as f64 - 7.0).sqrt() - 0.5).round() * PI/2.0).cos())
        .sum::<f64>()
        .round() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = [
            (1,(0, 2)),
            (12,(3, 23)),
            (23,(2, 25)),
            (1024,(31, 1968)),
        ];
        for (sample_input, sample_result) in sample {
           assert_eq!(run(sample_input), sample_result);
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = 325489;
        assert_eq!(run(challenge_input), (552, 330785));
    }
}
