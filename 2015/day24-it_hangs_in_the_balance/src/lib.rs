pub fn run(input: &str) -> (usize, usize) {
    let mut presents: Vec<_> = input.lines().map(|i| i.parse::<usize>().unwrap()).collect();
    presents.sort();
    presents.reverse();
    let first = min1_distributions(&presents, 3).iter().map(|list| list.iter().product()).min().unwrap();
    let second = min1_distributions(&presents, 4).iter().map(|list| list.iter().product()).min().unwrap();
    (first, second)
}

fn min1_distributions(presents: &[usize], packages: usize) -> Vec<Vec<usize>> {
    let target = presents.iter().sum::<usize>() / packages;
    get_distributions(presents, target, usize::MAX, packages)
}

fn can_be_ballanced(presents: &[usize], target: usize, packages: usize) -> bool {
    if packages == 2 {
        presents.iter()
            .enumerate()
            .filter(|(_, &p)| p <= target)
            .any(|(present_idx, present)| {
                let mut presents_left = presents.to_vec();
                presents_left.remove(present_idx);
                *present == target || can_be_ballanced(&presents_left, target-*present, packages) 
            } )
    } else {
        presents.iter()
            .enumerate()
            .filter(|(_, &p)| p <= target)
            .any(|(present_idx, present)| {
                let mut presents_left = presents.to_vec();
                presents_left.remove(present_idx);
                (*present == target && can_be_ballanced(&presents_left, presents_left.iter().sum::<usize>()/(packages-1), packages-1)) || can_be_ballanced(&presents_left, target-*present, packages)
            } )
    }
}

fn get_distributions(presents: &[usize], target: usize, max_left: usize, packages: usize) -> Vec<Vec<usize>> {
    if max_left == 0 {
        return Vec::new();
    }
    let mut best_distributions = Vec::new();
    let mut max_left = max_left;
    for (present_idx, &present) in presents.iter().enumerate() {
        let mut presents_left = presents.to_vec();
        presents_left.remove(present_idx);
        if present == target {
            if can_be_ballanced (&presents_left, presents_left.iter().sum::<usize>()/(packages-1), packages-1) {
                return vec![vec![target]];
            } else {
                continue;
            }
        }
        if present < target {
            let mut these_distributions = get_distributions(&presents_left, target-present, max_left-1, packages);
            if these_distributions.is_empty() {
                continue;
            }
            match these_distributions[0].len() {
                m if m == max_left-1 => {
                        these_distributions.iter_mut().for_each(|d| {
                            d.push(present);
                            best_distributions.push(d.to_vec());
                        });
                    },
                l if l < max_left-1 => {
                        max_left = l+1; 
                        these_distributions.iter_mut().for_each(|d| d.push(present));
                        best_distributions = these_distributions;
                    },
                _ => (),
            }
        }
    }
    best_distributions
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
        assert_eq!(run(&sample_input), (99, 44));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (11846773891, 80393059));
    }
}
