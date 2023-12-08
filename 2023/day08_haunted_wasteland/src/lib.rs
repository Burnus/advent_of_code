use core::fmt::Display;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum NetworkError<'a> {
    NodeNotFound(Node),
    LineMalformed(&'a str),
    InvalidChar(char),
    InputMalformed(&'a str),
    DoubleDefinition(Node),
}

pub struct InvalidCharError{ offending_character: char }
pub struct NodeNotFoundError{ node: Node }

impl From<InvalidCharError> for NetworkError<'_> {
    fn from(value: InvalidCharError) -> Self {
        Self::InvalidChar(value.offending_character)
    }
}

impl From<NodeNotFoundError> for NetworkError<'_> {
    fn from(value: NodeNotFoundError) -> Self {
        Self::NodeNotFound(value.node)
    }
}

impl Display for NetworkError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoubleDefinition(n) => write!(f, "Connections for node {} are defined more than once", from_node(n)),
            Self::InputMalformed(v) => write!(f, "Input must consist of exactly 2 parts separated by an empty line, but is:\n{v}"),
            Self::InvalidChar(c) => write!(f, "Invalid Direction {c} in instructions"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::NodeNotFound(n) => write!(f, "Error trying to find a way from node {}, for which there are no outgoing connections in the network", &from_node(n)),
        }
    }
}

type Node = u32;

fn node_from(chars: &[char; 3]) -> Node {
    ((chars[0] as u32) << 16) + ((chars[1] as u32) << 8) + chars[2] as u32
}

fn from_node(n: &Node) -> String {
    let mut res = String::new();
    res.push((n >> 16) as u8 as char);
    res.push(((n & 0x0000FF00) >> 8) as u8 as char);
    res.push((n & 0x000000FF) as u8 as char);
    res
}

#[derive(PartialEq, Clone, Copy)]
enum Direction{ Left, Right }

impl TryFrom<char> for Direction {
    type Error = InvalidCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            e => Err(Self::Error{offending_character: e}),
        }
    }
}

fn try_parse_maps(input: &str) -> Result<(Vec<Direction>, HashMap<Node, (Node, Node)>), NetworkError> {
    let parts: Vec<_> = input.split("\n\n").collect();
    if parts.len() != 2 {
        return Err(NetworkError::InputMalformed(input));
    }
    let path = parts[0].chars().map(Direction::try_from).collect::<Result<Vec<_>, InvalidCharError>>()?;

    let mut network = HashMap::new();
    for line in parts[1].lines() {
        let connection: Vec<char> = line.matches(char::is_alphabetic).map(|ch|ch.chars().next().unwrap()).collect();
        if connection.len() != 9 {
            return Err(NetworkError::LineMalformed(line));
        }
        let start = node_from(connection[0..3].try_into().unwrap());
        let left = node_from(connection[3..6].try_into().unwrap());
        let right = node_from(connection[6..9].try_into().unwrap());
        if network.contains_key(&start) {
            return Err(NetworkError::DoubleDefinition(start));
        }
        network.insert(start, (left, right));
    }

    Ok(( path, network ))
}

fn walk(network: &HashMap<Node, (Node, Node)>, path: &[Direction], start: &[Node], dest: &[Node]) -> Result<usize, NodeNotFoundError> {
    let mut currs = start.to_vec();
    // The ghosts always return to a destination in the same number of steps, so we only
    // need to track this number for all ghosts.
    let mut cycles = vec![0; start.len()];
    let len = path.len();

    for step in 0.. {
        if cycles.iter().all(|offset| offset != &0) {
            // The first step all ghosts will be at their destinations will be the least common
            // multiple of the steps any one of them took.
            return Ok(cycles.iter().cloned().reduce(lcm).unwrap());
        }
        for (idx, curr) in currs.iter_mut().enumerate() {
            if cycles[idx] == 0 && dest.contains(curr) {
                cycles[idx] = step;
            }
            let go = path[step % len];
            let ways = network.get(curr).ok_or(NodeNotFoundError{ node: *curr})?;
            *curr = match go {
                Direction::Left => ways.0,
                Direction::Right => ways.1,
            }
        }
    }
    unreachable!()
}

fn gcd(lhs: usize, rhs: usize) -> usize {
    let mut l = lhs;
    let mut r = rhs;
    while r != 0 {
        let temp = r;
        r = l % r;
        l = temp;
    }
    l
}

fn lcm(lhs: usize, rhs: usize) -> usize {
    (lhs / gcd(lhs, rhs)) * rhs
}

pub fn run(input: &str) -> Result<(usize, usize), NetworkError> {
    let (path, network) = try_parse_maps(input)?;
    let start = node_from(&['A', 'A', 'A']);
    let dest = node_from(&['Z', 'Z', 'Z']);
    let first = walk(&network, &path, &[start], &[dest])?;
    let ghost_starts: Vec<_> = network.keys().filter(|&k| k & 0xFF == b'A' as u32).cloned().collect();
    let ghost_dests: Vec<_> = network.keys().filter(|&k| k & 0xFF == b'Z' as u32).cloned().collect();
    let second = walk(&network, &path, &ghost_starts, &ghost_dests)?;
    Ok((first, second))
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
        assert_eq!(run(&sample_input), Ok((2, 2)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((19199, 13663968099527)));
    }
}
