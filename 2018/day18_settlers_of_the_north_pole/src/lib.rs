use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum LandUse {
    Open,
    Tree,
    Lumber,
}

struct Area {
    acres: Vec<Vec<LandUse>>,
    temp: Vec<Vec<LandUse>>,
    side_length: usize,
}

impl From<&str> for Area {
    fn from(value: &str) -> Self {
        let acres: Vec<Vec<_>> = value.lines().map(|line| line.chars().map(|c| match c {
                    '.' => LandUse::Open,
                    '|' => LandUse::Tree,
                    '#' => LandUse::Lumber,
                    _ => panic!("Unexpected token: {c}"),
                }).collect()).collect();
        Self {
            temp: acres.to_vec(),
            side_length: acres.len(),
            acres,
        }
    }
}

impl Area {
    fn adjacent(&self, col: usize, row: usize) -> (usize, usize) {
        let mut trees = 0;
        let mut lumber = 0;

        (col.saturating_sub(1)..=(col+1).min(self.side_length-1)).for_each(|x| {
            (row.saturating_sub(1)..=(row+1).min(self.side_length-1)).for_each(|y| {
                if x != col || y != row {
                    match self.acres[y][x] {
                        LandUse::Open => (),
                        LandUse::Tree => trees += 1,
                        LandUse::Lumber => lumber += 1,
                    }
                }
            });
        });
        (trees, lumber)
    }

    fn change(&mut self) {
        (0..self.side_length).for_each(|y| {
            (0..self.side_length).for_each(|x| {
                let old = self.acres[y][x];
                let neighbours = self.adjacent(x, y);
                let new = match (old, neighbours) {
                    (LandUse::Open, (t, _)) if t >= 3 => LandUse::Tree,
                    (LandUse::Tree, (_, l)) if l >= 3 => LandUse::Lumber,
                    (LandUse::Lumber, (t, l)) if t == 0 || l == 0 => LandUse::Open,
                    _ => old,
                };
                self.temp[y][x] = new;
            });
        });
        std::mem::swap(&mut self.temp, &mut self.acres);
    }

    fn count_trees(&self) -> usize {
        self.acres.iter().map(|row| row.iter().filter(|l| **l == LandUse::Tree).count()).sum()
    }

    fn count_lumber(&self) -> usize {
        self.acres.iter().map(|row| row.iter().filter(|l| **l == LandUse::Lumber).count()).sum()
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let mut area = Area::from(input);
    let mut mem = HashMap::new();
    for t in 0..10 {
        mem.insert(area.acres.clone(), t);
        area.change();
    }
    let first = area.count_lumber() * area.count_trees();
    for t in 10.. {
        if let Some(prev) = mem.get(&area.acres.clone()) {
            let period = t - prev;
            for _ in 0..((1000000000-t) % period) {
                area.change();
            }
            let second = area.count_lumber() * area.count_trees();
            return (first, second);
        }
        mem.insert(area.acres.clone(), t);
        area.change();
    }
    unreachable!("The for loop above always runs and only breaks by returning early.");
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
        assert_eq!(run(&sample_input), (1147, 0));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (355918, 202806));
    }
}
