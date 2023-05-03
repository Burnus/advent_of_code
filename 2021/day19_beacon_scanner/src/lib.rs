use core::fmt::Display;
use std::{num::ParseIntError, collections::{HashSet, HashMap}};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ParseIntError(std::num::ParseIntError),
    LineMalformed(String),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

const ROTATIONS_MATRICES: [[isize; 9]; 24] = [
    [1, 0, 0, 0, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, -1, 0],
    [1, 0, 0, 0, -1, 0, 0, 0, -1],
    [1, 0, 0, 0, 0, -1, 0, 1, 0],
    [0, 1, 0, 0, 0, 1, 1, 0, 0],
    [0, 1, 0, 1, 0, 0, 0, 0, -1],
    [0, 1, 0, 0, 0, -1, -1, 0, 0],
    [0, 1, 0, -1, 0, 0, 0, 0, 1],
    [0, 0, 1, 1, 0, 0, 0, 1, 0],
    [0, 0, 1, 0, 1, 0, -1, 0, 0],
    [0, 0, 1, -1, 0, 0, 0, -1, 0],
    [0, 0, 1, 0, -1, 0, 1, 0, 0],
    [-1, 0, 0, 0, -1, 0, 0, 0, 1],
    [-1, 0, 0, 0, 0, 1, 0, 1, 0],
    [-1, 0, 0, 0, 1, 0, 0, 0, -1],
    [-1, 0, 0, 0, 0, -1, 0, -1, 0],
    [0, -1, 0, 0, 0, -1, 1, 0, 0],
    [0, -1, 0, 1, 0, 0, 0, 0, 1],
    [0, -1, 0, 0, 0, 1, -1, 0, 0],
    [0, -1, 0, -1, 0, 0, 0, 0, -1],
    [0, 0, -1, -1, 0, 0, 0, 1, 0],
    [0, 0, -1, 0, 1, 0, 1, 0, 0],
    [0, 0, -1, 1, 0, 0, 0, -1, 0],
    [0, 0, -1, 0, -1, 0, -1, 0, 0],
]; 

type Coordinates = (isize, isize, isize);

struct Scan {
    probes: Vec<Coordinates>,
    distances: Vec<Vec<(usize, usize, usize)>>,
}

impl TryFrom<&str> for Scan {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut probes = Vec::new();
        let mut distances: Vec<Vec<(usize, usize, usize)>> = Vec::new();

        for line in value.lines().skip(1) {
            let coords: Vec<_> = line.split(',').map(|i| i.parse()).collect::<Result<Vec<_>, _>>()?;
            if coords.len() != 3 {
                return Err(Self::Error::LineMalformed(line.to_string()));
            }
            let this = (coords[0], coords[1], coords[2]);
            probes.push(this);
            let mut this_distances = Vec::new();
            for other in &probes {
                this_distances.push((manhattan_distance(this, *other), min_distance(this, *other), max_distance(this, *other)));
            }
            for (idx, other_distances) in distances.iter_mut().enumerate() {
                other_distances.push(this_distances[idx]);
            }
            distances.push(this_distances);
        }

        Ok(Self { probes, distances, })
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut scans: Vec<_> = input.split("\n\n").map(Scan::try_from).collect::<Result<Vec<_>, _>>()?;
    let mut known_beacons: HashSet<_> = scans[0].probes.iter().cloned().collect();
    let mut known_distances = HashMap::new();
    extend_distances(&mut known_distances, &scans[0].probes.to_vec());
    let mut scanners = vec![(0, 0, 0)];
    scans.swap_remove(0);

    while !scans.is_empty() {
        for (idx, scan) in scans.iter().enumerate() {
            if let Some((scanner, report)) = find_match(&known_beacons, &known_distances, scan) {
                extend_distances(&mut known_distances, &report);
                scanners.push(scanner);
                known_beacons.extend(report.iter());
                scans.swap_remove(idx);
                break;
            }
        }
    }

