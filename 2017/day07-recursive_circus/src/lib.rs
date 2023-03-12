use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
struct Program {
    name: String,
    weight: Option<usize>,
    total_weight: Option<usize>,
    parent: Option<usize>,
}

pub fn run(input: &str) -> (String, usize) {
    let mut towers = parse_input(input);
    let first = towers.iter().find(|prog| prog.parent.is_none()).map(|prog| prog.name.to_string()).unwrap();
    let second = set_total_weights_and_balance(&mut towers);
    (first, second)
}

fn set_total_weights_and_balance(tower: &mut [Program]) -> usize {
    let mut off_weight = None;
    for id in 0..tower.len() {
        let old_total_weight = tower[id].total_weight;
        if old_total_weight.is_none() {
            let new_total_weight = get_total_weight_and_balance(id, tower, &mut off_weight);
            tower[id].total_weight = new_total_weight;
        }
    }
    off_weight.expect("No imballance found")
}

fn get_total_weight_and_balance(id: usize, tower: &mut [Program], off_weight: &mut Option<usize>) -> Option<usize> {
    let this_weight = tower[id].weight.unwrap();
    let child_ids: Vec<usize> = tower.iter_mut()
                        .enumerate()
                        .filter(|(_idx, prog)| prog.parent == Some(id))
                        .map(|(idx, _prog)| idx)
                        .collect();
    if child_ids.is_empty() {
        tower[id].total_weight = Some(this_weight);
    }
    let mut child_weights = Vec::new();
    for child in &child_ids {
        if tower[*child].total_weight.is_none() {
            tower[*child].total_weight = get_total_weight_and_balance(*child, tower, off_weight);
        }
        child_weights.push((*child, tower[*child].total_weight.unwrap()));
    }

    let (min_id, min_child) = *child_weights.iter().min_by_key(|c| c.1).unwrap_or(&(0, 0));
    let (max_id, max_child) = *child_weights.iter().max_by_key(|c| c.1).unwrap_or(&(0, 0));
    if min_child != max_child {
        if child_weights.iter().filter(|c| c.1 == min_child).count() == 1 {
            // min is wrong
            let old_weight = tower[min_id].weight.unwrap();
            let old_total_weight = tower[min_id].total_weight.unwrap();
            let new_weight = old_weight + max_child - min_child;
            let new_total_weight = old_total_weight + max_child - min_child;
            tower[min_id].weight = Some(new_weight);
            tower[min_id].total_weight = Some(new_total_weight);
            *off_weight = Some(new_weight);
        } else {
            // max is wrong
            let old_weight = tower[max_id].weight.unwrap();
            let old_total_weight = tower[max_id].total_weight.unwrap();
            let new_weight = old_weight + min_child - max_child;
            let new_total_weight = old_total_weight + min_child - max_child;
            tower[max_id].weight = Some(new_weight);
            tower[max_id].total_weight = Some(new_total_weight);
            *off_weight = Some(new_weight);
        }
    }
    Some(this_weight + child_ids.iter().map(|id| tower[*id].total_weight.unwrap()).sum::<usize>())
}

fn parse_input(input: &str) -> Vec<Program> {
    let mut programs_list = HashMap::new();
    let mut towers = Vec::new();

    input.lines().for_each(|line| {
        let components: Vec<_> = line.split_whitespace().collect();
        let name = components[0];
        let weight: usize = components[1][1..components[1].len()-1].parse().unwrap();
        let program_count = programs_list.len();
        let id = *programs_list.entry(name).or_insert(program_count);
        if id == program_count {
            towers.push( Program { 
                name: name.to_string(), 
                weight: Some(weight),
                total_weight: None,
                parent: None,
            });
        } else {
            let mut parent_prog: &mut Program = &mut towers[id];
            parent_prog.weight = Some(weight);
        }

        let children_strings = if components.len()>3 {
            components[3..].to_vec()
        } else {
            Vec::new()
        };
        if children_strings.len()>1 {
            for child in &children_strings[..children_strings.len()-1] {
                let child_name = &child[..child.len()-1];
                if let Some(child_id) = programs_list.get(&child_name) {
                    let mut child_prog: &mut Program = &mut towers[*child_id];
                    child_prog.parent = Some(id);
                } else {
                    let child_id = programs_list.len();
                    programs_list.insert(child_name, child_id);
                    towers.push( Program { 
                        name: child_name.to_string(), 
                        weight: None, 
                        total_weight: None,
                        parent: Some(id), 
                    });
                }
            }
        }
        if let Some(&child_name) = children_strings.last() {
            if let Some(child_id) = programs_list.get(&child_name) {
                let mut child_prog: &mut Program = &mut towers[*child_id];
                child_prog.parent = Some(id);
            } else {
                let child_id = programs_list.len();
                programs_list.insert(child_name, child_id);
                towers.push( Program { 
                    name: child_name.to_string(), 
                    weight: None, 
                    total_weight: None,
                    parent: Some(id), 
                });
            }
        }
    });

    towers
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
        assert_eq!(run(&sample_input), ("tknk".to_string(), 60));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), ("uownj".to_string(), 596));
    }
}
