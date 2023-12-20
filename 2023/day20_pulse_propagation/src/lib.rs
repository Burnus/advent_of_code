use core::fmt::Display;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
	InvalidType(char),
    LineMalformed(&'a str),
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidType(c) => write!(f, "Module types can only be %, &, or broadcaster. Found {c} instead."),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Signal { High, Low, }

#[derive(Clone, PartialEq, Eq)]
enum ModuleType {
	FlipFlop(Signal),
	Conjunction(HashMap<usize, Signal>),
	Broadcast,
}

#[derive(Clone)]
struct Module {
	id: usize,
	module_type: ModuleType,
	outputs: Vec<usize>,
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut modules = try_parse_modules(input)?;
    let mut results = Vec::new();
	let (mut low_count, mut high_count, mut rx_idx) = (0, 0, 0);
    let rx_trigger = modules.iter().find(|m| m.outputs.contains(&1)).map(|m| m.id).unwrap();
    let trigger_triggers: Vec<_> = modules.iter().filter(|m| m.outputs.contains(&rx_trigger)).map(|m| m.id).collect();
    let mut trigger_trigger_indexes = vec![0; trigger_triggers.len()];

	for press in 0.. {
		let this = push_button(&mut modules, &trigger_triggers);
        if press < 1000 {
            low_count += this.0;
            high_count += this.1;
            results.push((low_count, high_count));
        }
        this.2.iter().enumerate().filter(|(_idx, res)| **res).for_each(|(idx, _res)| {
            if trigger_trigger_indexes[idx] == 0 {
                trigger_trigger_indexes[idx] = press+1;
                if trigger_trigger_indexes.iter().all(|count| count > &0) {
                    rx_idx = trigger_trigger_indexes.iter().cloned().reduce(lcm).unwrap();
                }
            }
        });
        if press >= 1000 && rx_idx > 0 {
            break;
        }
	}
	let first = low_count*high_count;

    let second = rx_idx;
    Ok((first, second))
}

fn lcm(lhs: usize, rhs: usize) -> usize {
    lhs * rhs / gcd(lhs, rhs)
}

fn gcd(lhs: usize, rhs: usize) -> usize {
    let (mut a, mut b) = (lhs, rhs);
    while b != 0 {
        (a, b) = (b, a%b);
    }
    a
}

fn try_parse_modules(input: &str) -> Result<Vec<Module>, ParseError> {
	let mut ids = HashMap::from([("output", 0), ("rx", 1), ("roadcaster", 2)]);
	let mut modules = Vec::from([Module{ id: 0, module_type: ModuleType::Broadcast, outputs: Vec::new() }, Module{ id: 1, module_type: ModuleType::Broadcast, outputs: Vec::new()}]);
	for line in input.lines() {
		let components: Vec<_> = line.split([' ', ',']).collect();
		if components.len() < 3 || components[0].len() < 2 {
			return Err(ParseError::LineMalformed(line));
		}
		let module_type = match components[0].chars().next() {
			Some('b') => Ok(ModuleType::Broadcast),
			Some('%') => Ok(ModuleType::FlipFlop(Signal::Low)),
			Some('&') => Ok(ModuleType::Conjunction(HashMap::new())),
			Some(e) => Err(ParseError::InvalidType(e)),
			None => unreachable!(),
		}?;
		let name = &components[0][1..];
		let id = match ids.get(&name) {
			Some(i) => *i,
			None => {
					let l = ids.len();
					ids.insert(name, l);
					l
				},
		};
		let outputs = components.iter().skip(2).step_by(2).map(|dest| {
					match ids.get(dest) {
						Some(i) => *i,
						None => {
								let l = ids.len();
								ids.insert(dest, l);
								l
							},
					}
				}).collect();
		modules.push(Module{ id, module_type, outputs, });
	}
	modules.sort_by_key(|module| module.id);
	for idx in 0..modules.len() {
		if matches!(modules[idx].module_type, ModuleType::Conjunction(_)) {
			let init = modules.iter().filter(|module| module.outputs.contains(&modules[idx].id)).map(|module| (module.id, Signal::Low)).collect();
			modules[idx].module_type = ModuleType::Conjunction(init);
		}
	}
	Ok(modules)
}

fn push_button(modules: &mut [Module], watch_list: &[usize]) -> (usize, usize, Vec<bool>) {
	let (mut send_low, mut send_high) = (Vec::from([(2, 2)]), Vec::new());
	let (mut low_count, mut high_count) = (0, 0);
    let mut watch_results = vec![false; watch_list.len()];
	while !send_low.is_empty() || !send_high.is_empty() {
		low_count += send_low.len();
		high_count += send_high.len();
		(send_low, send_high) = tick(modules, &send_low, &send_high);
        watch_list.iter().enumerate().for_each(|(idx, id)| if send_high.iter().any(|(from, _to)| from == id) { watch_results[idx] = true; });
	}
	(low_count, high_count, watch_results)
}

fn tick(modules: &mut [Module], low_to_send: &[(usize, usize)], high_to_send: &[(usize, usize)]) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
	let (mut next_low, mut next_high) = (Vec::new(), Vec::new());
	low_to_send.iter().for_each(|(from_idx, to_idx)| {
		let next = send_low(*from_idx, *to_idx, modules);
		match next.1 {
			Signal::Low => next.0.iter().for_each(|next_to_idx| { next_low.push((*to_idx, *next_to_idx)); }),
			Signal::High => next.0.iter().for_each(|next_to_idx| { next_high.push((*to_idx, *next_to_idx)); }),
		};
	});
	high_to_send.iter().for_each(|(from_idx, to_idx)| {
		let next = send_high(*from_idx, *to_idx, modules);
		match next.1 {
			Signal::Low => next.0.iter().for_each(|next_to_idx| { next_low.push((*to_idx, *next_to_idx)); }),
			Signal::High => next.0.iter().for_each(|next_to_idx| { next_high.push((*to_idx, *next_to_idx)); }),
		};
	});
	(next_low, next_high)
}

