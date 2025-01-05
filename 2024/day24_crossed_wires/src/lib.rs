use core::fmt::Display;
use std::{collections::{HashMap, HashSet}, num::ParseIntError};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    DuplicateName(&'a str),
    InputMalformed,
    IllegalSate(&'a str, &'a str),
    ParseIntError(std::num::ParseIntError),
    LineMalformed(&'a str),
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateName(v) => write!(f, "Input for gate {v} is defined twice"),
            Self::InputMalformed => write!(f, "Input must consist of the initial states, followed by an empty line, and the rules block"),
            Self::IllegalSate(name, state) => write!(f, "Unable to set gate {name} to {state}. Only '0' and '1' are allowed."),
            Self::LineMalformed(v) => write!(f, "Line is malformed:\n{v}\n Must be of form: x01 XOR y01 -> z01."),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Gate<'a> {
    Value(bool),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

#[derive(Clone)]
struct Device<'a> {
    gates: HashMap<&'a str, Gate<'a>>,
    output_gates: usize,
    x: usize,
    y: usize,
}

impl<'a> TryFrom<&'a str> for Device<'a> {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some((input, rules)) = value.split_once("\n\n") {
            let mut gates = HashMap::new();
            let mut output_gates = 0;
            let mut x = 0;
            let mut y = 0;

            for line in input.lines() {
                if let Some((name, val)) = line.split_once(": ") {
                    let input = name.chars().next();
                    let idx = name[1..].parse::<usize>()?;
                    match (input, val) {
                        (_, "0") => (),
                        (Some('x'), "1") => x |= 1 << idx,
                        (Some('y'), "1") => y |= 1 << idx,
                        _ => return Err(Self::Error::IllegalSate(name, val)),
                    }
                }
            }
            for line in rules.lines() {
                let components: Vec<_> = line.split_whitespace().collect();
                if components.len() != 5 {
                    return Err(Self::Error::LineMalformed(line));
                }
                let op = components[1];
                let in_0 = components[0];
                let in_1 = components[2];
                let this = components[4];
                if let Some(idx) = this.strip_prefix('z') {
                    let idx = idx.parse::<usize>()?;
                    output_gates = output_gates.max(idx+1);
                    match op {
                        "AND" => _ = gates.insert(this, Gate::And(in_0, in_1)),
                        "OR" => _ = gates.insert(this, Gate::Or(in_0, in_1)),
                        "XOR" => _ = gates.insert(this, Gate::Xor(in_0, in_1)),
                        e => return Err(Self::Error::IllegalSate(components[4], e)),
                    }
                } else {
                    if gates.contains_key(&this) {
                        return Err(Self::Error::DuplicateName(components[4]));
                    }
                    match op {
                        "AND" => _ = gates.insert(this, Gate::And(in_0, in_1)),
                        "OR" => _ = gates.insert(this, Gate::Or(in_0, in_1)),
                        "XOR" => _ = gates.insert(this, Gate::Xor(in_0, in_1)),
                        e => return Err(Self::Error::IllegalSate(components[4], e)),
                    }
                }
            }
            Ok(Self {
                gates,
                output_gates,
                x,
                y,
            })
        } else {
            Err(Self::Error::InputMalformed)
        }
    }
}

impl<'a> Device<'a> {
    fn eval_gate(&mut self, name: &str, modifying: bool) -> bool {
        if let Some(num) = name.strip_prefix('x') {
            if let Ok(digit) = num.parse::<usize>() {
                return self.x & (1 << digit) > 0;
            }
        } else if let Some(num) = name.strip_prefix('y') {
            if let Ok(digit) = num.parse::<usize>() {
                return self.y & (1 << digit) > 0;
            }
        }
        // unwrap() is safe here, because we know we will only ever call this function on valid
        // gates.
        let this = *self.gates.get(name).unwrap();
        let res = match this {
            Gate::Value(a) => a,
            Gate::And(a, b) => match (self.gates.get(a), self.gates.get(b)) {
                (Some(Gate::Value(false)), _) | (_, Some(Gate::Value(false))) => {
                    false
                },
                _ => {
                    self.eval_gate(a, modifying) && self.eval_gate(b, modifying)
                },
            },
            Gate::Or(a, b) => match (self.gates.get(a), self.gates.get(b)) {
                (Some(Gate::Value(true)), _) | (_, Some(Gate::Value(true))) => {
                    true
                },
                _ => {
                    self.eval_gate(a, modifying) || self.eval_gate(b,  modifying)
                },
            },
            Gate::Xor(a, b) => self.eval_gate(a, modifying) ^ self.eval_gate(b, modifying),
        };
        if modifying {
            // unwrap() is safe here because we would have failed earlier otherwise
            *self.gates.get_mut(name).unwrap() = Gate::Value(res);
        }
        res
    }

