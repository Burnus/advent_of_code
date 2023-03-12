#[derive(Default)]
struct Node {
    children: Vec<usize>,
    data: Vec<usize>,
}

pub fn run(input: &str) -> (usize, usize) {
    let entries: Vec<_> = input.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
    let mut cursor = 0;
    let mut nodes = Vec::new();
    while cursor < entries.len()-1 {
        parse_tree(&entries, &mut nodes, &mut cursor);
    }
    
    let first = nodes.iter().map(|n| n.data.iter().sum::<usize>()).sum();
    let second = value(&nodes, 0);
    (first, second)
}

fn value(nodes: &[Node], root_idx: usize) -> usize {
    let root = &nodes[root_idx];
    if root.children.is_empty() {
        root.data.iter().sum()
    } else {
        root.data.iter()
            .filter(|id| id <= &&root.children.len())
            .map(|child| value(nodes, root.children[*child-1]))
            .sum()
    }
}

fn parse_tree(entries: &[usize], nodes: &mut Vec<Node>, cursor: &mut usize) {
    let id = nodes.len();
    let child_count = entries[*cursor];
    let data_count = entries[*cursor+1];
    *cursor += 2;
    nodes.push(Node::default());
    for _ in 0..child_count {
        let child_id = nodes.len();
        nodes[id].children.push(child_id);
        parse_tree(entries, nodes, cursor);
    }
    nodes[id].data = entries[*cursor..*cursor + data_count].to_vec();
    *cursor += data_count;
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
        assert_eq!(run(&sample_input), (138, 66));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (46962, 22633));
    }
}
