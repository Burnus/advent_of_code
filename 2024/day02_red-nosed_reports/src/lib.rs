use std::num::ParseIntError;

#[derive(Clone)]
struct Report {
    levels: Vec<usize>,
}

impl TryFrom<&str> for Report {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let levels: Vec<_> = value.split_whitespace().map(|n| n.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
        Ok(Self { levels })
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        self.levels.len() < 2 || (
            self.levels[1] > self.levels[0] && self.levels.windows(2).all(|w| w[1] > w[0] && w[1] - w[0] <= 3) ||
            self.levels[1] < self.levels[0] && self.levels.windows(2).all(|w| w[1] < w[0] && w[0] - w[1] <= 3)
        )
    }

    fn dampened(&self) -> Vec<Self> {
        let len = self.levels.len();
        if len < 2 {
            Vec::from([self.clone()])
        } else {
            (0..len).map(|idx| {
                let levels = self.levels.iter().take(idx).copied().chain(self.levels.iter().skip(idx+1).copied()).collect();
                Self{ levels }
            }).collect()
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseIntError> {
    let reports: Vec<_> = input.lines().map(Report::try_from).collect::<Result<Vec<_>, _>>()?;
    let first = reports.iter().filter(|r| r.is_safe()).count();
    let second = reports.iter().filter(|r| r.dampened().iter().any(|rd| rd.is_safe())).count();
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
        assert_eq!(run(&sample_input), Ok((2, 4)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((591, 621)));
    }
}
