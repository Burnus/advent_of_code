use std::collections::{HashSet, HashMap, VecDeque};

#[derive(PartialEq, Clone)]
enum Faction { Elf, Goblin }

type Position = (usize, usize);

#[derive(PartialEq, Clone)]
struct Actor {
    faction: Faction,
    position: Position,
    hp: usize,
    damage: usize
}

impl Actor {
    fn new(position: Position, faction: Faction) -> Self {
        Self {
            faction,
            position,
            hp: 200,
            damage: 3,
        }
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let (actors, walls) = parse_map(input);
    let first = combat_result(&mut actors.clone(), &walls, false).unwrap();
    let mut second = 0;
    for power in 4.. {
        let mut new_actors = actors.clone();
        new_actors.iter_mut().for_each(|actor| {
            if actor.faction == Faction::Elf {
                actor.damage = power;
            }
        });
        if let Some(success) = combat_result(&mut new_actors, &walls, true) {
            second = success;
            break;
        }
    }
    (first, second)
}

fn combat_result(actors: &mut Vec<Actor>, walls: &[Position], break_if_elf_dies: bool) -> Option<usize> {
    let by_position = |a: &Actor, b: &Actor| -> std::cmp::Ordering { a.position.0.cmp(&b.position.0).then_with(|| a.position.1.cmp(&b.position.1)) };
    for round in 0.. {
        let mut actor_idx = 0;
        loop {
            if actor_idx >= actors.len() {
                break;
            }
            let mut actor = actors[actor_idx].clone();
            let enemies: Vec<_> = actors.iter().filter(|e| e.faction != actor.faction).collect();
            if enemies.is_empty() {
                return Some(round * actors.iter().map(|a| a.hp).sum::<usize>());
            }
            let threatened: Vec<Position> = enemies.iter().flat_map(|e| get_neighbours(e.position)).collect();
            if !threatened.contains(&actor.position) {
                let open: Vec<Position> = threatened.iter().cloned().filter(|n| !actors.iter().any(|a| a.position == *n) && !walls.iter().any(|w| w == n)).collect();
                actor.position = step_to_nearest(actor.position, &open, &actors.iter().map(|a| a.position).collect::<Vec<Position>>().iter().chain(walls.iter()).cloned().collect::<Vec<Position>>());
                actors[actor_idx].position = actor.position;
            }
            if threatened.contains(&actor.position) {
                let possible_targets = get_neighbours(actor.position);
                let enemy_idx = actors.iter().enumerate().filter(|(_idx, t)| t.faction != actor.faction && possible_targets.contains(&t.position)).min_by(|a, b| a.1.hp.cmp(&b.1.hp).then_with(|| by_position(a.1, b.1))).unwrap().0;
                if actors[enemy_idx].hp <= actor.damage {
                    if break_if_elf_dies && actor.faction == Faction::Goblin {
                        return None;
                    }
                    actors.remove(enemy_idx);
                    if enemy_idx < actor_idx {
                        actor_idx -= 1;
                    }
                } else {
                    actors[enemy_idx].hp -= actor.damage;
                }
            }
            actor_idx += 1;
        }
        actors.sort_by(by_position);
    }
    unreachable!("The loop always runs and we only break from it by returning early.");
}

fn get_neighbours(position: Position) -> [Position; 4] {
    [
        (position.0-1, position.1),
        (position.0, position.1-1),
        (position.0, position.1+1),
        (position.0+1, position.1),
    ]
}

fn step_to_nearest(starting: Position, targets: &[Position], obstacles: &[Position]) -> Position {
    let mut found = HashSet::new();
    let mut open_set = VecDeque::from([starting]);
    let mut distances = HashMap::from([(starting, 0)]);
    let mut first_step = HashMap::new();
    let mut shortest_path = usize::MAX;
    while !open_set.is_empty() {
        let current = open_set.pop_front().unwrap();
        let curr_distance = *distances.get(&current).unwrap();
        if targets.contains(&current) {
            shortest_path = shortest_path.min(curr_distance);
        } else if curr_distance < shortest_path {
            for neighbour in get_neighbours(current) {
                if !found.contains(&neighbour) && !obstacles.contains(&neighbour) {
                    let first = *first_step.get(&current).unwrap_or(&neighbour);
                    let distance = curr_distance + 1;
                    found.insert(neighbour);
                    open_set.push_back(neighbour);
                    first_step.insert(neighbour, first);
                    distances.insert(neighbour, distance);
                }
            }
        }
    }
    if shortest_path == usize::MAX {
        starting
    } else {
        let target = targets.iter().filter(|t| Some(&shortest_path) == distances.get(t)).min_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1))).unwrap();
        *first_step.get(target).unwrap()
    }
}

fn parse_map(map: &str) -> (Vec<Actor>, Vec<Position>) {
    let mut actors = Vec::new();
    let mut walls = Vec::new();
    
    map.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            match c {
                'E' => actors.push(Actor::new((y, x), Faction::Elf)),
                'G' => actors.push(Actor::new((y, x), Faction::Goblin)),
                '#' => walls.push((y, x)),
                '.' => (),
                _ => panic!("Unexpected Token at {x}, {y}: {c}"),
            }
        });
    });

    (actors, walls)
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
        let sample_inputs = read_file("tests/sample_input");
        let sample_inputs: Vec<_> = sample_inputs.split("\n\n").collect();
        let expected = [
                (27730, 4988),
                (36334, 29064),
                (39514, 31284),
                (27755, 3478),
                (28944, 6474),
                (18740, 1140),
            ];
        for (idx, sample_input) in sample_inputs.iter().enumerate() {
            assert_eq!(run(sample_input), expected[idx]);
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (250594, 52133));
    }
}
