use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Component { Chip(u8), Generator(u8) }

impl Component {
    fn new(element: &str, component_type: &str, elements: &mut HashMap<String, u8>) -> Self {
        let next_element = elements.len() as u8;
        match &component_type[..component_type.len()-1] {
            "generato" | "generator" => Self::Generator(*elements.entry(element.to_string()).or_insert(next_element)),
            "microchi" | "microchip" => Self::Chip(*elements.entry(element.split('-').next().unwrap().to_string()).or_insert(next_element)),
            _ => panic!("Unable to construct Component {element} {component_type}"),
        }
    }
}


pub fn run(input: &str) -> (usize, usize) {
    let mut elements = HashMap::new();
    let compound: Vec<_> = input.lines().map(|line| parse_line(line, &mut elements)).collect();
    let mut items: Vec<_> = Vec::new();
    elements.values().for_each(|el| {
        items.push((compound.iter().position(|floor| floor.contains(&Component::Chip(*el))).unwrap(), compound.iter().position(|floor| floor.contains(&Component::Generator(*el))).unwrap()));
    });
    items.sort();
    let elevator = 0;
    let mut goal = vec![(compound.len()-1, compound.len()-1); items.len()];
    let first = a_star_search((elevator, &items), (goal[0].0, goal.to_vec()));
    items.append(&mut vec![(0,0); 2]);
    goal.append(&mut vec![(compound.len()-1, compound.len()-1); 2]);
    let second = a_star_search((elevator, &items), (goal[0].0, goal));
    (first, second)
}

fn is_allowed(current: &[(usize, usize)]) -> bool {
    let unpaired: Vec<_> = current.iter().filter(|(chip, gen)| chip != gen).collect();
    !unpaired.iter().any(|(chip, _)| current.iter().any(|(_, gen)| chip == gen))
}

