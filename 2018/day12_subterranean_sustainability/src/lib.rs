pub fn run(input: &str) -> (isize, isize) {
    let (state_line, rules_lines) = input.split_once("\n\n").unwrap();
    let mut state: Vec<_> = state_line.split_whitespace().nth(2).unwrap().chars().enumerate().filter(|(_idx, c)| *c == '#').map(|(idx, _c)| idx as isize).collect();
    let rules: Vec<_> = rules_lines.lines().map(|line| line.split_once(" => ").unwrap()).filter(|(_pat, res)| *res == "#").map(|(pat, _res)| to_bool(pat)).collect();
    for _ in 0..20 {
        apply_rules(&mut state, &rules);
    }
    let first = state.iter().sum();
    for _ in 20..1_000 {
        apply_rules(&mut state, &rules);
    }
    let s_1000: isize = state.iter().sum();
    for _ in 1_000..2_000 {
        apply_rules(&mut state, &rules);
    }
    let s_2000: isize = state.iter().sum();
    let second = (50_000_000-2)*(s_2000-s_1000)+s_2000;
    (first, second)
}

fn to_bool(pat: &str) -> [bool; 5] {
    assert_eq!(pat.len(), 5);
    let mut res = [false; 5];
    pat.chars().enumerate().for_each(|(idx, c)| {
        if c == '#' {
            res[idx] = true;
        }
    });
    res
}

fn apply_rules(state: &mut Vec<isize>, rules: &[[bool; 5]]) {
    let mut new_state = Vec::new();
    for pot in state[0]-2..=state[state.len()-1]+2 {
        let mut local_state = [false; 5];
        for idx in 0..5 {
            if state.binary_search(&(pot+idx-2)).is_ok() {
                local_state[idx as usize] = true;
            }
        }
        if rules.contains(&local_state) {
            new_state.push(pot);
        }
    }

    std::mem::swap(&mut new_state, state);
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
        assert_eq!(run(&sample_input), (325, 999999999374));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (1623, 1600000000401));
    }
}
