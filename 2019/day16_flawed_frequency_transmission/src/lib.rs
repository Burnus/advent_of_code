use std::num::ParseIntError;
use std::collections::HashMap;


pub fn run(input: &str) -> Result<(usize, usize), ParseIntError> {
    let mut signal = input.trim().to_string();
    let offset = signal[..7].parse()?;
    let signal_2 = signal.repeat(10_000)[offset..].to_string();
    for _phase in 0..100 {
        signal = fft(&signal, 0);
    }
    let first = signal[0..8].parse()?;
    let second = first_digits(&signal_2);
    Ok((first, second))
}

fn get_coeffitient(idx: usize, offset: usize, phase: usize, mem: &mut HashMap<(usize, usize),usize>) -> usize {
    if offset > idx {
        0
    } else if idx-offset == 0 || phase == 1 {
        1
    } else if let Some(c) = mem.get(&(idx-offset, phase)) {
        *c
    } else {
        mem.remove(&(idx.saturating_sub(9), phase));
        let c = (get_coeffitient(idx-1, offset, phase, mem) + get_coeffitient(idx, offset, phase-1, mem)) % 10;
        mem.insert((idx-offset, phase), c);
        c
    }
}

fn first_digits(signal: &str) -> usize {
    let mut digits = [0; 8];
    let mut mem = HashMap::new();
    for (idx, freq) in signal.bytes().enumerate() {
        let freq = (freq - b'0') as usize;
        assert!((0..=9).contains(&freq));
        for (pos, digit) in digits.iter_mut().enumerate() {
            *digit = (*digit + freq * get_coeffitient(idx, pos, 100, &mut mem) ) % 10;
        }
    }
    digits.iter().enumerate().map(|(pos, i)| i * 10_i32.pow(7-pos as u32) as usize).sum::<usize>()
}

fn fft(signal: &str, offset: usize) -> String {
    let mut res = String::new();
    for position in 0..signal.len() {
        let pos: isize = (position..signal.len()).filter(|i| ((i+offset+1)/(position+1))%4 == 1).map(|i| (signal.as_bytes().get(i).unwrap() - b'0') as isize).sum();
        let neg: isize = (position..signal.len()).filter(|i| ((i+offset+1)/(position+1))%4 == 3).map(|i| (signal.as_bytes().get(i).unwrap() - b'0') as isize).sum();
        res += &format!("{digit}", digit = (pos-neg).abs() % 10);
    }
    res
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
        let expected = [
            (24465799, 84462026),
            (82441489, 78725270),
            (52486276, 53553731),
        ];
        for (idx, input) in sample_input.lines().enumerate() {
            assert_eq!(run(input), Ok(expected[idx]));
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((42205986, 13270205)));
    }
}
