pub enum NumberFormat { DigitsOnly, DigitsAndSpelledOut }

/// Extract the first digit of String `line`, concatenated with its last digit, as a usize. If `f`
/// is `NumberFormat::DigitsOnly`, only the ASCII characters `0` through `9` count as digits, if it
/// is `NumberFormat::DigitsAndSpelledOut`, the English words `one` through `nine` count as well.
/// In the latter case, overlaps are allowed.
///
/// ## Example
/// ```
/// use day01_trebuchet::{NumberFormat, calibration_value};
/// assert_eq!(calibration_value("foo123four", NumberFormat::DigitsOnly), 13);
/// assert_eq!(calibration_value("twone3elevenzero", NumberFormat::DigitsAndSpelledOut), 23);
/// ```
pub fn calibration_value(line: &str, f: NumberFormat) -> usize {
    // Replace number words with digits for part 2, but keep all Es, Ns, Os and Ts at the beginning
    // and end, because the words may overlap (which is allowed). Tho other letters, as well as
    // longer overlaps cannot occur in these words (i. e. while `one` ends in `ne`, no number
    // starts with `ne`; conversely, `six` starts with an S, which no number ends with).
    let line_v2 = line.replace("one", "o1e").replace("two", "t2o").replace("three", "t3e").replace("four", "4").replace("five", "5e").replace("six", "6").replace("seven", "7n").replace("eight", "e8t").replace("nine", "n9e");
    let line = match f {
        NumberFormat::DigitsOnly => line,
        NumberFormat::DigitsAndSpelledOut => &line_v2,
    };
    let digits: Vec<&str> = line.matches(|c: char| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        0
    } else {
        10 * digits[0].parse::<usize>().unwrap() + digits[digits.len()-1].parse::<usize>().unwrap()
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let first = input.lines().map(|l| calibration_value(l, NumberFormat::DigitsOnly)).sum();
    let second = input.lines().map(|l| calibration_value(l, NumberFormat::DigitsAndSpelledOut)).sum();
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
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input), (351, 423));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (54644, 53348));
    }
}
