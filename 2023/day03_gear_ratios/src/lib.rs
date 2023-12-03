use std::collections::HashSet;

fn find_part_numbers(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut parts = Vec::new();
    let mut possible_gears = Vec::new();
    let mut symbol_positions = HashSet::new();
    let mut star_positions = HashSet::new();
    for (line_idx, line) in input.lines().enumerate() {
        for (char_idx, char) in line.chars().enumerate() {
            if !['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'].contains(&char) {
                symbol_positions.insert((line_idx, char_idx));
            }
            if char == '*' {
                star_positions.insert((line_idx, char_idx));
            }
        }
    }
    let mut cur_num = 0;
    for (line_idx, line) in input.lines().chain("\n".lines()).enumerate() {
        if cur_num > 0 {
            let digits = (1_u32..).find(|d| 10_usize.pow(*d) > cur_num).unwrap() as usize;
            let mut is_part = false;
            let max_x = input.lines().nth(line_idx-1).unwrap().chars().count()-1;
            for y in line_idx.saturating_sub(2)..=line_idx {
                for x in max_x.saturating_sub(digits)..=max_x {
                    if symbol_positions.contains(&(y, x)) {
                        is_part = true;
                        if star_positions.contains(&(y, x)) {
                            possible_gears.push(((y, x), cur_num));
                        }
                    }
                }
            }
            if is_part {
                parts.push(cur_num);
            }

            cur_num = 0;
        }
        for (byte_idx, byte) in line.bytes().enumerate() {
            match byte {
                n if n.is_ascii_digit() => cur_num = cur_num * 10 + (n - b'0') as usize,
                _ => {
                        if cur_num > 0 {
                            let digits = (1_u32..).find(|d| 10_usize.pow(*d) > cur_num).unwrap() as usize;
                            let mut is_part = false;
                            for y in line_idx.saturating_sub(1)..=line_idx+1 {
                                for x in byte_idx.saturating_sub(digits+1)..=byte_idx {
                                    if symbol_positions.contains(&(y, x)) {
                                        is_part = true;
                                        if star_positions.contains(&(y, x)) {
                                            possible_gears.push(((y, x), cur_num));
                                        }
                                    }
                                }
                            }
                            if is_part {
                                parts.push(cur_num);
                            }
                            cur_num = 0;
                        }
                    },
            }
        }
    }
    let mut gears = Vec::new();
    for (curr_idx, possible_gear) in possible_gears.iter().enumerate() {
        let parts_adjacent_to_this_star = possible_gears.iter().enumerate().filter(|(_idx, (star, _part))| star == &possible_gear.0).collect::<Vec<_>>();
        if parts_adjacent_to_this_star.len() == 2 && parts_adjacent_to_this_star[0].0 == curr_idx {
            gears.push(parts_adjacent_to_this_star.iter().map(|(_idx, (_star, part))| part).product());
        }
    }
    (parts, gears)
}

pub fn run(input: &str) -> (usize, usize) {
    let (parts, gears) = find_part_numbers(input);
    let first = parts.iter().sum();
    let second = gears.iter().sum();
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    fn empty_str() {
        assert_eq!(run(""), (0, 0))
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), (4364, 467835));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (546312, 87449461));
    }
}
