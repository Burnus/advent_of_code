use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    LineMalformed(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

struct FileSystem {
    nodes: Vec<Node>,
    current_working_dir: NodeID,
}

struct Node {
    parent: Option<NodeID>,
    next_sibling: Option<NodeID>,
    first_child: Option<NodeID>,
    last_child: Option<NodeID>,

    name: String,
    size: Option<usize>,
}

#[derive(Clone, Copy)]
struct NodeID {
    index: usize,
}

impl FileSystem {
    fn new_node(&mut self, name: String, size: Option<usize>) -> NodeID {
        let next_index = self.nodes.len();
        self.nodes.push(Node {
            parent: None,
            next_sibling: None,
            first_child: None,
            last_child: None,

            name,
            size,
        });
        NodeID { index: next_index }
    }

    fn init(&mut self) {
        self.new_node(String::from("/"), None);
    }

    fn cd(&mut self, target: &str) {
        self.current_working_dir = match target {
            "/" => NodeID { index: 0 },
            ".." => NodeID { index: self.nodes[self.current_working_dir.index].parent.unwrap().index },
            dir => NodeID { index: self.get_child_dir_index(dir).unwrap() },
        }
    }

    fn get_child_dir_index(&self, name: &str) -> Option<usize> {
        if let Some(first_child) = self.nodes[self.current_working_dir.index].first_child {
            let mut this_child_index = first_child.index;
            while self.nodes[this_child_index].name != *name.to_string() {
                if let Some(next_sibling) = self.nodes[this_child_index].next_sibling{
                    this_child_index = next_sibling.index;
                } else {
                return None;
                }
            }
            Some(this_child_index)
        } else {
            None
        }
    }

    fn ensure_child_exists(&mut self, name: &str, size: Option<usize>) {
        if self.get_child_dir_index(name).is_some() {
            return;
        }
        let cwd = self.current_working_dir.index;
        let new_node = self.new_node(name.to_string(), size).index;
        self.nodes[new_node].parent = Some(NodeID { index: cwd });
        if let Some(sibling) = self.nodes[cwd].last_child {
            self.nodes[sibling.index].next_sibling = Some(NodeID { index: new_node });
        } else {
            self.nodes[cwd].first_child = Some(NodeID { index: new_node });
        }
        self.nodes[cwd].last_child = Some(NodeID { index: new_node });
    }

    fn parse_terminal_output(&mut self, terminal_output: &str) -> Result<(), ParseError> {
        for line in terminal_output.lines() {
            if line.starts_with("$ cd") {
                self.cd(&line[5..]);
            } else if line == "$ ls" {
                continue;
            } else if let Some((size, name)) = line.split_once(' ') {
                    self.ensure_child_exists(name, size.parse().ok());
            } else {
                return Err(ParseError::LineMalformed(line.to_string()));
            }
        }
        Ok(())
    }

    fn get_directory_sizes(&self, anchor: NodeID) -> Vec<(String, usize)> {
        let mut this_directory_file_sizes = 0;
        let mut sub_directories_file_sizes = Vec::new();
        let mut this_child = self.nodes[anchor.index].first_child;
        while this_child.is_some() {
            let this_index = this_child.unwrap().index;
            if let Some(filesize) = self.nodes[this_index].size {
                this_directory_file_sizes += filesize;
            } else {
                sub_directories_file_sizes.append(&mut self.get_directory_sizes(NodeID { index: this_index }));
                this_directory_file_sizes += sub_directories_file_sizes.last().unwrap().1;
            }
            this_child = self.nodes[this_index].next_sibling;
        }
        sub_directories_file_sizes.push((self.nodes[anchor.index].name.clone(), this_directory_file_sizes));
        sub_directories_file_sizes
    }
}

pub fn run(terminal_output: &str) -> Result<(usize, usize), ParseError> {
    let mut file_system = FileSystem {
        nodes: Vec::new(),
        current_working_dir: NodeID { index: 0 },
    };
    file_system.init();
    file_system.parse_terminal_output(terminal_output)?;

    let directory_sizes = file_system.get_directory_sizes(NodeID { index: 0 });
    let dir_sizes_under_100k_sum: usize = directory_sizes.iter()
        .filter(|(_, size)| *size<=100_000)
        .map(|(_, size)| *size)
        .sum();
    let total_size = directory_sizes.last().unwrap().1;
    let smallest_dir_to_delete_size = directory_sizes.iter()
        .filter(|(_, size)| *size>=total_size-40_000_000)
        .map(|(_, size)| *size)
        .min()
        .unwrap_or_default();

    Ok((dir_sizes_under_100k_sum, smallest_dir_to_delete_size))
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
        assert_eq!(run(&sample_input), Ok((95437, 24933642)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((1743217, 8319096)));
    }
}
