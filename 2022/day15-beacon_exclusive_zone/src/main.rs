use std::{fs, collections::{HashSet, HashMap}};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

impl Sensor {
    fn from(reading: &str) -> Self {
        let components = reading.split(' ').collect::<Vec<&str>>();
        if components.len() != 10 {
            panic!("{components:?} does not have 10 items.");
        }

        let sensor_x_str = &components[2][2..];
        let sensor_y_str = &components[3][2..];
        let beacon_x_str = &components[8][2..];
        let beacon_y_str = &components[9][2..];

        let sensor_x = sensor_x_str[0..sensor_x_str.len()-1].parse::<isize>().unwrap();
        let sensor_y = sensor_y_str[0..sensor_y_str.len()-1].parse::<isize>().unwrap();
        let beacon_x = beacon_x_str[0..beacon_x_str.len()-1].parse::<isize>().unwrap();
        let beacon_y = beacon_y_str[0..].parse::<isize>().unwrap();

        let position = Position {
            x: sensor_x,
            y: sensor_y,
        };
        let beacon_position = Position {
            x: beacon_x,
            y: beacon_y,
        };

        Self {
            position: position.clone(),
            beacon_distance: position.distance_to(&beacon_position),
        }
    }

    fn beacon_from(reading: &str) -> Position {
        let components = reading.split(' ').collect::<Vec<&str>>();
        if components.len() != 10 {
            panic!("{components:?} does not have 10 items.");
        }

        let beacon_x_str = &components[8][2..];
        let beacon_y_str = &components[9][2..];

        let beacon_x = beacon_x_str[0..beacon_x_str.len()-1].parse::<isize>().unwrap();
        let beacon_y = beacon_y_str[0..].parse::<isize>().unwrap();

        Position {
            x: beacon_x,
            y: beacon_y,
        }
    }


    fn at_row(&self, row: isize) -> HashSet<isize> {
        let slice_depth = self.beacon_distance - (row-self.position.y).abs();
        match slice_depth {
            nope if nope <= 0 => HashSet::new(),
            _ => (self.position.x-slice_depth..=self.position.x+slice_depth).collect(),
        }
    }

    fn first_non_reachables(&self, unreachables: &mut HashMap<Position, bool>, min: isize, max: isize) {
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


fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn beacon_free_positions(row: isize, sensors: &[Sensor], beacons: &HashSet<Position>) -> usize {
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

fn get_non_reachable(sensors: &[Sensor], beacons: &HashSet<Position>, max: isize) -> isize {
    let mut first_non_reachables = HashMap::new();
    sensors.iter()
        .for_each(|s| s.first_non_reachables(&mut first_non_reachables, 0, max));

    if let Some((&pos, _)) = &first_non_reachables.iter()
        .find(|(pos, &repeat)| repeat && !beacons.contains(pos) && !is_reachable_by(pos, sensors)) {
            pos.x * 4_000_000 + pos.y 
    } else {
        0 
    }
}

fn main() {
    let readings = read_file("input");

    let sensors = readings.lines().map(Sensor::from).collect::<Vec<_>>();
    let beacons = readings.lines().map(Sensor::beacon_from).collect::<HashSet<_>>();

    println!("Not in Line 2_000_000: {}", beacon_free_positions(2_000_000, &sensors, &beacons));
    println!("Non-Reachable Position found with frequency {}.", get_non_reachable(&sensors, &beacons, 4_000_000));
}

#[test]
fn sample_input() {
    let readings = read_file("tests/sample_input");
    let sensors = readings.lines().map(Sensor::from).collect::<Vec<_>>();
    let beacons = readings.lines().map(Sensor::beacon_from).collect::<HashSet<_>>();

    assert_eq!(beacon_free_positions(10, &sensors, &beacons), 26);
    assert_eq!(get_non_reachable(&sensors, &beacons, 20), 56000011);
}

#[test]
fn challenge_input() {
    let readings = read_file("tests/input");
    let sensors = readings.lines().map(Sensor::from).collect::<Vec<_>>();
    let beacons = readings.lines().map(Sensor::beacon_from).collect::<HashSet<_>>();

    assert_eq!(beacon_free_positions(2_000_000, &sensors, &beacons), 5367037);
    assert_eq!(get_non_reachable(&sensors, &beacons, 4_000_000), 11914583249288);
}
