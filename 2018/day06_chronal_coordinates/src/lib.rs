use std::collections::{HashSet, HashMap};

pub fn run(input: &str, safe_distance: isize) -> (usize, usize) {
    let list: Vec<_> = input.lines().map(|line| line.split_once(", ").unwrap()).collect();
    let coordinates: Vec<_> = list.iter().map(|s| (s.0.parse::<isize>().unwrap(), s.1.parse::<isize>().unwrap())).collect();
    let areas = get_areas(&coordinates);
    // dbg!(&areas);
    let first = areas.iter().filter(|a| **a < usize::MAX).max().unwrap();
    let second = safe_area(&coordinates, safe_distance);
    (*first, second)
}

fn safe_area(coordinates: &[(isize, isize)], safe_distance: isize) -> usize {
    let first = coordinates[0];
    let mut area = 0;
    (first.0-safe_distance/4..first.0+safe_distance/4).for_each(|x| {
        (first.1-safe_distance/4..first.1+safe_distance/4).for_each(|y| {
            if coordinates.iter().map(|c| c.0.abs_diff(x) + c.1.abs_diff(y)).sum::<usize>() < safe_distance as usize {
                area += 1;
            }
        });
    });
    area
}

fn get_areas(coordinates: &[(isize, isize)]) -> Vec<usize> {
    let mut found = HashSet::new();
    for c in coordinates { found.insert(*c); }
    let mut sizes = vec![1; coordinates.len()];
    let mut last_step: Vec<Vec<(isize, isize)>> = coordinates.iter().map(|c| Vec::from([*c])).chain(Vec::new()).collect();
    for _ in 0..500 {
        let mut found_this_step = HashMap::new();
        for (origin_idx, origin) in last_step.iter().enumerate() {
            for open in origin {
                for neighbour_offset in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let neighbour = (open.0 + neighbour_offset.0, open.1 + neighbour_offset.1);
                    if !found.contains(&neighbour) {
                        found_this_step.entry(neighbour).and_modify(|o| if *o != origin_idx { *o = last_step.len()-1; }).or_insert(origin_idx);
                    }
                }
            }
        } 
        for v in &mut last_step {
            *v = Vec::new();
        }
        for (coords, origin) in found_this_step {
            last_step[origin].push(coords);
            if origin < sizes.len()-1 {
                sizes[origin] += 1;
            }
            found.insert(coords);
        }
    }
    last_step.iter().take(coordinates.len()-1).enumerate().filter(|(_idx, v)| !v.is_empty()).map(|(idx, _v)| idx).for_each(|idx| sizes[idx] = usize::MAX);

    sizes
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
        assert_eq!(run(&sample_input, 32), (17, 16));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input, 10_000), (3251, 47841));
    }
}
