use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    UnequalInputLength(usize, usize),
    ParseIntError(std::num::ParseIntError),
    LineMalformed(&'a str),
    InputMustBeTwoLines(&'a str),
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputMustBeTwoLines(i) => write!(f, "Input has to be exactly 2 lines long, but was \"{i}\""),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
            Self::UnequalInputLength(t, d) => write!(f, "Input had a different number of values for both lines: {t} times but {d} distances"),
        }
    }
}

struct Race {
    time: usize,
    distance: usize,
}

fn try_into_races(input: &str) -> Result<Vec<Race>, ParseError> {
    let lines: Vec<_> = input.lines().collect();
    if lines.len() != 2 {
        return Err(ParseError::InputMustBeTwoLines(input));
    }
    let times: Vec<_> = lines[0].split_whitespace().skip(1).map(|n| n.parse::<usize>()).collect::<Result<Vec<_>, ParseIntError>>()?;
    let distances: Vec<_> = lines[1].split_whitespace().skip(1).map(|n| n.parse::<usize>()).collect::<Result<Vec<_>, ParseIntError>>()?;
    if times.len() != distances.len() {
        return Err(ParseError::UnequalInputLength(times.len(), distances.len()));
    }
    Ok((0..times.len()).map(|idx| Race { time: times[idx], distance: distances[idx], }).collect::<Vec<_>>())
}

fn count_winning_strategies(race: &Race) -> usize {
    // find the minimum time we need to press the button in order to beat the record, using
    // newton's method
    let est_cutoff = estimate_cutoff(race.time as f64, race.distance as f64, 0.0);
    // find the actual minimum by trying the close integers.
    let cutoff = (est_cutoff-(2.min(est_cutoff))..=est_cutoff+2).find(|t| t*(race.time-t) > race.distance).unwrap();
    // since the function is symetric around race.time/2, we know the maximum time we can press the
    // button is race.time-cutoff, so we can easily calculate the count of integers in the range:
    // (cutoff..=race.time-cutoff).count() ==
    // (0..=race.time-(2*cutoff)).count() ==
    // (1..=race.time-(2*cutoff)+1).count() ==
    // (race.time-2*cutoff)+1
    race.time+1-2*cutoff
}

fn estimate_cutoff(time: f64, distance: f64, x: f64) -> usize {
    let y = x * (time-x) - distance;
    if y.abs() < 0.5 {
        return x.round() as usize;
    }
    let dy = (x+1.0) * (time-(x+1.0)) - (distance + y);
    estimate_cutoff(time, distance, x-y/dy)
}

fn fix_kerning(races: &[Race]) -> Race {
    let mut time = 0;
    let mut distance = 0;
    races.iter().for_each(|race| {
        let mut time_multiplier = 10;
        while time_multiplier <= race.time {
            time_multiplier *= 10;
        }
        time = time * time_multiplier + race.time;
        let mut distance_multiplier = 10;
        while distance_multiplier <= race.distance {
            distance_multiplier *= 10;
        }
        distance = distance * distance_multiplier + race.distance;
    });
    Race { 
        time,
        distance, 
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let races: Vec<_> = try_into_races(input)?;
    let first = races.iter().map(count_winning_strategies).product();
    let the_race = fix_kerning(&races);
    let second = count_winning_strategies(&the_race);
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
        assert_eq!(run(&sample_input), Ok((288, 71503)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((252000, 36992486)));
    }
}
