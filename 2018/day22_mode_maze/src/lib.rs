use std::collections::{HashMap, HashSet, BTreeSet};

struct Regions {
    erosion_levels: HashMap<(usize, usize), usize>,
    target: (usize, usize),
    depth: usize,
}

impl Regions {
    fn get_geo_index(&mut self, (x, y): (usize, usize)) -> usize {
        if [(0, 0), self.target].contains(&(x, y)) {
            0
        } else if y == 0 {
            x * 16807
        } else if x == 0 {
            y * 48271
        } else {
            self.get_erosion_level((x-1, y)) * self.get_erosion_level((x, y-1))
        }
    }

    fn get_erosion_level(&mut self, coordinates: (usize, usize)) -> usize {
        if let Some(res) = self.erosion_levels.get(&coordinates) {
            *res
        } else {
            let res = (self.get_geo_index(coordinates) + self.depth) % 20183;
            self.erosion_levels.insert(coordinates, res);
            res
        }
    }

    fn get_type_score(&mut self, coordinates: (usize, usize)) -> usize {
        self.get_erosion_level(coordinates)%3
    }

    fn get_neighbours(&mut self, current: NavigationState) -> Vec<NavigationState> {
        let (x, y) = current.coordinates;

        // Determine the other tool allowed in this terrain:
        // Their numbers all add up to 3 (0+1+2). Substract the one we already have equiped (we
        // make sure this is allowed below) and the one not allowed here (which equals the terrain
        // score) and we have left the allowed one.
        let other_tool = 3-current.tool-self.get_type_score(current.coordinates);
        
        // We can always stay in place and switch tools.
        let mut res = Vec::from([NavigationState{
            coordinates: (current.coordinates), 
            tool: other_tool, 
            time_elapsed: current.time_elapsed+7,
            projected_total: current.time_elapsed+7 + x.abs_diff(self.target.0) + y.abs_diff(self.target.1) + 7*other_tool.abs_diff(1),
        }]);

        // For all directions (barring negative x and y): If our current tool is allowed there, we
        // can go there in 1 minute.
        if x>0 && self.get_type_score((x-1, y)) != current.tool {
            res.push(NavigationState { coordinates: (x-1, y), tool: current.tool, time_elapsed: current.time_elapsed+1, projected_total: current.time_elapsed+1 + (x-1).abs_diff(self.target.0) + y.abs_diff(self.target.1) + 7*(current.tool.abs_diff(1)) });
        }
        if self.get_type_score((x+1, y)) != current.tool {
            res.push(NavigationState { coordinates: (x+1, y), tool: current.tool, time_elapsed: current.time_elapsed+1, projected_total: current.time_elapsed+1 + (x+1).abs_diff(self.target.0) + y.abs_diff(self.target.1) + 7*(current.tool.abs_diff(1)) });
        }
        if y>0 && self.get_type_score((x, y-1)) != current.tool {
            res.push(NavigationState { coordinates: (x, y-1), tool: current.tool, time_elapsed: current.time_elapsed+1, projected_total: current.time_elapsed+1 + x.abs_diff(self.target.0) + (y-1).abs_diff(self.target.1) + 7*(current.tool.abs_diff(1)) });
        }
        if self.get_type_score((x, y+1)) != current.tool {
            res.push(NavigationState { coordinates: (x, y+1), tool: current.tool, time_elapsed: current.time_elapsed+1, projected_total: current.time_elapsed+1 + x.abs_diff(self.target.0) + (y+1).abs_diff(self.target.1) + 7*(current.tool.abs_diff(1)) });
        }
        res
    }
}

#[derive(Debug, PartialEq, Eq)]
struct NavigationState {
    coordinates: (usize, usize),
    // the tool number equals the type score of the terrain where they aren't allowed, so we can
    // reuse that for determining the allowed tools
    tool: usize,
    time_elapsed: usize,
    // projected_total is the time it took to get here + the manhattan distance between current and target + 7 if the current tool
    // is not 1 (Torch). This is guaranteed to be lower than or equal to the actual time, since we
    // will never find a faster way here by visiting later and the other summands are the actual
    // time remaining in the best case scenario.
    projected_total: usize,
}

impl Ord for NavigationState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.projected_total.cmp(&self.projected_total).then_with(|| other.coordinates.0.cmp(&self.coordinates.0)).then_with(|| other.coordinates.1.cmp(&self.coordinates.1)).then_with(|| other.tool.cmp(&self.tool))
    }
}

impl PartialOrd for NavigationState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let lines: Vec<_> = input.lines().collect();
    let depth = lines[0].split_whitespace().nth(1).unwrap().parse().unwrap();
    let target_components: Vec<_> = lines[1].split_whitespace().nth(1).unwrap().split(',').collect();
    let target = (target_components[0].parse().unwrap(), target_components[1].parse().unwrap());
    let mut regions = Regions {
        target,
        depth,
        erosion_levels: HashMap::new(),
    };
    let first = (0..=target.0).map(|x| (0..=target.1).map(|y| regions.get_type_score((x, y))).sum::<usize>()).sum();
    let second = shortest_path(&mut regions);
    (first, second)
}


fn shortest_path(regions: &mut Regions) -> usize {
    let starting = NavigationState {
        coordinates: (0, 0),
        tool: 1,
        time_elapsed: 0,
        projected_total: regions.target.0 + regions.target.1,
    };
    let mut visited = HashSet::new();
    let mut open = BTreeSet::from([starting]);
    loop {
        let current = open.pop_last().unwrap();
        if current.coordinates == regions.target && current.tool == 1 {
            return current.time_elapsed;
        }
        if visited.contains(&(current.coordinates, current.tool)) {
            continue;
        }
        visited.insert((current.coordinates, current.tool));
        regions.get_neighbours(current).into_iter().for_each(|neighbour| {
            open.insert(neighbour);
        });
    }
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
        assert_eq!(run(&sample_input), (114, 45));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (11810, 1015));
    }
}
