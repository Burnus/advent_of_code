use std::collections::{HashMap, HashSet};

pub fn run(input: usize, start: (usize, usize), goal: (usize, usize)) -> (usize, usize) {
    let first = count_steps(start, goal, input);
    let second = get_all(HashSet::from([start]), HashSet::from([start]), input, 50).len();
    (first, second)
}

fn get_all(current: HashSet<(usize, usize)>, new_last_step: HashSet<(usize, usize)>, input: usize, steps_left: usize) -> HashSet<(usize, usize)> {
    if steps_left == 0 {
        return current;
    }
    let new_this_step: HashSet<(usize, usize)> = new_last_step.iter().flat_map(|coord| get_neighbours(*coord, input).into_iter().filter(|neighbour| !current.contains(neighbour))).collect();
    get_all(current.union(&new_this_step).cloned().collect(), new_this_step, input, steps_left-1)
}

fn is_open_space((x, y): (usize, usize), input: usize) -> bool {
    (x*x + 3*x + 2*x*y + y + y*y + input).count_ones() % 2 == 0
}

fn get_neighbours(current: (usize, usize), input: usize) -> Vec<(usize, usize)> {
    let mut res = vec![(current.0, current.1 + 1), (current.0 + 1, current.1)];
    if current.0 > 0 {
        res.push((current.0-1, current.1));
    }
    if current.1 > 0 {
        res.push((current.0, current.1-1));
    }
    res.into_iter().filter(|&coords| is_open_space(coords, input)).collect()
}

// A* search
fn count_steps(start: (usize, usize), goal: (usize, usize), input: usize) -> usize {
    let mut open_set = HashSet::from([start]);
    let mut g_scores = HashMap::from([(start, 0)]);
    let mut f_scores = HashMap::from([(start, start.0.abs_diff(goal.0)+start.1.abs_diff(goal.1))]);
    while !open_set.is_empty() {
        let current = *open_set.iter().min_by_key(|c| f_scores.get(c).unwrap()).unwrap();
        open_set.remove(&current);
        let current_g = *g_scores.get(&current).unwrap();
        if current == goal {
            return current_g;
        }
        for neighbour in get_neighbours(current, input) {
            let tentative_g_score = current_g + 1;
            if g_scores.get(&neighbour).unwrap_or(&usize::MAX) > &tentative_g_score {
                g_scores.insert(neighbour, tentative_g_score);
                open_set.insert(neighbour);
                f_scores.insert(neighbour, tentative_g_score + neighbour.0.abs_diff(goal.0) + neighbour.1.abs_diff(goal.1));
            }
        }
    }
    panic!("No solution found");
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
        let sample_input = read_file("tests/sample_input").trim().parse().unwrap();
        assert_eq!(run(sample_input, (1, 1), (7, 4)), (11, 151));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input").trim().parse().unwrap();
        assert_eq!(run(challenge_input, (1, 1), (31, 39)), (90, 135));
    }
}
