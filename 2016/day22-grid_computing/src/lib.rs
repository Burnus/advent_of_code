use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    x: isize,
    y: isize,
    used: usize,
    avail: usize,
}

impl Node {
    fn parse(line: &str) -> Self {
        let components: Vec<_> = line.split_whitespace().collect();
        assert_eq!(components.len(), 5);
        let name: Vec<_> = components[0].split('-').collect();
        let used_str = components[2];
        let avail_str = components[3];
        Self {
            y: name[name.len()-1][1..].parse().unwrap(),
            x: name[name.len()-2][1..].parse().unwrap(),
            used: used_str[..used_str.len()-1].parse::<usize>().unwrap(),
            avail: avail_str[..avail_str.len()-1].parse::<usize>().unwrap(),
        }
    }

    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let mut nodes: Vec<_> = input.lines().skip(2).map(Node::parse).collect();
    let goal_node = nodes.iter().filter(|n| n.y == 0).max_by_key(|n| n.x).unwrap().clone();
    // Normalize the data units by dividing by the lowest. WARNING: While this will vastly reduce
    // the number of states the A* algorithm below considers distinct, and thus greatly improves
    // runtime, it may lead to incorrect result due to rounding errors on arbitrary input.
    let min = nodes.iter().filter(|n| n.used > 0).map(|n| n.used).min().unwrap();
    nodes.iter_mut().for_each(|n| { n.used /= min; n.avail/= min; });
    let first = find_pairs(&nodes);
    let second = find_shortest_path_to_top_right(&nodes, (goal_node.x, goal_node.y));
    (first, second)
}

fn get_neighbours(current: &[Node], goal: (isize, isize)) -> Vec<(Vec<Node>, (isize, isize))> {
    let mut res = Vec::new();
    for origin in current {
        if origin.used == 0 {
            continue;
        }
        for direction in [(-1,0), (1, 0), (0,-1), (0,1)] {
            let mut new = current.to_vec();
            if let Some(mut destination) = new.iter_mut().find(|dest| dest.x == origin.x + direction.0 && dest.y == origin.y + direction.1 && dest.avail >= origin.used) {
                let new_goal = if goal == (origin.x, origin.y) {
                    (destination.x, destination.y)
                } else {
                    goal
                };
                destination.used += origin.used;
                destination.avail -= origin.used;
                let mut new_origin = new.iter_mut().find(|node| node.x == origin.x && node.y == origin.y).unwrap();
                new_origin.used = 0;
                new_origin.avail += origin.used;
                res.push((new, new_goal));
            }
        }
    }
    res
}

/// A* search algorithm
fn find_shortest_path_to_top_right(start: &[Node], goal: (isize, isize)) -> usize {
    let mut empty = start.iter().find(|n| n.used == 0).cloned().unwrap_or(Node { x: goal.0, y: goal.1, used: 0, avail: 0});
    let mut open_set = HashSet::from([(start.to_vec(), goal)]);
    let mut shortest = HashMap::from([((start.to_vec(), goal), 0)]);
    // WARNING: The h function is too pessimistic to be considered generally admissible. The first
    // term (`(goal.0+goal.1)*5`) should be fine (TM), since it corresponds to the actual costs of
    // circling the empty slot around the target data. It slightly overestimates because it doesn't
    // account for a circulation already in progress, but that really only matters in the very end. 
    // For arbitrary data it might be better to `.saturating_sub()` one circle. The rest introduces a heavy bias
    // toward moving the gap upwards and a slight one for moving it rightwards. This of course is a
    // gross overestimation, especially in y direction and causes it to not consider all directions
    // especially in the early steps. 
    let mut estimate = HashMap::from([((start.to_vec(), goal), (goal.0+goal.1) as usize * 5 + 4*empty.y.abs_diff(goal.1) + empty.x.abs_diff(goal.0))]);

    while !open_set.is_empty() {
        let (current, goal_location) = open_set.iter().min_by_key(|cg| estimate.get(cg).unwrap()).unwrap().clone();
        open_set.remove(&(current.to_vec(), goal_location));
        let costs_so_far = *shortest.get(&(current.to_vec(), goal_location)).unwrap();
        if goal_location == (0, 0) {
            return costs_so_far;
        }
        for neighbour in get_neighbours(&current, goal_location) {
            let tentative_dist = costs_so_far + 1;
            if shortest.get(&neighbour).unwrap_or(&usize::MAX) > &tentative_dist {
                empty = neighbour.0.iter().find(|n| n.used == 0).cloned().unwrap_or(Node { x: neighbour.1.0, y: neighbour.1.1, used: 0, avail: 0 });
                open_set.insert(neighbour.clone());
                shortest.insert(neighbour.clone(), tentative_dist);
                estimate.insert(neighbour.clone(), tentative_dist + (neighbour.1.0 + neighbour.1.1) as usize * 5 + 4*empty.y.abs_diff(neighbour.1.1) + empty.x.abs_diff(neighbour.1.0));
            }
        }
    }
    panic!("Exhausted all routes but found no solution");
}

fn find_pairs(nodes: &[Node]) -> usize {
    nodes.iter()
        .filter(|left| left.used != 0)
        .map(|left| nodes.iter()
                        .filter(|&right| !left.eq(right) && right.avail >= left.used)
                        .count())
        .sum()
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
        assert_eq!(run(&sample_input), (7, 7));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (934, 207));
    }
}