fn get_neighbours(current: (usize, &Vec<(usize, usize)>), goal_floor: usize) -> Vec<(usize, Vec<(usize, usize)>)> {
    let elevator = current.0;
    let mut res = Vec::new();
    if elevator > 0 {
        for (idx_1, (chip_1, generator_1)) in current.1.iter().enumerate() {
            if *chip_1 == elevator {
                let mut this_neighbour = (current.0, current.1.to_vec());
                this_neighbour.0 -= 1;
                this_neighbour.1[idx_1].0 -= 1;
                this_neighbour.1.sort();
                if !res.contains(&this_neighbour) && is_allowed(&this_neighbour.1) {
                    res.push((this_neighbour.0, this_neighbour.1.to_vec()));
                }
                for (idx_2, (chip_2, generator_2)) in this_neighbour.1.iter().enumerate() {
                    if *chip_2 == elevator {
                        let mut this_neighbour = this_neighbour.clone();
                        this_neighbour.1[idx_2].0 -= 1;
                        this_neighbour.1.sort();
                        if !res.contains(&this_neighbour) && is_allowed(&this_neighbour.1) {
                            res.push(this_neighbour);
                        }
                    }
                    if *generator_2 == elevator {
                        let mut this_neighbour = this_neighbour.clone();
                        this_neighbour.1[idx_2].1 -= 1;
                        this_neighbour.1.sort();
                        if !res.contains(&this_neighbour) && is_allowed(&this_neighbour.1) {
                            res.push(this_neighbour);
                        }
                    }
                }
            }
            if *generator_1 == elevator {
                let mut this_neighbour = (current.0, current.1.to_vec());
                this_neighbour.0 -= 1;
                this_neighbour.1[idx_1].1 -= 1;
                this_neighbour.1.sort();
                if !res.contains(&this_neighbour) && is_allowed(&this_neighbour.1) {
                    res.push((this_neighbour.0, this_neighbour.1.to_vec()));
                }
                for (idx_2, (chip_2, generator_2)) in this_neighbour.1.iter().enumerate() {
                    if *chip_2 == elevator {
                        let mut this_neighbour = this_neighbour.clone();
                        this_neighbour.1[idx_2].0 -= 1;
                        this_neighbour.1.sort();
                        if !res.contains(&this_neighbour) && is_allowed(&this_neighbour.1) {
                            res.push(this_neighbour);
                        }
                    }
                    if *generator_2 == elevator {
                        let mut this_neighbour = this_neighbour.clone();
                        this_neighbour.1[idx_2].1 -= 1;
                        this_neighbour.1.sort();
                        if !res.contains(&this_neighbour) && is_allowed(&this_neighbour.1) {
                            res.push(this_neighbour);
                        }
                    }
                }
            }
        }
    }
    if elevator < goal_floor {
        for (idx_1, (chip_1, generator_1)) in current.1.iter().enumerate() {
            if *chip_1 == elevator {
                let mut this_neighbour = (current.0, current.1.to_vec());
                this_neighbour.0 += 1;
                this_neighbour.1[idx_1].0 += 1;
                this_neighbour.1.sort();
                if !res.contains(&this_neighbour) && is_allowed(&this_neighbour.1) {
                    res.push((this_neighbour.0, this_neighbour.1.to_vec()));
                }
                for (idx_2, (chip_2, generator_2)) in this_neighbour.1.iter().enumerate() {
                    if *chip_2 == elevator {
                        let mut this_neighbour = this_neighbour.clone();
                        this_neighbour.1[idx_2].0 += 1;
                        this_neighbour.1.sort();
                        if !res.contains(&this_neighbour) && is_allowed(&this_neighbour.1) {
                            res.push(this_neighbour);
                        }
                    }
                    if *generator_2 == elevator {
                        let mut this_neighbour = this_neighbour.clone();
                        this_neighbour.1[idx_2].1 += 1;
                        this_neighbour.1.sort();
                        if !res.contains(&this_neighbour) && is_allowed(&this_neighbour.1) {
                            res.push(this_neighbour);
                        }
                    }
                }
            }
            if *generator_1 == elevator {
                let mut this_neighbour = (current.0, current.1.to_vec());
                this_neighbour.0 += 1;
                this_neighbour.1[idx_1].1 += 1;
                this_neighbour.1.sort();
                if !res.contains(&this_neighbour) && is_allowed(&this_neighbour.1) {
                    res.push((this_neighbour.0, this_neighbour.1.to_vec()));
                }
                for (idx_2, (chip_2, generator_2)) in this_neighbour.1.iter().enumerate() {
                    if *chip_2 == elevator {
                        let mut this_neighbour = this_neighbour.clone();
                        this_neighbour.1[idx_2].0 += 1;
                        this_neighbour.1.sort();
                        if !res.contains(&this_neighbour) && is_allowed(&this_neighbour.1) {
                            res.push(this_neighbour);
                        }
                    }
                    if *generator_2 == elevator {
                        let mut this_neighbour = this_neighbour.clone();
                        this_neighbour.1[idx_2].1 += 1;
                        this_neighbour.1.sort();
                        if !res.contains(&this_neighbour) && is_allowed(&this_neighbour.1) {
                            res.push(this_neighbour);
                        }
                    }
                }
            }
        }
    }
    res
}

fn h_score(current: &[(usize, usize)], goal_floor: usize) -> usize {
    goal_floor*current.len() - current.iter().map(|(a, b)| a+b).sum::<usize>()/2
}

fn a_star_search(start: (usize, &[(usize, usize)]), goal: (usize, Vec<(usize, usize)>)) -> usize {
    let mut open_set = HashSet::from([(start.0, start.1.to_vec())]);
    let mut g_score = HashMap::from([((start.0, start.1.to_vec()), 0)]);
    let mut f_score = HashMap::from([((start.0, start.1.to_vec()), h_score(start.1, goal.0))]);

    loop {
        let current = open_set.iter()
            .min_by(|&a, &b| f_score.get(a).unwrap()
                    .cmp(f_score.get(b).unwrap()))
            .unwrap().to_owned();
        if current == goal {
            return *g_score.get(&current).unwrap();
        }
        open_set.remove(&current);
        for neighbour in get_neighbours((current.0, &current.1), goal.0) {
            let neighbour = (neighbour.0, neighbour.1.to_vec());
            let tentative_g_score = g_score.get(&current).unwrap() + 1;
            let current_g_score = *g_score.get(&neighbour).unwrap_or(&usize::MAX);
            if tentative_g_score < current_g_score {
                g_score.insert(neighbour.to_owned(), tentative_g_score);
                f_score.insert(neighbour.to_owned(), tentative_g_score + h_score(&neighbour.1, goal.0));
                open_set.insert(neighbour);
            }
        }
        if open_set.is_empty() {
            break;
        }
    }
    // Open Set is empty but goal was never reached. This means there is no solution.
    usize::MAX
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
        assert_eq!(run(&sample_input), (11, usize::MAX));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (37, 61));
    }
}
