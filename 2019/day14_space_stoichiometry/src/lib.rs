use std::collections::{HashMap, VecDeque};

type Chemical = usize;

struct Reaction {
    input: Vec<(usize, Chemical)>,
    output: (usize, Chemical),
}

impl Reaction {
    fn from(line: &str, chemicals: &mut Vec<String>) -> Self {
        let mut get_chemical = |name: &str| -> usize {
            if let Some(idx) = chemicals.iter().position(|c| c == &name.to_string()) {
                idx
            } else {
                chemicals.push(name.to_string());
                chemicals.len()-1
            }
        };
        let (in_str, out_str) = line.split_once(" => ").unwrap();
        let in_components: Vec<_> = in_str.split(&[' ', ',']).chain([""].into_iter()).collect();
        assert_eq!(in_components.len()%3, 0);
        let out_components: Vec<_> = out_str.split(' ').collect();
        assert_eq!(out_components.len(), 2);
        let output = (out_components[0].parse().unwrap(), get_chemical(out_components[1]));

        let input = in_components.chunks(3).map(|c| (c[0].parse::<usize>().unwrap(), get_chemical(c[1]))).collect();

        Self {
            input,
            output,
        }
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let mut chemicals = Vec::new();
    let reactions: Vec<_> = input.lines().map(|line| Reaction::from(line, &mut chemicals)).collect();
    let fuel = chemicals.iter().position(|chem| chem == &String::from("FUEL")).unwrap();
    let ore =  chemicals.iter().position(|chem| chem == &String::from("ORE")).unwrap();
    // dbg!(&chemicals);
    let first = break_down(&reactions, fuel, ore, 1);
    let second = bisection_find(1000000000000/first, 10000000000000/first, &reactions, fuel, ore, 1000000000000);
    (first, second)
}

fn bisection_find(lower: usize, upper: usize, reactions: &[Reaction], target: usize, raw: usize, stock: usize) -> usize {
    if upper-lower < 2 {
        lower 
    } else {
        let mid = (upper+lower)/2;
        if break_down(reactions, target, raw, mid) > stock {
            bisection_find(lower, mid, reactions, target, raw, stock)
        } else {
            bisection_find(mid, upper, reactions, target, raw, stock)
        }
    }
}

fn break_down(reactions: &[Reaction], target: Chemical, raw: Chemical, amount: usize) -> usize {
    let mut current = VecDeque::from([(amount, target)]);
    let mut leftovers = HashMap::new();

    while !(current.len() == 1 && current[0].1 == raw) {
        let (next_count, next_chem): (usize, Chemical) = current.pop_front().unwrap();
        if next_chem == raw {
            current.push_back((next_count, next_chem));
            continue;
        }
        // dbg!(next_chem);
        let reaction = reactions.iter().find(|r| r.output.1 == next_chem).unwrap();
        let multiplier = (next_count + reaction.output.0 - 1)/reaction.output.0;
        *leftovers.entry(next_chem).or_insert(0) += (reaction.output.0 * multiplier).saturating_sub(next_count);
        // eprintln!("Breaking down {next_count} {next_chem} into");
        for (input_count, input_chem) in &reaction.input {
            let mut required = input_count * multiplier;
                // eprintln!(" {required} {input_chem}");
            if let Some(left) = leftovers.get_mut(input_chem) {
                let consumed = required.min(*left);
                required -= consumed;
                *left -= consumed;
                // eprintln!("  {required} after consuming leftovers. {left} left");
            }
            if required > 0 {
                if let Some(idx) = current.iter().position(|c| c.1 == *input_chem) {
                    current[idx].0 += required;
                } else {
                    current.push_back((required, *input_chem));
                }
            }
        }
    }
    current[0].0
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
        assert_eq!(run(&sample_input), (2210736, 460664));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (1582325, 0));
    }
}
