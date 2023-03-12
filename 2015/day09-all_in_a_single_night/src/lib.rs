use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> (usize, usize) {
    let distances = get_distances(input);
    let first  = try_all(&distances, &{|a, b| b.cmp(a)});
    let second = try_all(&distances, &{|a, b| a.cmp(b)});
    (first, second)
}

fn get_distances(input: &str) -> HashMap<(u16, u16), usize> {
    let mut cities = HashMap::new();
    let mut map = HashMap::new();

    input.lines().for_each(|line| {
        let components: Vec<&str> = line.split(' ').collect();
        assert_eq!(components.len(), 5);

        let next = 2_u16.pow(cities.len() as u32);
        let from = *cities.entry(components[0]).or_insert(next);
        let next = 2_u16.pow(cities.len() as u32);
        let to = *cities.entry(components[2]).or_insert(next);
        let distance = components[4].parse().unwrap();

        map.insert((from, to), distance);
        map.insert((to, from), distance);
    });

    map
}

fn try_all<F>(distances: &HashMap<(u16, u16), usize>, comparison: &F) -> usize 
    where F: Fn(&usize, &usize) -> std::cmp::Ordering
{
    let starting_points: HashSet<u16> = distances.keys().map(|(from, _)| *from).collect();

    starting_points.iter()
        .map(|&from| try_all_from(from, distances, from, comparison))
        .max_by(comparison)
        .unwrap()
}

fn try_all_from<F>(current: u16, distances: &HashMap<(u16, u16), usize>, visited: u16, comparison: &F) -> usize
    where F: Fn(&usize, &usize) -> std::cmp::Ordering
{
    distances.keys()
        .filter(|(from, to)| *from == current && *to & visited == 0)
        .map(|(_, to)| distances.get(&(current, *to)).unwrap() + try_all_from(*to, distances, visited | to, comparison))
        .max_by(comparison)
        .unwrap_or(0)
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
        assert_eq!(run(&sample_input), (605, 982));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (117, 909));
    }
}