fn send_low(from_idx: usize, to_idx: usize, modules: &mut [Module]) -> (Vec<usize>, Signal) {
    let curr = modules[to_idx].clone();
    match curr.module_type {
        ModuleType::Broadcast => (curr.outputs.to_vec(), Signal::Low),
        ModuleType::FlipFlop(Signal::Low) => {
            modules[to_idx].module_type = ModuleType::FlipFlop(Signal::High);
            (curr.outputs.to_vec(), Signal::High)
        },
        ModuleType::FlipFlop(Signal::High) => {
            modules[to_idx].module_type = ModuleType::FlipFlop(Signal::Low);
            (curr.outputs.to_vec(), Signal::Low)
        },
        ModuleType::Conjunction(inputs) => {
            let mut new = inputs;
            new.insert(from_idx, Signal::Low);
            modules[to_idx].module_type = ModuleType::Conjunction(new);
            (curr.outputs.to_vec(), Signal::High)
        },
    }
}

fn send_high(from_idx: usize, to_idx: usize, modules: &mut [Module]) -> (Vec<usize>, Signal) {
    let curr = modules[to_idx].clone();
    match curr.module_type {
        ModuleType::Broadcast => (curr.outputs.to_vec(), Signal::High),
        ModuleType::FlipFlop(_) => (Vec::new(), Signal::High),
        ModuleType::Conjunction(inputs) => {
            let mut new = inputs;
            new.insert(from_idx, Signal::High);
            modules[to_idx].module_type = ModuleType::Conjunction(new.clone());
            if new.iter().all(|(_idx, input)| input == &Signal::High) {
                (curr.outputs.to_vec(), Signal::Low)
            } else {
                (curr.outputs.to_vec(), Signal::High)
            }
        },
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
        assert_eq!(run(&sample_input), Ok((11687500, 1)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((836127690, 240914003753369)));
    }
}
