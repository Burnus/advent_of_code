use std::collections::HashSet;

pub fn run(input: &str) -> (usize, usize) {
    let (replacements, initial) = parse_input(input);
    let first = get_replacements(&replacements, initial).len();
    let second = find_reduction(&replacements, initial.to_string()).expect("Unable to construct target molecule");
//    let second = a_star_search(initial, "e", &replacements, &str::len);
    (first, second)
}

fn parse_input(input: &str) -> (Vec<(&str, &str)>, &str) {
    let (replacements_str, initial) = input.split_once("\n\n").expect("Unable to split input by blank line");

     (
         replacements_str.lines()
            .map(|line| line.split_once(" => ").unwrap_or_else(|| panic!("unable to split {} by \" => \"", line)))
            .collect(),
        initial.trim()
    )
}
/*
fn a_star_search(start: &str, goal: &str, replacements: &[(&str, &str)], h: &dyn Fn(&str)->usize) -> usize {
    // The set of discovered nodes. Initially only the start node is known.
    let mut open_set = HashSet::from([start.to_string()]);
    // A map from a node to its predecessor on its cheapest known path.
    let mut came_from = HashMap::new();
    // A map from a node to its lowest known costs.
    let mut g_score = HashMap::from([(start.to_string(), 0)]);
    // Estimated costs of each path (f = g+h)
    let mut f_score = HashMap::from([(start.to_string(), h(start))]);

    loop {
        let current = open_set.iter()
            .min_by(|&a, &b| f_score.get(a).unwrap()
                    .cmp(f_score.get(b).unwrap()))
            .unwrap().to_owned();
        if current == goal {
            return reconstruct_path(came_from, &current);
        }
        open_set.remove(&current);
        for neighbour in get_reductions(replacements, current.to_string()) {
            let tentative_g_score = g_score.get(&current).unwrap() + 1;
            let current_g_score = *g_score.get(&neighbour[..]).unwrap_or(&usize::MAX);
            if tentative_g_score < current_g_score {
                came_from.insert(neighbour.to_owned(), current.to_string());
                g_score.insert(neighbour.to_owned(), tentative_g_score);
                f_score.insert(neighbour.to_owned(), tentative_g_score + h(&neighbour));
                open_set.insert(neighbour);
            }
        }
        if open_set.is_empty() {
            break;
        }
    }
    panic!("Open Set is empty, but goal was never reached.")
}

fn reconstruct_path(came_from: HashMap<String, String>, goal: &str) -> usize {
    let mut total_path_len = 0;
    let mut current = goal;
    while let Some(predecessor) = came_from.get(current) {
        total_path_len += 1;
        current = predecessor;
    }
    total_path_len
}
*/

// Always returns the first reduction it finds by trying to shorten the string as much as possible
// as early as possible. This yields the correct results for me, but more hostile inputs probably
// require a more thorough approach, such as the A* algorithm shown above. 
fn find_reduction(replacements: &[(&str, &str)], target: String) -> Option<usize> {
    if target == *"e" {
        Some(0)
    } else {
        let mut next_step: Vec<_> = get_reductions(replacements, target).into_iter().collect();

        next_step.sort_by_key(|a| a.len());
        for next_attemt in next_step {
            let this_attempt = find_reduction(replacements, next_attemt);
            if let Some(score) = this_attempt {
                return Some(score + 1);
            }
        }
        None
    }
}

fn get_reductions(replacements: &[(&str, &str)], target: String) -> HashSet<String> {
    let mut res = HashSet::new();
    for idx in 0..target.len() {
        for (from, to) in replacements {
            if target[idx..].find(*to) == Some(0) {
                res.insert(format!("{}{}{}", &target[..idx].to_string(), from, &target[idx+to.len()..].to_string()));
            }
        }
    }

    res
}

fn get_replacements(replacements: &[(&str, &str)], initial: &str) -> HashSet<String> {
    let mut res = HashSet::new();
    for idx in 0..initial.len() {
        for (from, to) in replacements {
            if initial[idx..].find(*from) == Some(0) {
                res.insert(format!("{}{}{}", &initial[..idx].to_string(), to, &initial[idx+from.len()..].to_string()));
            }
        }
    }

    res
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
        assert_eq!(run(&sample_input), (4, 3));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (535, 212));
    }
}
