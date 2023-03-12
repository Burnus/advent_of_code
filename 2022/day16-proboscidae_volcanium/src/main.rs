use std::{fs, usize, collections::HashMap};

#[derive(Clone)]
struct Valve {
    id: u8,
    flow_rate: usize,
    connected_valves: Vec<u8>,
    open: bool,
}

fn get_all_distances(valves: &[Valve]) -> HashMap<(u8, u8), u8> {
    let mut network: HashMap<(u8, u8), u8> = valves.iter()
        .flat_map(|origin| origin.connected_valves.iter()
             .map(|destination| ((origin.id, *destination),1_u8)))
        .collect();

     for _ in 0..3 {
        for a in valves {
            for b in valves {
                for c in valves {
                    if let Some(ab) = network.get(&(a.id, b.id)) {
                        if let Some(bc) = network.get(&(b.id, c.id)) {
                            let ac = network.get(&(a.id, c.id));
                            network.insert((a.id, c.id), (ab+bc).min(*ac.unwrap_or(&u8::MAX)));
                        }
                    }
                }
            }
        }
    }
    network
}

fn lookup_or_insert<'a>(array: &mut Vec<&'a str>, value: &'a str) -> u8 {
    if let Some(index) = array.iter().position(|val| val==&value) {
        index as u8
    } else {
        array.push(value);
        (array.len()-1) as u8
    }
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn try_permutations(valves: &[Valve], distances: &HashMap<(u8,u8),u8>, starting_index: u8, time: u8) -> usize {
    let closed_valves: Vec<Valve> = valves.iter().filter(|v| !v.open).cloned().collect();
    let mut permutations_map: Vec<Vec<Vec<usize>>> = vec![(0..closed_valves.len()).map(|i| vec![closed_valves[i].id as usize]).collect()];

    for _step in 0..closed_valves.len() {
        let mut new_permutations = Vec::new();
        for permutation in &permutations_map[permutations_map.len()-1] {
            for valve in &closed_valves {
                let valve_id = valve.id as usize;
                if permutation.contains(&valve_id) {
                    continue;
                }
                let mut new_permutation = permutation.clone();
                new_permutation.push(valve_id);
                let mut last_position = starting_index as usize;
                let mut time_spent = 0;
                for position in &new_permutation {
                    time_spent += distances.get(&(last_position as u8, *position as u8)).unwrap() + 1;
                    last_position = *position;
                }
                if time_spent < time {
                    new_permutations.push(new_permutation);
                }
            }
        }
        permutations_map.push(new_permutations.clone());
    }

    let mut available_permutations: Vec<Vec<usize>> = Vec::new();
    for level in permutations_map {
        for permutation in level {
            available_permutations.push(permutation.clone());
        }
    }

    let mut best_so_far = 0;
    // try all non-overlapping permutations
    for human_permutation in &available_permutations {
        let this_try = try_permutation(valves, distances, human_permutation, starting_index, time);
        if this_try > best_so_far {
            best_so_far = this_try;
        }
    }
    best_so_far
}

fn try_permutations_with_elephants(valves: &[Valve], distances: &HashMap<(u8,u8),u8>, starting_index: u8, time: u8) -> usize {
    let closed_valves: Vec<Valve> = valves.iter().filter(|v| !v.open).cloned().collect();
    let mut permutations_map: Vec<Vec<Vec<usize>>> = vec![(0..closed_valves.len()).map(|i| vec![closed_valves[i].id as usize]).collect()];

    for _step in 0..closed_valves.len() {
        let mut new_permutations = Vec::new();
        for permutation in &permutations_map[permutations_map.len()-1] {
            for valve in &closed_valves {
                let valve_id = valve.id as usize;
                if permutation.contains(&valve_id) {
                    continue;
                }
                let mut new_permutation = permutation.clone();
                new_permutation.push(valve_id);
                let mut last_position = starting_index as usize;
                let mut time_spent = 0;
                for position in &new_permutation {
                    time_spent += distances.get(&(last_position as u8, *position as u8)).unwrap() + 1;
                    last_position = *position;
                }
                if time_spent < time {
                    new_permutations.push(new_permutation);
                }
            }
        }
        permutations_map.push(new_permutations.clone());
    }

    let mut available_permutations: Vec<Vec<usize>> = Vec::new();
    for level in permutations_map {
        for permutation in level {
            available_permutations.push(permutation.clone());
        }
    }

    let mut best_so_far = 0;
    // try all non-overlapping permutations
    for human_permutation in &available_permutations {
        'next_permutation: for elephant_permutation in &available_permutations {
            // make sure we don't get the same permutation with reversed roles
            if human_permutation[0] < elephant_permutation[0] {
                continue;
            }
            for valve in elephant_permutation {
                if human_permutation.contains(valve) {
                    continue 'next_permutation;
                }
            }
            let this_try = try_permutation(valves, distances, human_permutation, starting_index, time) + try_permutation(valves, distances, elephant_permutation, starting_index, time);
            if this_try > best_so_far {
                best_so_far = this_try;
            }
        }
    }
    best_so_far
}

