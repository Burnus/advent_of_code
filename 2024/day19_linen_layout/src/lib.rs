use core::fmt::Display;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InputMalformed,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputMalformed => write!(f, "Input must consist of a list of available patterns, separated by \", \", an empty line, and a newline-separated list of the desired designs"),
        }
    }
}

// Try to consume the design into patterns. If it has been fully consumed, we are successfull.
// Otherwise try consuming the first characters in every way we can, and check the rest.
fn can_be_constructed(design: &str, patterns: &VecDeque<&str>) -> bool {
    design.is_empty() || 
    patterns.iter().any(|p| design.starts_with(p) && can_be_constructed(&design[p.len()..], patterns))
}

// The same as above, but count the successfull ways and sum them up , instead of returning 
// after the first one. Also, memorize the results in a HashMap, so we don't have to compute 
// them again. The HashMap must contain (String::new(), 1) prior to the call, so we can 
// avoid the extra branch for empty designs.
fn ways_to_constrtuct(design: &str, patterns: &[&str], mem: &mut HashMap<String, usize>) -> usize {
    if let Some(known) = mem.get(design) {
        *known
    } else {
        let ways = patterns
            .iter()
            .filter(|p| design.starts_with(*p))
            .map(|p| ways_to_constrtuct(&design[p.len()..], patterns, mem))
            .sum();
        mem.insert(design.to_string(), ways);
        ways
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    if let Some((patterns, designs)) = input.split_once("\n\n") {
        let patterns: Vec<_> = patterns.split(", ").collect();
        let mut filtered_patterns = VecDeque::from(patterns.clone());

        // Trim the patterns: For instance, the samle contains the patterns 'r', 'b', and 'g', as
        // well as 'rb', 'gb', and 'br', which can be constructed from the former and so are
        // unnecessary for part 1, but would significantly slow down the program.
        (0..patterns.len()).for_each(|_| {
            let this = filtered_patterns.pop_front().unwrap();
            if !can_be_constructed(this, &filtered_patterns) {
                filtered_patterns.push_back(this);
            }
        });
        let designs : Vec<_> = designs.lines().collect();
        let possible: Vec<_> = designs.iter().filter(|d| can_be_constructed(d, &filtered_patterns)).collect();
        let first = possible.len();
        let mut mem = HashMap::from([(String::new(), 1)]);
        let second = possible.iter().map(|d| ways_to_constrtuct(d, &patterns, &mut mem)).sum();
        Ok((first, second))
    } else {
        Err(ParseError::InputMalformed)
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
        assert_eq!(run(&sample_input), Ok((6, 16)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((251, 616957151871345)));
    }
}
