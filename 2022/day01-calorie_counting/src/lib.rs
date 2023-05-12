use std::num::ParseIntError;

fn try_parse_cal_list(list: &str) -> Result<Vec<usize>, ParseIntError> {
    let mut cals: Vec<_> = list.split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|individual_list| individual_list.lines()
             .map(|n| n.parse::<usize>())
             .sum::<Result<usize, _>>())
        .collect::<Result<Vec<_>, _>>()?;

    cals.sort_by_key(|i| std::cmp::Reverse(*i));
    Ok(cals)
}

pub fn run(input: &str) -> Result<(usize, usize), ParseIntError> {
    let elves = try_parse_cal_list(input)?;
    Ok((elves[0], elves.iter().take(3).sum::<usize>()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_file(path: &str) -> String {
        std::fs::read_to_string(path)
            .expect("File not Found")
    }

    #[test]
    fn sample_input() {
        let list = read_file("tests/sample_input");
        assert_eq!(run(&list), Ok((24000, 45000)));
    }

    #[test]
    fn challenge_input() {
        let list = read_file("tests/input");
        assert_eq!(run(&list), Ok((71780, 212489)));
    }
}