fn try_permutation(valves: &[Valve], distances: &HashMap<(u8, u8), u8>, permutation: &[usize], starting_index: u8, time: u8) -> usize {
    let mut last_position = starting_index as usize;
    let mut time_remaining = time as usize;
    let mut released = 0;
    for valve_id in permutation {
        time_remaining -= *distances.get(&(last_position as u8, *valve_id as u8)).unwrap() as usize + 1;
        released += time_remaining * valves[*valve_id].flow_rate as usize;
        last_position = *valve_id;
    }
    released
}

fn init(scan: &str) -> (Vec<Valve>, HashMap<(u8, u8), u8>, u8) {
    let mut ids = Vec::new();
    let mut all_valves: Vec<Valve> = scan.lines()
        .map(|valve_line| {
            let components = valve_line.split(' ').collect::<Vec<&str>>();
            if components.len() < 10 { panic!("{valve_line} has fewer than 10 components."); }
            let id = lookup_or_insert(&mut ids, components[1]);
            let flow_rate_with_semicolon = &components[4][5..];
            let flow_rate = flow_rate_with_semicolon[..flow_rate_with_semicolon.len()-1].parse::<usize>().unwrap();
            let mut connected_valves = Vec::new();
            for other_valve_with_comma in components.iter().skip(9).take(components.len()-10) {
                connected_valves.push(lookup_or_insert(&mut ids, &other_valve_with_comma[..other_valve_with_comma.len()-1]));
            }
            connected_valves.push(lookup_or_insert(&mut ids, components[components.len()-1]));

            Valve { 
                id, 
                flow_rate,
                connected_valves,
                open: flow_rate == 0, 
            }
        })
        .collect();

    all_valves.sort_by_key(|v| v.id);
    let all_distances = get_all_distances(&all_valves);
    
    (all_valves, all_distances, lookup_or_insert(&mut ids, "AA"))
}

fn main() {
    //let scan = read_file("sample_input");
    let scan = read_file("input");


    let (all_valves, all_distances, starting_index) = init(&scan);
    //let all_distances = get_all_distances(&all_valves);
    //let starting_index = lookup_or_insert(&mut ids, "AA");

    println!("Working alone, we release {} units.", try_permutations(&all_valves, &all_distances, starting_index, 30));

    let with_elephants = try_permutations_with_elephants(&all_valves, &all_distances, starting_index, 26);
    println!("Using elephants, we release {with_elephants} units.");
}

#[test]
fn sample_input() {
    let scan = read_file("tests/sample_input");
    let (all_valves, all_distances, starting_index) = init(&scan);

    assert_eq!(try_permutations(&all_valves, &all_distances, starting_index, 30), 1651);
    assert_eq!(try_permutations_with_elephants(&all_valves, &all_distances, starting_index, 26), 1707);
}

#[test]
fn challenge_input() {
    let scan = read_file("tests/input");
    let (all_valves, all_distances, starting_index) = init(&scan);

    assert_eq!(try_permutations(&all_valves, &all_distances, starting_index, 30), 2056);
    assert_eq!(try_permutations_with_elephants(&all_valves, &all_distances, starting_index, 26), 2513);
}
