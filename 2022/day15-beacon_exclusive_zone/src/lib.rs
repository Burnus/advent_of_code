use core::fmt::Display;
use std::num::ParseIntError;
use std::collections::{BTreeSet, BTreeMap};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    ParseIntError(std::num::ParseIntError),
    LineMalformed(&'a str),
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn distance_to(&self, other: &Position) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

}

struct Sensor {
    position: Position,
    beacon_distance: isize,
}

impl <'a> TryFrom<&'a str> for Sensor {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let components = value.split(' ').collect::<Vec<&str>>();
        if components.len() != 10 {
            return Err(Self::Error::LineMalformed(value));
        }

        let sensor_x_str = &components[2][2..];
        let sensor_y_str = &components[3][2..];
        let beacon_x_str = &components[8][2..];
        let beacon_y_str = &components[9][2..];

        let sensor_x = sensor_x_str[0..sensor_x_str.len()-1].parse::<isize>()?;
        let sensor_y = sensor_y_str[0..sensor_y_str.len()-1].parse::<isize>()?;
        let beacon_x = beacon_x_str[0..beacon_x_str.len()-1].parse::<isize>()?;
        let beacon_y = beacon_y_str[0..].parse::<isize>().unwrap();

        let position = Position {
            x: sensor_x,
            y: sensor_y,
        };
        let beacon_position = Position {
            x: beacon_x,
            y: beacon_y,
        };

        Ok(Self {
            position,
            beacon_distance: position.distance_to(&beacon_position),
        })
    }
}

impl Sensor {
    fn beacon_from(reading: &str) -> Result<Position, ParseError> {
        let components = reading.split(' ').collect::<Vec<&str>>();
        if components.len() != 10 {
            return Err(ParseError::LineMalformed(reading));
        }

        let beacon_x_str = &components[8][2..];
        let beacon_y_str = &components[9][2..];

        let beacon_x = beacon_x_str[0..beacon_x_str.len()-1].parse::<isize>()?;
        let beacon_y = beacon_y_str[0..].parse::<isize>()?;

        Ok(Position {
            x: beacon_x,
            y: beacon_y,
        })
    }


    fn at_row(&self, row: isize) -> BTreeSet<isize> {
        let slice_depth = self.beacon_distance - (row-self.position.y).abs();
        match slice_depth {
            nope if nope <= 0 => BTreeSet::new(),
            _ => (self.position.x-slice_depth..=self.position.x+slice_depth).collect(),
        }
    }

    fn first_non_reachables(&self, unreachables: &mut BTreeMap<Position, bool>, min: isize, max: isize) {
        // top right
        for i in 0..=self.beacon_distance {
            let x = self.position.x+i;
            let y = self.position.y-(self.beacon_distance+1)+i;
            if (min..=max).contains(&x) && (min..=max).contains(&y) {
                unreachables.entry(Position{ x, y }).and_modify(|repeat| *repeat = true).or_insert(false);
            }
        }

        // bottom right
        for i in 0..=self.beacon_distance {
            let x = self.position.x+self.beacon_distance+1-i;
            let y = self.position.y+i;
            if (min..=max).contains(&x) && (min..=max).contains(&y) {
                unreachables.entry(Position{ x, y }).and_modify(|repeat| *repeat = true).or_insert(false);
            }
        }

        // bottom left
        for i in 0..=self.beacon_distance {
            let x = self.position.x-i;
            let y = self.position.y+self.beacon_distance+1+i;
            if (min..=max).contains(&x) && (min..=max).contains(&y) {
                unreachables.entry(Position{ x, y }).and_modify(|repeat| *repeat = true).or_insert(false);
            }
        }

        // top left
        for i in 0..=self.beacon_distance {
            let x = self.position.x-(self.beacon_distance+1)+i;
            let y = self.position.y-i;
            if (min..=max).contains(&x) && (min..=max).contains(&y) {
                unreachables.entry(Position{ x, y }).and_modify(|repeat| *repeat = true).or_insert(false);
            }
        }
    }
}


fn beacon_free_positions(row: isize, sensors: &[Sensor], beacons: &BTreeSet<Position>) -> usize {
    sensors.iter()
        .map(|s| s.at_row(row))
        .reduce(|a, b| a.union(&b).cloned().collect())
        .unwrap()
        .len() - beacons.iter().filter(|b| b.y == row).count()
}

fn is_reachable_by(position: &Position, sensors: &[Sensor]) -> bool {
    for sensor in sensors {
        if position.distance_to(&sensor.position) <= sensor.beacon_distance {
            return true;
        }
    }
    false
}

fn get_non_reachable(sensors: &[Sensor], beacons: &BTreeSet<Position>, max: isize) -> isize {
    let mut first_non_reachables = BTreeMap::new();
    sensors.iter()
        .for_each(|s| s.first_non_reachables(&mut first_non_reachables, 0, max));

    if let Some((&pos, _)) = &first_non_reachables.iter()
        .find(|(pos, &repeat)| repeat && !beacons.contains(pos) && !is_reachable_by(pos, sensors)) {
            pos.x * 4_000_000 + pos.y 
        } else {
            0 
        }
}

pub fn run(input: &str) -> Result<(usize, isize), ParseError> {
    let sensors = input.lines().map(Sensor::try_from).collect::<Result<Vec<_>, _>>()?;
    let beacons = input.lines().map(Sensor::beacon_from).collect::<Result<BTreeSet<_>, _>>()?;

    let first = beacon_free_positions(2_000_000, &sensors, &beacons); 
    let second = get_non_reachable(&sensors, &beacons, 4_000_000);
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
    fn sample_input() {
        let readings = read_file("tests/sample_input");
        let sensors = readings.lines().map(Sensor::try_from).collect::<Result<Vec<_>, _>>().unwrap();
        let beacons = readings.lines().map(Sensor::beacon_from).collect::<Result<BTreeSet<_>, _>>().unwrap();

        assert_eq!(beacon_free_positions(10, &sensors, &beacons), 26);
        assert_eq!(get_non_reachable(&sensors, &beacons, 20), 56000011);
    }

    #[test]
    fn challenge_input() {
        let readings = read_file("tests/challenge_input");

        assert_eq!(run(&readings), Ok((5367037, 11914583249288))); 
    }
}
