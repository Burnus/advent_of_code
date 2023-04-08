use core::fmt::Display;
use std::{num::ParseIntError, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidMaskElement(char),
    LineMalformed(String),
    ParseIntError(ParseIntError),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMaskElement(c) => write!(f, "Mask contains invalid Element: {c}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse Int: {e}"),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

#[derive(Clone, PartialEq)]
enum MaskElem {
    X,
    Zero,
    One,
}

impl MaskElem {
    fn bit_val(&self) -> usize {
        match self {
            Self::One => 1,
            _ => 0,
        }
    }
}

enum Instruction {
    Mask(Vec<MaskElem>),
    Mem(usize, usize),
}

impl TryFrom<&str> for Instruction {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let components: Vec<_> = value.split(&[' ', '=', '[', ']'][..]).collect();
        if components.len() == 4 && components[0] == "mask" && components[3].len() == 36 {
            Ok(Self::Mask(components[3].chars().map(|c| match c { 
                'X' => Ok(MaskElem::X), 
                '0' => Ok(MaskElem::Zero), 
                '1' => Ok(MaskElem::One), 
                _ => Err(Self::Error::InvalidMaskElement(c)),
            }).collect::<Result<Vec<_>, _>>()?))
        } else if components.len() == 6 && components[0] == "mem" {
            Ok(Self::Mem(components[1].parse()?, components[5].parse()?))
        } else {
            Err(Self::Error::LineMalformed(value.to_string()))
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut mask = vec![MaskElem::X; 36];
    let mut mem_1 = HashMap::new();
    let mut mem_2 = HashMap::new();
    let instructions: Vec<_> = input.lines().map(Instruction::try_from).collect::<Result<Vec<_>, _>>()?;
    instructions.iter().for_each(|instr| {
        match instr {
            Instruction::Mask(m) => mask = m.to_vec(),
            Instruction::Mem(addr, val) => {
                mem_1.insert(*addr, apply_mask(&mask, *val));
                apply_mask_v2(&mask, &mut mem_2, *addr, *val);
            },
        }
    });
    let first = mem_1.values().sum();
    let second = mem_2.values().sum();
    Ok((first, second))
}

fn apply_mask(mask: &[MaskElem], value: usize) -> usize {
    let mut res = value;
    mask.iter().rev().enumerate().for_each(|(idx, mask_elem)| {
        match mask_elem {
            MaskElem::X => (),
            MaskElem::One => res |= 2_usize.pow(idx as u32),
            MaskElem::Zero => {res |= 2_usize.pow(idx as u32); res -= 2_usize.pow(idx as u32);},
        }
    });
    res
}

fn apply_mask_v2(mask: &[MaskElem], mem: &mut HashMap<usize, usize>, address: usize, value: usize) {
    let fixed_part: usize = mask.iter().rev().enumerate().filter(|(_idx, mask_elem)| mask_elem != &&MaskElem::X).map(|(idx, me)| address & 2_usize.pow(idx as u32) | me.bit_val() * 2_usize.pow(idx as u32)).sum();
    let mut addresses = vec![fixed_part];
    mask.iter().rev().enumerate().filter(|(_idx, mask_elem)| mask_elem == &&MaskElem::X).for_each(|(idx, _me)| {
        let mut new = addresses.to_vec();
        new.iter_mut().for_each(|addr| *addr |= 2_usize.pow(idx as u32));
        addresses.append(&mut new);
    });
    addresses.iter().for_each(|addr| {
        mem.insert(*addr, value);
    });
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
        assert_eq!(run(&sample_input), Ok((51, 208)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((6559449933360, 3369767240513)));
    }
}
