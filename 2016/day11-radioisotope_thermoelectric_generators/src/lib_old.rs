use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
enum Component { Chip(u8), Generator(u8), Elevator }

impl Component {
    fn new(element: &str, component_type: &str, elements: &mut HashMap<String, u8>) -> Self {
        let next_element = elements.len() as u8;
        match &component_type[..component_type.len()-1] {
            "generato" | "generator" => Self::Generator(*elements.entry(element.to_string()).or_insert(next_element)),
            "microchi" | "microchip" => Self::Chip(*elements.entry(element.split('-').next().unwrap().to_string()).or_insert(next_element)),
            _ => panic!("Unable to construct Component {element} {component_type}"),
        }
    }

    fn corresponding_generator(&self) -> Self {
        if let Self::Chip(s) = self {
            Self::Generator(*s)
        } else {
            panic!("There is no corresponding generator for {self:?}");
        }
    }
}


pub fn run(input: &str) -> (usize, usize) {
    let mut elements = HashMap::new();
    let mut compound: Vec<_> = input.lines().map(|line| parse_line(line, &mut elements)).collect();
    compound[0].push(Component::Elevator);
    let first = a_star_search(compound.as_slice());
    compound[0].pop();
    compound[0].append(&mut vec![Component::Chip(elements.len() as u8), Component::Generator(elements.len() as u8), Component::Chip(elements.len() as u8 + 1), Component::Generator(elements.len() as u8 + 1)]);
    compound[0].sort();
    compound[0].push(Component::Elevator);
    let second = a_star_search(compound.as_slice());
    (first, second)
}

fn is_allowed(compound: &[Vec<Component>]) -> bool {
    for floor in compound {
        let chips_this_floor: Vec<_> = floor.iter().filter(|c| matches!(c, Component::Chip(_))).collect();
        let generators_this_floor: Vec<_> = floor.iter().filter(|c| matches!(c, Component::Generator(_))).collect();
        if generators_this_floor.is_empty() {
            continue;
        }
        for chip in chips_this_floor {
            if !generators_this_floor.contains(&&chip.corresponding_generator()) {
                // We destroyed some generators
                return false;
            }
        }
    }
    true
}

fn get_neighbours(current: &[Vec<Component>]) -> Vec<Vec<Vec<Component>>> {
    let floor_number = current.iter().position(|f| f.contains(&Component::Elevator)).unwrap();
    let floor_items = &current[floor_number];
    let mut res = Vec::new();
    for (item_1_idx, item_1) in floor_items.iter().enumerate().take(floor_items.len()-1) {
        if floor_number > 0 {
            let mut new_compound = current.to_vec();
            new_compound[floor_number-1].push(item_1.clone());
            new_compound[floor_number-1].sort();
            new_compound[floor_number-1].push(Component::Elevator);
            new_compound[floor_number].pop();
            new_compound[floor_number].remove(item_1_idx);
            if is_allowed(&new_compound) {
                res.push(new_compound.to_vec());
            }
            for (item_2_idx, item_2) in floor_items.iter().enumerate().skip(item_1_idx+1).take(floor_items.len()-item_1_idx-2) {
                let mut new_compound = new_compound.to_vec();
                new_compound[floor_number-1].pop();
                new_compound[floor_number-1].push(item_2.clone());
                new_compound[floor_number-1].sort();
                new_compound[floor_number-1].push(Component::Elevator);
                new_compound[floor_number].remove(item_2_idx-1);
                if is_allowed(&new_compound) {
                    res.push(new_compound);
                }
            }
        }
        if floor_number < current.len()-1 {
            let mut new_compound = current.to_vec();
            new_compound[floor_number+1].push(item_1.clone());
            new_compound[floor_number+1].sort();
            new_compound[floor_number+1].push(Component::Elevator);
            new_compound[floor_number].pop();
            new_compound[floor_number].remove(item_1_idx);
            if is_allowed(&new_compound) {
                res.push(new_compound.to_vec());
            }
            for (item_2_idx, item_2) in floor_items.iter().enumerate().skip(item_1_idx+1).take(floor_items.len()-item_1_idx-2) {
                let mut new_compound = new_compound.to_vec();
                new_compound[floor_number+1].pop();
                new_compound[floor_number+1].push(item_2.clone());
                new_compound[floor_number+1].sort();
                new_compound[floor_number+1].push(Component::Elevator);
                new_compound[floor_number].remove(item_2_idx-1);
                if is_allowed(&new_compound) {
                    res.push(new_compound);
                }
            }
        }
    }
    res
}

fn a_star_search(start: &[Vec<Component>]) -> usize {
    let mut goal = [vec![], vec![], vec![], start.iter().flatten().filter(|c| !matches!(c, Component::Elevator)).cloned().collect()];
    goal.sort();
    goal[3].push(Component::Elevator);
    let mut open_set = HashSet::from([start.to_vec()]);
    let mut open_set_back = HashSet::from([goal.to_vec()]);
    let mut g_score = HashMap::from([(start.to_vec(), 0)]);
    let mut g_score_back = HashMap::from([(goal.to_vec(), 0)]);

    loop {
        let current = open_set.iter()
            .min_by(|&a, &b| g_score.get(a).unwrap()
                    .cmp(g_score.get(b).unwrap()))
            .unwrap().to_owned();
        if let Some(score) = g_score_back.get(&current) {
            return *g_score.get(&current).unwrap() + score;
        }
        open_set.remove(&current);
        for neighbour in get_neighbours(current.as_slice()) {
            let tentative_g_score = g_score.get(&current).unwrap() + 1;
            let current_g_score = *g_score.get(&neighbour[..]).unwrap_or(&usize::MAX);
            if tentative_g_score < current_g_score {
                g_score.insert(neighbour.to_owned(), tentative_g_score);
                open_set.insert(neighbour.to_vec());
            }
        }
        if open_set.is_empty() {
            break;
        }
        let current = open_set_back.iter()
            .min_by(|&a, &b| g_score_back.get(a).unwrap()
                    .cmp(g_score_back.get(b).unwrap()))
            .unwrap().to_owned();
        if let Some(score) = g_score.get(&current) {
            return *g_score_back.get(&current).unwrap() + score;
        }
        open_set_back.remove(&current);
        for neighbour in get_neighbours(current.as_slice()) {
            let tentative_g_score = g_score_back.get(&current).unwrap() + 1;
            let current_g_score = *g_score_back.get(&neighbour[..]).unwrap_or(&usize::MAX);
            if tentative_g_score < current_g_score {
                g_score_back.insert(neighbour.to_owned(), tentative_g_score);
                open_set_back.insert(neighbour.to_vec());
            }
        }
        if open_set_back.is_empty() {
            break;
        }
    }
    panic!("Open Set is empty, but goal was never reached.")
}

fn parse_line(line: &str, elements: &mut HashMap<String, u8>) -> Vec<Component> {
    let components: Vec<_> = line.split(' ').collect();
    match components.len() {
        6 => Vec::new(),
        7 => Vec::from([Component::new(components[5], components[6], elements)]),
        x if x > 10 => {
                let mut res = Vec::new();
                for idx in 0..x/3-2 {
                    res.push(Component::new(components[3*idx+5], components[3*idx+6], elements));
                }
                res.push(Component::new(components[x-2], components[x-1], elements));
                res.sort();
                res
            },
        _ => panic!("Unable to parse {line}"),
    }
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
        assert_eq!(run(&sample_input), (11, 0));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (37, 0));
    }
}
