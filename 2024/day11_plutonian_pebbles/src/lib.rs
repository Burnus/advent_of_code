use std::{collections::HashMap, num::ParseIntError};


/// Returns the number of stones that this `stone` will be replaced by after `remaining_steps`
/// blinks. We can iterate stone by stone, since they don't influence each other. `mem` is used for
/// memoization, since we expect many duplicate stones.
fn blink(stone: usize, remaining_steps: usize, mem: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(len) = mem.get(&(stone, remaining_steps)) {
        return *len;
    }
    if remaining_steps == 0 {
        return 1;
    }
    // Rules:
    // 0 -> 1
    // ab -> a, b
    // x -> 2024 * x
    let next = if stone == 0 { 
        vec![1] 
    } else {
        let stone_digits = stone.ilog10() + 1;
        if stone_digits & 1 == 0 {
            vec![stone / 10_usize.pow(stone_digits >> 1), stone % 10_usize.pow(stone_digits >> 1)]
        } else {
            // 2024 == 253 * 8. This is slightly faster than `stone * 2024`.
            vec![(stone * 253) << 3]
        }
    };
    let res = next.iter().map(|&stone| blink(stone, remaining_steps-1, mem)).sum();
    mem.insert((stone, remaining_steps), res);
    res
}

pub fn run(input: &str) -> Result<(usize, usize), ParseIntError> {
    let stones: Vec<_> = input.split_whitespace().map(|n| n.parse::<usize>()).collect::<Result<Vec<usize>, _>>()?;
    let mut mem = HashMap::new();
    let first = stones.iter().map(|&stone| blink(stone, 25, &mut mem)).sum();
    let second = stones.iter().map(|&stone| blink(stone, 75, &mut mem)).sum();
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
        assert_eq!(run(&sample_input), Ok((55312, 65601038650482)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((204022, 241651071960597)));
    }
}
