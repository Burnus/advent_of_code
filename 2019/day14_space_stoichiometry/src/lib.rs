use std::collections::{HashMap, VecDeque};

type Chemical = usize;

struct Reagent {
    id: usize,
    amount: usize,
}

struct Reaction {
    input: Vec<Reagent>,
    output: Reagent,
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
        let output = Reagent {
            id: get_chemical(out_components[1]),
            amount: out_components[0].parse().unwrap(), 
        };

        let input = in_components.chunks(3).map(|c| Reagent { id: get_chemical(c[1]), amount: c[0].parse::<usize>().unwrap(), }).collect();

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
    let second = bisection_find(1_000_000_000_000/first, 10_000_000_000_000/first, &reactions, fuel, ore, 1_000_000_000_000);
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
    let mut current = VecDeque::from([Reagent {amount, id: target, }]);
    let mut leftovers = HashMap::new();

    while !(current.len() == 1 && current[0].id == raw) {
        let next = current.pop_front().unwrap();
        if next.id == raw {
            current.push_back(next);
            continue;
        }
        let reaction = reactions.iter().find(|r| r.output.id == next.id).unwrap();
        let multiplier = (next.amount + reaction.output.amount - 1)/reaction.output.amount;
        *leftovers.entry(next.id).or_insert(0) += (reaction.output.amount * multiplier).saturating_sub(next.amount);
        for input in &reaction.input {
            let mut required = input.amount * multiplier;
            if let Some(left) = leftovers.get_mut(&input.id) {
                let consumed = required.min(*left);
                required -= consumed;
                *left -= consumed;
            }
            if required > 0 {
                if let Some(idx) = current.iter().position(|c| c.id == input.id) {
                    current[idx].amount += required;
                } else {
                    current.push_back(Reagent { id: input.id, amount: required, });
                }
            }
        }
    }
    current[0].amount
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
        assert_eq!(run(&challenge_input), (1582325, 2267486));
    }
}
