use core::fmt::Display;
use std::num::ParseIntError;
use std::collections::HashMap;
use std::cmp::Ordering;

const MIN: usize = 1;
const MAX: usize = 4000;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    InvalidCategory(char),
	InvalidComparison(char),
    LineMalformed(&'a str),
    ParseIntError(std::num::ParseIntError),
    RuleMalformed(&'a str),
	WrongNewLineCount,
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCategory(c) => write!(f, "Parts may only consist of categories x, m, a, and s. Found \"{c}\" instead."),
            Self::InvalidComparison(c) => write!(f, "Comparisons can only be <, or >. Found {c} instead."),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
            Self::RuleMalformed(v) => write!(f, "Rule is malformed: {v}"),
            Self::WrongNewLineCount => write!(f, "Input does not consist of two parts separated by an empty line."),
        }
    }
}

struct Rule {
	category: usize,
	comparison: Ordering,
	value: usize,
	true_id: usize,
}

impl Rule {
    fn applies_to(&self, part: &Part) -> bool {
        part.components[self.category].cmp(&self.value) == self.comparison
    }
}

struct Workflow {
	id: usize,
	rules: Vec<Rule>,
}

impl Workflow {
    fn next(&self, part: &Part) -> usize {
        for rule in &self.rules {
            if rule.applies_to(part) {
                return rule.true_id;
            }
        }
        self.rules.last().unwrap().true_id
    }
}

struct Part {
	components: [usize; 4],
}

impl<'a> TryFrom<&'a str> for Part {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		let components: Vec<_> = value.split(['{', '=', ',', '}']).collect();
		if components.len() != 10 {
			return Err(Self::Error::LineMalformed(value));
		}
		let x = components[2].parse()?;
		let m = components[4].parse()?;
		let a = components[6].parse()?;
		let s = components[8].parse()?;
		
		Ok(Self{ components: [x, m, a, s], })
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let input_parts: Vec<_> = input.split("\n\n").collect();
	if input_parts.len() != 2 {
		return Err(ParseError::WrongNewLineCount);
	}
	let workflows = parse_workflows(input_parts[0])?;
	let parts: Vec<_> = input_parts[1].lines().map(Part::try_from).collect::<Result<Vec<_>, ParseError>>()?;
    let first = parts.iter().filter(|part| {
                    let mut queue = 0; 
                    loop { 
                        queue = workflows[queue].next(part);
                        if queue < 3 {
                            break;
                        }
                    }
                    queue == 1
            }).map(|part| part.components.iter().sum::<usize>()).sum();
    let second = find_ranges(&workflows, 0, &[(MIN, MAX); 4]).iter()
						.map(|range| range.iter().map(|(lower, upper)| upper+1-lower).product::<usize>())
						.sum();
    Ok((first, second))
}

fn find_ranges(workflows: &[Workflow], current_rule: usize, in_ranges: &[(usize, usize); 4]) -> Vec<[(usize, usize); 4]> {
	if in_ranges.iter().any(|(lower, upper)| lower > upper) {
		return Vec::new();
	}
	match current_rule {
		1 => vec![*in_ranges],
		2 => Vec::new(),
		_ => {
			let mut res = Vec::new();
			let mut ranges_left = *in_ranges;
			workflows[current_rule].rules.iter().for_each(|rule| {
				let cat = rule.category;
				let cmp = rule.comparison;
				let val = rule.value;
				let mut out_ranges = ranges_left;
				match cmp {
					Ordering::Less => out_ranges[cat].1 = out_ranges[cat].1.min(val-1),
					Ordering::Greater => out_ranges[cat].0 = out_ranges[cat].0.max(val+1),
					Ordering::Equal => unreachable!("Equality is not implemented"),
				};
				find_ranges(workflows, rule.true_id, &out_ranges).iter().for_each(|range| {
					res.push(*range);
				});
				match cmp {
					Ordering::Less => ranges_left[cat].0 = ranges_left[cat].0.max(val),
					Ordering::Greater => ranges_left[cat].1 = ranges_left[cat].1.min(val),
					Ordering::Equal => unreachable!("Equality is not implemented"),
				};
			});
			res
		},
	}
}

fn parse_workflows(input: &str) -> Result<Vec<Workflow>, ParseError> {
	let mut ids = HashMap::from([("in", 0), ("A", 1), ("R", 2)]);
	let mut workflows = Vec::from([Workflow{id: 1, rules: Vec::new()}, Workflow{id: 2, rules: Vec::new()},]);
	for line in input.lines() {
		let components: Vec<_> = line.split(['{', ':', ',', '}']).collect();
		let name = components[0];
		let id = match ids.get(name) {
			Some(i) => *i,
			None => {
					let l = ids.len();
					ids.insert(name, l);
					l
				},
		};
        let mut rules = Vec::new();
        for c in components[1..components.len()-2].chunks(2) {
            let (condition, dest) = (c[0], c[1]);
            if condition.len() < 3 {
                return Err(ParseError::RuleMalformed(condition));
            }
            let category = match condition.chars().next() {
                Some('x') => Ok(0),
                Some('m') => Ok(1),
                Some('a') => Ok(2),
                Some('s') => Ok(3),
                Some(e) => Err(ParseError::InvalidCategory(e)),
                None => Err(ParseError::RuleMalformed(condition)),
            }?;
            let comparison = match condition.chars().nth(1) {
                Some('<') => Ok(Ordering::Less),
                Some('>') => Ok(Ordering::Greater),
                Some(e) => Err(ParseError::InvalidComparison(e)),
                None => Err(ParseError::RuleMalformed(condition)),
            }?;
            let value = condition[2..].parse()?;
            let true_id = match ids.get(dest) {
                Some(i) => *i,
                None => {
                    let l = ids.len();
                    ids.insert(dest, l);
                    l
                },
            };
            rules.push(Rule{ category, comparison, value, true_id });
        }
        let default = components[components.len()-2];
        let true_id = match ids.get(default) {
            Some(i) => *i,
            None => {
                    let l = ids.len();
                    ids.insert(default, l);
                    l
                },
        };
		// Last rule is default. Since all components are guaranteed to have values in [1..=4000], this comparison is always true.
        rules.push(Rule{ category: 0, comparison: Ordering::Less, value: usize::MAX, true_id });
        workflows.push(Workflow { id, rules, });
	}
	workflows.sort_by_key(|workflow| workflow.id);
	Ok(workflows)
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
        assert_eq!(run(&sample_input), Ok((19114, 167409079868000)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((446517, 0)));
    }
}
