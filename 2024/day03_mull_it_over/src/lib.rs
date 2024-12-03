use std::convert::Infallible;

fn instructions_from(value: &str) -> (usize, usize) {
    let mut skip = 0;
    let mut total_sum = 0;
    let mut enabled_sum = 0;

    // We build a list of enabling and disabling instructions beforehand, so we don't have to
    // rfind() them in the str for every product.
    // We start with the default case of enabled. All other indexes are offset by 1, so an input
    // starting with `don't()" will take precedence over it. The offset doesn't hurt us, since we
    // look up by match index anyway, which is always at least 12 higher than the last match for
    // `do()` or `don't()`.
    let mut enabled: Vec<(usize, bool)> = [(0, true)].into_iter()
        .chain(value
        .match_indices("do()")
        .map(|(idx, _match)| (idx+1, true))
        .chain(value
            .match_indices("don't()")
            .map(|(idx, _match)| (idx+1, false))
            )).collect();
    enabled.sort();
    while let Some(idx) = value[skip..].find("mul(") {
        skip += idx + 4;
        if let Some(idx) = value[skip..].find(',') {
            if let Ok(lhs) = value[skip..skip+idx].parse::<usize>() {
                skip += idx + 1;
                if let Some(idx) = value[skip..].find(')') {
                    if let Ok(rhs) = value[skip..skip+idx].parse::<usize>() {
                        skip += idx + 1;
                        let product = lhs * rhs;
                        total_sum += product;

                        // `partition_point()` returns the index into enabled for the next do() or
                        // don't() after our current match. The entry preceeding this, contains the
                        // relevant instruction.
                        if enabled[enabled.partition_point(|(idx, _is_enabled)| *idx < skip)-1].1 {
                            enabled_sum += product;
                        }
                    }
                }
            }
        }
    }
    (total_sum, enabled_sum)
}

pub fn run(input: &str) -> Result<(usize, usize), Infallible> {
    let (first, second) = instructions_from(input);
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
        assert_eq!(run(&sample_input), Ok((161, 48)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((159892596, 92626942)));
    }
}