    let first = known_beacons.len();
    let second = scanners.iter().map(|lhs| scanners.iter().map(|rhs| manhattan_distance(*lhs, *rhs)).max().unwrap()).max().unwrap();
    Ok((first, second))
}

fn find_match(known_beacons: &HashSet<Coordinates>, known_distances: &HashMap<(usize, usize, usize), Vec<[Coordinates; 2]>>, scan: &Scan) -> Option<(Coordinates, Vec<Coordinates>)> {
    let matches = scan.distances.iter().enumerate().flat_map(|(y, row)| row.iter().enumerate().skip(y+1).filter(|(_x, dist)| known_distances.contains_key(dist)).map(|(x, _dist)| (y, x)).collect::<Vec<_>>()).collect::<Vec<_>>(); 

    // Ignore if we have matches for less than 12 probes (2 choose 12 = 66 matches in total)
    if matches.len() < 66 {
        return None;
    }

    for (y, x) in matches {
        for known_pair in known_distances.get(&scan.distances[y][x]).unwrap() {
            let [kp1, kp2] = known_pair;
            let [rp1, rp2] = [scan.probes[y], scan.probes[x]];

            if let Some(rotation) = ROTATIONS_MATRICES.iter()
                .find(|&r| vec_sub(*kp1, matrix_mul(r, rp1)) == vec_sub(*kp2, matrix_mul(r, rp2))) {
                    let translation = vec_sub(*kp1, matrix_mul(rotation, rp1));

                    let transformed = scan.probes.iter()
                        .map(|p| vec_add(matrix_mul(rotation, *p), translation))
                        .collect::<Vec<_>>();

                    if transformed.iter().filter(|p| known_beacons.contains(p)).count() >= 3 {
                        return Some((translation, transformed));
                    }
                }
        }
    }
    None
}

fn extend_distances(known_distances: &mut HashMap<(usize, usize, usize), Vec<[Coordinates; 2]>>, report: &Vec<Coordinates>) {
    for lhs in report {
        for rhs in report {
            let distances = (manhattan_distance(*lhs, *rhs), min_distance(*lhs, *rhs), max_distance(*lhs, *rhs));
            known_distances.entry(distances).or_insert(Vec::from([[*lhs, *rhs]]));
        }
    }
}

fn matrix_mul(matrix: &[isize; 9], vec: Coordinates) -> Coordinates {
    (
        vec.0 * matrix[0] + vec.1 * matrix[3] + vec.2 * matrix[6],
        vec.0 * matrix[1] + vec.1 * matrix[4] + vec.2 * matrix[7],
        vec.0 * matrix[2] + vec.1 * matrix[5] + vec.2 * matrix[8]
     )
}

fn vec_sub(lhs: Coordinates, rhs: Coordinates) -> Coordinates {
    (lhs.0-rhs.0, lhs.1-rhs.1, lhs.2-rhs.2)
}

fn vec_add(lhs: Coordinates, rhs: Coordinates) -> Coordinates {
    (lhs.0+rhs.0, lhs.1+rhs.1, lhs.2+rhs.2)
}

fn manhattan_distance(lhs: Coordinates, rhs: Coordinates) -> usize {
    lhs.0.abs_diff(rhs.0) + lhs.1.abs_diff(rhs.1) + lhs.2.abs_diff(rhs.2)
}

fn min_distance(lhs: Coordinates, rhs: Coordinates) -> usize {
    lhs.0.abs_diff(rhs.0).min(lhs.1.abs_diff(rhs.1)).min(lhs.2.abs_diff(rhs.2))
}

fn max_distance(lhs: Coordinates, rhs: Coordinates) -> usize {
    lhs.0.abs_diff(rhs.0).max(lhs.1.abs_diff(rhs.1)).max(lhs.2.abs_diff(rhs.2))
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
        assert_eq!(run(&sample_input), Ok((79, 3621)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((430, 11860)));
    }
}