    fn eval_output_gate(&mut self, idx: usize, modifying: bool) -> usize {
        let name = if idx < 10 {
            &format!("z0{idx}")[..]
        } else {
            &format!("z{idx}")[..]
        };
        self.eval_gate(name, modifying) as usize
    }

    fn eval(&mut self) -> usize {
        (0..self.output_gates).map(|idx| self.eval_output_gate(idx, true) << idx).sum()
    }

    fn get_dependent_gates(&self, name: &'a str) -> HashSet<&'a str> {
        match self.gates.get(name) {
            Some(Gate::And(a, b)) | Some(Gate::Or(a, b )) | Some(Gate::Xor(a, b)) => 
            HashSet::from([name]).union(&self.get_dependent_gates(a)).cloned().collect::<HashSet<_>>().union(&self.get_dependent_gates(b)).cloned().collect(),
            Some(Gate::Value(_)) => HashSet::from([name]),
            None => HashSet::new(),
        }
    }

    fn is_loop_free(&self, name: &str, previous: &HashSet<&str>) -> bool {
        match self.gates.get(name) {
            None | Some(Gate::Value(_)) => true,
            Some(Gate::And(a, b)) | Some(Gate::Or(a, b )) | Some(Gate::Xor(a, b)) => {
                if previous.contains(a) || previous.contains(b) {
                    return false;
                }
                let mut previous = previous.clone();
                previous.insert(a);
                previous.insert(b);
                self.is_loop_free(a, &previous) && self.is_loop_free(b, &previous)
            }
        }
    }
    fn output_gate(idx: usize) -> String {
        if idx < 10 {
            format!("z0{idx}")
        } else {
            format!("z{idx}")
        }
    }

    /// Determines if the `idx`th least significant bit of z can be traced to input bits (x and y)
    /// on all paths (returning `true`), or if they form a loop (returning `false`).
    fn output_is_loop_free(&self, idx: usize) -> bool {
        self.is_loop_free(&Self::output_gate(idx)[..], &HashSet::new())
    }

    /// Checks if the rightmost `z_idx` bits of z behave like an adder. Returns `None`, if they do,
    /// and `Some(idx)` otherwise, where `idx` is the first bit (from the right), which differs.
    fn check_until(&mut self, z_idx: usize) -> Option<usize> {
        let tests_0 = [(0, 0), (0, 1), (1, 0), (1, 1)];
        if tests_0.iter().any(|(l, r)| {
            self.x = *l;
            self.y = *r;
            !self.output_is_loop_free(0) ||
            self.eval_output_gate(0, false) != l ^ r
        }) {
            return Some(0);
        }
        if z_idx == 0 {
            return None;
        }
        let tests = [
            (0, 0, 0, 0), (0, 1, 0, 0), (1, 0, 0, 0), (1, 1, 0, 0),
            (0, 0, 0, 1), (0, 1, 0, 1), (1, 0, 0, 1), (1, 1, 0, 1),
            (0, 0, 1, 0), (0, 1, 1, 0), (1, 0, 1, 0), (1, 1, 1, 0),
            (0, 0, 1, 1), (0, 1, 1, 1), (1, 0, 1, 1), (1, 1, 1, 1),
        ];
        (1..=z_idx).find(|&z_idx| {
            !self.output_is_loop_free(z_idx) ||
            tests.iter().any(|(l, r, prev_l, prev_r)| {
                self.x = ((*l << 1) + *prev_l) << (z_idx - 1);
                self.y = ((*r << 1) + *prev_r) << (z_idx - 1);
                self.eval_output_gate(z_idx, false) != l ^ r ^ (prev_l & prev_r)
            })
        })
    }

    /// Try swapping all combinations of two gates, where at least one of them is contained in
    /// `must_include` and none of them in `swapped_before` and determine if their swap results in
    /// the rightmost `z_idx` bits of z are correct. Returns an unsorted Vec of all such pairs.
    fn try_swaps(&'a mut self, z_idx: usize, must_include: &[&'a str], swapped_before: &[&'a str]) -> Vec<[String; 2]> {
        let mut res = Vec::new();
        // We need to clone these so the borrow checker won't complain about concurrent borrows in the
        // swap loops.
        let all_inputs: Vec<_> = self.gates.keys().cloned().collect();
        // Try only switching one pair of gates and hope we find a solution this way (works for my
        // input).
        for &gate_1 in must_include {
            if swapped_before.contains(&gate_1) {
                continue;
            }
            // The unwrap()s below are safe because the constructor made sure all gates exist.
            let inputs_1 = *self.gates.get(gate_1).unwrap();
            for &gate_2 in all_inputs.iter().filter(|&&gate_2| 
                gate_2 != gate_1 && 
                !swapped_before.contains(&gate_2))
            {
                let inputs_2 = *self.gates.get(gate_2).unwrap();
                *self.gates.get_mut(gate_1).unwrap() = inputs_2;
                *self.gates.get_mut(gate_2).unwrap() = inputs_1;
                if self.check_until(z_idx).is_none() {
                    res.push([gate_1.into(), gate_2.into()]);
                }
                *self.gates.get_mut(gate_1).unwrap() = inputs_1;
                *self.gates.get_mut(gate_2).unwrap() = inputs_2;
            }
        }
        res
    }

    /// Find the necessary (up to 8) swaps to turn this device into a binary adder.
    /// `swapped_before` contains the swaps already established (this should be an empty array at
    /// first). Returns a `Vec` of the affected gates, sorted alphabetically.
    fn swap_gates(&mut self, swapped_before: &[String]) -> Vec<String> {
        let mut swaps_performed = swapped_before.to_vec();
        loop {
            if let Some(next_error) = self.check_until(self.output_gates-2) {
                let output_name = Self::output_gate(next_error);
                let swapped_before: Vec<_> = swaps_performed.iter().map(|s: &String| &s[..]).collect();
                let must_include: Vec<&str> = self.get_dependent_gates(&output_name[..]).into_iter().collect();
                let mut next = self.clone();
                let new_possible_swaps = next.try_swaps(next_error, &must_include, &swapped_before);
                match new_possible_swaps.len() {
                    // The unwrap()s below are safe because the constructor made sure all gates exist.
                    0 => return Vec::new(),     // If we found no solution, return early
                    1 => {
                        // We found one solution. Continue with it.
                        for swap in new_possible_swaps[0].chunks(2) {
                            let (gate_1, gate_2) = (swap[0].to_string(), swap[1].to_string());
                            let inputs_1 = *self.gates.get(&gate_1[..]).unwrap();
                            *self.gates.get_mut(&gate_1[..]).unwrap() = *self.gates.get(&gate_2[..]).unwrap();
                            *self.gates.get_mut(&gate_2[..]).unwrap() = inputs_1;
                            swaps_performed.push(gate_1);
                            swaps_performed.push(gate_2);
                        }
                    },
                    _ => {
                        // We found more than one solution. 
                        // Spawn a new Device for each and try them all.
                        for swaps in new_possible_swaps {
                            if swaps.len() + swaps_performed.len() > 8 {
                                continue;
                            }
                            let mut next = self.clone();
                            let mut swapped_before = swaps_performed.to_vec();
                            for swap in swaps.chunks(2) {
                                let (gate_1, gate_2) = (swap[0].to_string(), swap[1].to_string());
                                let inputs_1 = *self.gates.get(&gate_1[..]).unwrap();
                                *next.gates.get_mut(&gate_1[..]).unwrap() = *self.gates.get(&gate_2[..]).unwrap();
                                *next.gates.get_mut(&gate_2[..]).unwrap() = inputs_1;
                                swapped_before.push(gate_1);
                                swapped_before.push(gate_2);
                            }
                            let res = next.swap_gates(&swapped_before);
                            if res.len() <= 8 {
                                return res;
                            }
                        }
                        return Vec::new();
                    }
                }
            } else if swaps_performed.len() <= 8 {
                swaps_performed.sort();
                return swaps_performed;
            } else {
                return Vec::new();
            }
        }
    }
}

pub fn run(input: &str) -> Result<(usize, String), ParseError> {
    let mut device_1 = Device::try_from(input)?;
    let mut device_2 = device_1.clone();
    let first = device_1.eval();
    let second = if device_2.output_gates > 13 {
        device_2.swap_gates(&[]).join(",")
    } else {
        String::new()
    };
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
        assert_eq!(run(&sample_input), Ok((2024, "".to_string())));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((57270694330992, "gwh,jct,rcb,wbw,wgb,z09,z21,z39".to_string())));
    }
}
