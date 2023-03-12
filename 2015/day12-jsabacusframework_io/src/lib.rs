pub fn run(input: &str) -> (i32, i32) {
    let first = input.lines().map(sumup_numbers).sum();
    let second = input.lines().map(|line| sumup_numbers(&filter_red(line))).sum();
    (first, second)
}

fn sumup_numbers(line: &str) -> i32 {
    let mut sum = 0;

    let mut sign = 1_i32;
    let mut this_number = 0_u32;
    for c in line.chars() {
        if c == '-' {
            sign *= -1;
        } else if let Some(d) = c.to_digit(10) {
            this_number *= 10;
            this_number += d;
        } else {
            sum += this_number as i32 * sign;
            this_number = 0;
            sign = 1
        }
    }
   // dbg!(sum)
    sum
}

fn filter_red(line: &str) -> String {
    let left_square: Vec<_> = line.match_indices('[').map(|(idx, _)| idx).collect();
    let right_square: Vec<_> = line.match_indices(']').map(|(idx, _)| idx).collect();
    let left_curly: Vec<_> = line.match_indices('{').map(|(idx, _)| idx).collect();
    let right_curly: Vec<_> = line.match_indices('}').map(|(idx, _)| idx).collect();
    let reds: Vec<_> = line.match_indices("red").map(|(idx, _)| idx).collect();

    let mut filtered = String::from(line);

    for red in &reds {
        let left_count = left_curly.partition_point(|lc| lc < red);
        let right_count = right_curly.partition_point(|rc| rc < red);
        let curly_level = left_count - right_count;
        let start = *left_curly.iter().take(left_count).rev().find(|lc| left_curly.partition_point(|left| left <= lc) - right_curly.partition_point(|right| right <= lc) == curly_level).unwrap();
        
        // skip reds that are in an array, unless they are part of an object inside the array
        let left_square_count = left_square.partition_point(|ls| ls < red);
        let square_level = left_square_count - right_square.partition_point(|rs| rs < red); 
        if square_level > 0 {
            let opening_square = *left_square.iter().take(left_square_count).rev().find(|ls| left_square.partition_point(|left| left <= ls) - right_square.partition_point(|right| right <= ls) == square_level).unwrap();
            if opening_square > start {
                continue;
            }
        }
        let end = *right_curly.iter()
            .enumerate()
            .skip(right_count)
            .find(|(idx, end)| left_curly.partition_point(|lc| lc < end) - idx == curly_level)
            .unwrap_or_else(|| panic!("Unable to find a right brace after {red} in {line}"))
            .1 + 1;

        filtered.replace_range(start..end, &"x".repeat(end-start));
    }
    
    filtered
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), (39, 22));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (119433, 68466));
    }
}
