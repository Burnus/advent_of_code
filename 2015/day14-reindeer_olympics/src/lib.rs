pub fn run(input: &str, finish_time: usize) -> (usize, usize) {
    let reindeers: Vec<_> = input.lines().map(get_speeds).collect();
    let first = reindeers.iter().map(|r| distance_at(*r, finish_time)).max().unwrap();
    let mut points = vec![0; reindeers.len()];
    for t in 1..=finish_time {
        let max = reindeers.iter().map(|r| distance_at(*r, t)).max().unwrap();
        reindeers.iter().enumerate().filter(|&(_idx, r)| distance_at(*r, t) == max).for_each(|(idx, _r)| points[idx] += 1);
    }
    let second = *points.iter().max().unwrap();
    (first, second)
}

fn distance_at((speed, travel_time, rest_time): (usize, usize, usize), finish_time: usize) -> usize {
    let full_sorties = finish_time / (travel_time + rest_time);
    let last_partial = finish_time % (travel_time + rest_time);
    full_sorties * travel_time * speed + last_partial.min(travel_time) * speed
}

fn get_speeds(line: &str) -> (usize, usize, usize) {
    let components: Vec<_> = line.split(' ').collect();
    assert_eq!(components.len(), 15);
    (
        components[3].parse().unwrap(),
        components[6].parse().unwrap(),
        components[13].parse().unwrap(),
    )
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
        assert_eq!(run(&sample_input, 1000), (1120, 689));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input, 2503), (2660, 1256));
    }
}
