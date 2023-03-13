use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Orbit {
    trabant_id: usize,
    center_id: usize,
}

pub fn run(input: &str) -> Result<(usize, usize), String> {
    let graph: Vec<_> = graph_from(input)?;
    let first = count_direct_and_indirect_orbits(&graph);
    let common_orbit = get_common_center(&graph, 1, 2);

    let second = distance(&graph, common_orbit, 1) + distance(&graph, common_orbit, 2) - 2;
    Ok((first, second))
}

fn graph_from(map: &str) -> Result<Vec<Orbit>, String> {
    let mut bodies = HashMap::from([("COM", 0), ("YOU", 1), ("SAN", 2)]);
    let mut graph: Vec<Orbit> = Vec::new();
    for line in map.lines() {
        if let Some((center, trabant)) = line.split_once(')') {
            let mut bodies_len = bodies.len();
            let center_id = *bodies.entry(center).or_insert(bodies_len);
            bodies_len = bodies.len();
            let trabant_id = *bodies.entry(trabant).or_insert(bodies_len);
            graph.push(Orbit { center_id, trabant_id } );
        } else {
            return Err(format!("Malformed input: '{line}' doesn't contain a ')'."));
        }
    }
    graph.sort();
    Ok(graph)
}

fn count_direct_and_indirect_orbits(graph: &[Orbit]) -> usize {
    graph.iter().map(|o| count_upstream_bodies(graph, o.trabant_id)).sum()
}

fn count_upstream_bodies(graph: &[Orbit], trabant_id: usize) -> usize {
    let trabant_idx = graph.binary_search_by_key(&trabant_id, |o| o.trabant_id).unwrap();
    if graph[trabant_idx].center_id == 0 {
        1
    } else {
        1 + count_upstream_bodies(graph, graph[trabant_idx].center_id)
    }
}

fn get_common_center(graph: &[Orbit], lhs: usize, rhs: usize) -> usize {
    if lhs == rhs {
        return lhs;
    }
    if is_orbited_by(graph, lhs, rhs) {
        return lhs;
    } else if is_orbited_by(graph, rhs, lhs) {
        return rhs;
    }
    let trabant_idx = graph.binary_search_by_key(&lhs, |o| o.trabant_id).unwrap();
    get_common_center(graph, graph[trabant_idx].center_id, rhs) 
}

fn is_orbited_by(graph: &[Orbit], inner: usize, outer: usize) -> bool {
    if inner == outer {
        true
    } else {
        let outer_idx = graph.binary_search_by_key(&outer, |o| o.trabant_id).unwrap();
        if graph[outer_idx].center_id == 0 {
            false
        } else {
            is_orbited_by(graph, inner, graph[outer_idx].center_id)
        }
    }
}

fn distance(graph: &[Orbit], inner: usize, outer: usize) -> usize {
    if inner == outer {
        0
    } else {
        let outer_idx = graph.binary_search_by_key(&outer, |o| o.trabant_id).unwrap();
        if graph[outer_idx].center_id == 0 {
            usize::MAX
        } else {
            distance(graph, inner, graph[outer_idx].center_id) + 1
        }
    }
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
        assert_eq!(run(&sample_input), Ok((54, 4)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((417916, 523)));
    }
}
