use std::num::ParseIntError;

#[derive(Clone, Copy)]
enum FuelRate { Constant, Linear }

pub fn run(input: &str) -> Result<(usize, usize), ParseIntError> {
    let mut positions: Vec<_> = input.split(',').map(|i| i.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
    positions.sort();
    let first = find_closest_alignment(&positions, FuelRate::Constant);
    let second = find_closest_alignment(&positions, FuelRate::Linear);
    Ok((first, second))
}

fn get_sum_of_distances(positions: &[usize], target: usize, fuel_burning_rate: FuelRate) -> usize {
    match fuel_burning_rate {
        FuelRate::Constant => positions.iter().map(|pos| pos.abs_diff(target)).sum(),
        FuelRate::Linear => positions.iter().map(|pos| {
            let n = pos.abs_diff(target);
            (n * (n + 1))/2
        }).sum(),
    }
}

fn find_closest_alignment(positions: &[usize], fuel_burning_rate: FuelRate) -> usize {

    // start with the mean position
    let mut current_pos = positions[positions.len()/2];
    let mut current_dist = get_sum_of_distances(positions, current_pos, fuel_burning_rate);

    // try larger
    loop {
        let next_pos = current_pos+1;
        let next_dist = get_sum_of_distances(positions, next_pos, fuel_burning_rate);
        if next_dist < current_dist {
            current_dist = next_dist;
            current_pos = next_pos
        } else {
            break;
        }
    }

    // try smaller
    loop {
        let next_pos = current_pos-1;
        let next_dist = get_sum_of_distances(positions, next_pos, fuel_burning_rate);
        if next_dist < current_dist {
            current_dist = next_dist;
            current_pos = next_pos
        } else {
            break;
        }
    }

    current_dist
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
        assert_eq!(run(&sample_input), Ok((37, 168)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((347011, 98363777)));
    }
}
