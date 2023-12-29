use core::fmt::Display;
use std::collections::{HashMap, VecDeque};

const MAX_DISCONNECTS: usize = 3;

#[derive(Debug, PartialEq, Eq)]
pub enum GraphError<'a> {
    LineMalformed(&'a str),
    NoDisconnection,
}

impl Display for GraphError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Line must consist of at least two components, separated by \": \": {v}"),
            Self::NoDisconnection => write!(f, "Unable to find a way to disconnect this network"),
        }
    }
}

pub fn run(input: &str) -> Result<usize, GraphError> {
    let graph = try_parse_network(input)?;
    try_separate(&graph).map_err(|_| GraphError::NoDisconnection)
}

fn try_parse_network(input: &str) -> Result<Vec<Vec<usize>>, GraphError> {
    let mut res = Vec::new();
    let mut ids = HashMap::new();
    for line in input.lines() {
        let words: Vec<_> = line.split([':', ' ']).collect();
        if words.len() < 3 {
            return Err(GraphError::LineMalformed(line));
        }
        let name = words[0];
        let lhs = *ids.entry(name).or_insert_with(|| {res.push(Vec::new()); res.len()-1 });
        words.iter().skip(2).for_each(|name| {
            let rhs = *ids.entry(name).or_insert_with(|| {res.push(Vec::new()); res.len()-1 });
            res[lhs].push(rhs);
            res[rhs].push(lhs);
        });
    }
    Ok(res)
}

fn try_separate(graph: &[Vec<usize>]) -> Result<usize, ()> {
    // Find nodes that can't be disconnected because there are more connections between them than
    // we are allowed to cut.
    let mut strongly_connected = vec![Vec::new(); graph.len()];
    graph.iter().enumerate().for_each(|(lhs, conn)| {
        conn.iter().cloned().for_each(|rhs| {
            // max_len is a tradeoff betweeen the runtime of this loop vs. the large one below. 11
            // is benchmarked to be the best for my input. This eliminates 3209 out of the 3310
            // total connections.
            if lhs < rhs && is_strongly_connected(graph, lhs, rhs, 11) {
                strongly_connected[lhs].push(rhs);
            }
        });
    });

    // Try cutting everything that remains and see what sticks
    for (first_lhs, conn) in graph.iter().enumerate() {
        for first_rhs in conn.iter().cloned().filter(|&rhs| rhs > first_lhs && !strongly_connected[first_lhs].contains(&rhs)) {
            for (second_lhs, conn) in graph.iter().enumerate().skip(first_lhs) {
                for second_rhs in conn.iter().cloned().filter(|&rhs| rhs > second_lhs && !strongly_connected[second_lhs].contains(&rhs) && (first_lhs, first_rhs) != (second_lhs, rhs)) {
                    for (third_lhs, conn) in graph.iter().enumerate().skip(second_lhs) {
                        for third_rhs in conn.iter().cloned().filter(|&rhs| 
                                                                     rhs > third_lhs && 
                                                                     !strongly_connected[third_lhs].contains(&rhs) &&
                                                                     ![(first_lhs, first_rhs), (second_lhs, second_rhs)].contains(&(third_lhs, rhs))) {
                            let unaffected_idx = (0..).find(|idx| ![first_lhs, first_rhs, second_lhs, second_rhs, third_lhs, third_rhs].contains(idx)).unwrap();
                            let mut new = graph.to_vec();
                            new[first_lhs] = new[first_lhs].iter().cloned().filter(|rhs| *rhs != first_rhs).collect();
                            new[first_rhs] = new[first_rhs].iter().cloned().filter(|rhs| *rhs != first_lhs).collect();
                            new[second_lhs] = new[second_lhs].iter().cloned().filter(|rhs| *rhs != second_rhs).collect();
                            new[second_rhs] = new[second_rhs].iter().cloned().filter(|rhs| *rhs != second_lhs).collect();
                            new[third_lhs] = new[third_lhs].iter().cloned().filter(|rhs| *rhs != third_rhs).collect();
                            new[third_rhs] = new[third_rhs].iter().cloned().filter(|rhs| *rhs != third_lhs).collect();
                            let size = flood_fill(&new, unaffected_idx);
                            if size < graph.len()-MAX_DISCONNECTS {
                                return Ok(size*(graph.len()-size));
                            }
                        }
                    }
                }
            }
        }
    }
    Err(())
}

fn is_strongly_connected(graph: &[Vec<usize>], start: usize, dest: usize, max_len: usize) -> bool {
    let mut used = vec![Vec::new(); graph.len()];
    let mut found = 0;
    let mut open_set = VecDeque::from([Vec::from([start])]);
    let stop: Vec<_> = graph[start].iter().cloned().chain(std::iter::once(start)).collect();
    while let Some(path) = open_set.pop_front() {
        let curr = *path.last().unwrap();
        if curr == dest {
            if found == MAX_DISCONNECTS {
                return true;
            } else {
                path.windows(2).for_each(|w| {
                    used[w[0]].push(w[1]);
                    used[w[1]].push(w[0]);
                    // discard any remaining paths that share any connection with this one, since
                    // they wouldn't really be redundant to it.
                    open_set.iter_mut().filter(|p| p.windows(2).any(|pw| pw == w || pw[0] == w[1] && pw[1] == w[0])).for_each(|p| *p = stop.to_vec());
                });
                found += 1;
            }
        }
        if path.len() == max_len {
            return false;
        }
        graph[curr].iter().for_each(|next| {
            if !path.contains(next) && !used[curr].contains(next) {
                open_set.push_back(path.iter().cloned().chain(std::iter::once(*next)).collect());
            }
        });
    }
    false
}

fn flood_fill(graph: &[Vec<usize>], starting_idx: usize) -> usize {
    let mut reachable = vec![false; graph.len()];
    reachable[starting_idx] = true;
    let mut open_set = Vec::from([starting_idx]);
    while let Some(curr) = open_set.pop() {
        graph[curr].iter().for_each(|neighbour| {
            if !reachable[*neighbour] {
                reachable[*neighbour] = true;
                open_set.push(*neighbour);
            }
        });
    }
    reachable.iter().filter(|n| **n).count()
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
        assert_eq!(run(&sample_input), Ok(54));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok(546804));
    }
}
