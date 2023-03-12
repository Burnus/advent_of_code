#[derive(Clone)]
struct Light {
    state: bool,
}

#[derive(Clone)]
struct Grid {
    lights: Vec<Vec<Light>>,
}

impl Grid {
    fn from(input: &str) -> Self {
        Self {
            lights: input.lines()
                        .map(|line| line.chars()
                                        .map(|c| Light { state: c == '#' })
                                        .collect())
                        .collect() 
        }
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let x_min = x.saturating_sub(1);
        let x_max = (x+1).min(self.lights[0].len()-1);
        let y_min = y.saturating_sub(1);
        let y_max = (y+1).min(self.lights.len()-1);

        let mut out = Vec::new();
        for c in x_min..=x_max {
            for r in y_min..=y_max {
                if c != x || r != y {
                    out.push((c, r));
                }
            }
        }
        out
    }

    fn step(&mut self, broken: bool) {
        let old_grid = self.clone();
        for (row_idx, row) in self.lights.iter_mut().enumerate() {
            for (col_idx, light) in row.iter_mut().enumerate() {
                let this_state = light.state;
                let neighbours_on = old_grid.get_neighbours(col_idx, row_idx).iter().filter(|n| old_grid.lights[n.1][n.0].state).count();

                light.state = match (this_state, neighbours_on) {
                    (true, n) if (2..=3).contains(&n) => true,
                    (true, _) => false,
                    (false, 3) => true,
                    (false, _) => false,
                };
            }
        }
        if broken {
            self.broken_on();
        }
    }

    fn broken_on(&mut self) {
        for c in [0, self.lights[0].len()-1] {
            for r in [0, self.lights.len()-1] {
                self.lights[r][c].state = true;
            }
        }
    }

    fn count_on(&self) -> usize {
        self.lights.iter().map(|r| r.iter().filter(|l| l.state).count()).sum()
    }
}

pub fn run(input: &str, steps: usize) -> (usize, usize) {
    let mut grid = Grid::from(input);
    let mut grid2 = grid.clone();
    grid2.broken_on();
    for _ in 0..steps {
        grid.step(false);
        grid2.step(true);
    }
    let first = grid.count_on();
    let second = grid2.count_on(); 
    (first, second)
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
        let expected = [ (15, 17), (11, 18), (8, 18), (4, 18), (4, 14), (4, 17) ];
        for (idx, lights) in expected.into_iter().enumerate() {
            assert_eq!(run(&sample_input, idx), lights);
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input, 100), (1061, 1006));
    }
}
