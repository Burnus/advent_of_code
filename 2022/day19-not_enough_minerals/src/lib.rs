use core::fmt::Display;
use std::num::ParseIntError;
use std::collections::{HashMap, VecDeque};

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

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot_cost: u8,
    clay_robot_cost: u8,
    obsidian_robot_ore_cost: u8,
    obsidian_robot_clay_cost: u8,
    geode_robot_ore_cost: u8,
    geode_robot_obsidian_cost: u8,
}

impl<'a> TryFrom<&'a str> for Blueprint {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let components: Vec<&str> = value.split(' ').collect();
        if components.len() != 32 {
            return Err(Self::Error::LineMalformed(value));
        }
        Ok(Self {
            id: components[1][..components[1].len()-1].parse()?,
            ore_robot_cost: components[6].parse()?,
            clay_robot_cost: components[12].parse()?,
            obsidian_robot_ore_cost: components[18].parse()?,
            obsidian_robot_clay_cost: components[21].parse()?, 
            geode_robot_ore_cost: components[27].parse()?, 
            geode_robot_obsidian_cost: components[30].parse()?, 
        })
    }
}

fn more_robots_required(robot_count: u8, stock: u8, max_demand: u8, time_remaining: u8) -> bool {
    (robot_count as usize * time_remaining as usize + stock as usize) < (max_demand as usize * time_remaining as usize)
}

impl Blueprint {
    fn collect_geodes(&self, time: u8) -> u8 {
        self.try_bfs(time)
    }

    fn try_bfs(&self, time: u8) -> u8 {
        let mut open_set = VecDeque::from([(Inventory::new(), time)]);
        let mut mem = HashMap::new();
        let mut best = 0;

        while let Some((inventory, time_remaining)) = open_set.pop_front() {
            if time_remaining == 0 {
                best = best.max(inventory.geodes);
            } else if let Some(best_time) = mem.get(&inventory.as_arr()) {
                if *best_time >= time_remaining {
                    continue;
                }
            } else {
                mem.insert(inventory.as_arr(), time_remaining);
                // let inventory = inventory.collect();

                // Always buy a Geode Robot if we can afford it and there is at least 1 unit of time remaining
                // (so it will produce at least once)
                if time_remaining > 1 && inventory.ore >= self.geode_robot_ore_cost && inventory.obsidian >= self.geode_robot_obsidian_cost {
                    let mut new_inventory = inventory;
                    new_inventory.collect();
                    new_inventory.ore -= self.geode_robot_ore_cost;
                    new_inventory.obsidian -= self.geode_robot_obsidian_cost;
                    new_inventory.geode_robots += 1;
                    open_set.push_back((new_inventory, time_remaining-1));
                } else {
                    // Save Ressources only if there is any robot we can't afford, but we already produce the
                    // required ressource, or we are close to the end
                    if time_remaining < 4 || inventory.ore < *[self.ore_robot_cost, self.clay_robot_cost, self.obsidian_robot_ore_cost, self.geode_robot_ore_cost].iter().max().unwrap() || 
                        inventory.clay < self.obsidian_robot_clay_cost && inventory.clay_robots > 0 ||  
                        inventory.obsidian < self.geode_robot_obsidian_cost && inventory.obsidian_robots > 0
                        {
                            let mut new_inventory = inventory;
                            new_inventory.collect();
                            open_set.push_back((new_inventory, time_remaining-1));
                        }
                    // Buy an Ore Robot if
                    // - we can afford it, and
                    // - we don't already produce enough Ore for any other Robot each round, and
                    // - there are at least 2 rounds left (1 to produce and buy a Geode Robot, 1 for that
                    // to produce).
                    // if time_remaining > 2 && inventory.ore >= self.ore_robot_cost && inventory.ore_robots < *[self.ore_robot_cost, self.clay_robot_cost, self.obsidian_robot_ore_cost, self.geode_robot_ore_cost].iter().max().unwrap() {
                    if time_remaining > 2 && inventory.ore >= self.ore_robot_cost && more_robots_required(inventory.ore_robots, inventory.ore, *[self.ore_robot_cost, self.clay_robot_cost, self.obsidian_robot_ore_cost, self.geode_robot_ore_cost].iter().max().unwrap(), time_remaining-1) {
                        let mut new_inventory = inventory;
                        new_inventory.collect();
                        new_inventory.ore -= self.ore_robot_cost;
                        new_inventory.ore_robots += 1;
                        open_set.push_back((new_inventory, time_remaining-1));
                    }
                    // Buy a Clay Robot if
                    // - we can afford it, and
                    // - we don't already produce enough Clay for an Obsidian Robot each round, and
                    // - there are at least 3 rounds left (1 to produce and buy an Obsidian Robot, 1 for that
                    // to produce and buy a Geode Robot, and 1 for that to produce).
                    // if time_remaining > 3 && inventory.ore >= self.clay_robot_cost && inventory.clay_robots < self.obsidian_robot_clay_cost {
                    if time_remaining > 3 && inventory.ore >= self.clay_robot_cost && more_robots_required(inventory.clay_robots, inventory.clay, self.obsidian_robot_clay_cost, time_remaining-1) {
                        let mut new_inventory = inventory;
                        new_inventory.collect();
                        new_inventory.ore -= self.clay_robot_cost;
                        new_inventory.clay_robots += 1;
                        open_set.push_back((new_inventory, time_remaining-1));
                    }
                    // Buy an Obsidian Robot if
                    // - we can afford it, and
                    // - we don't already produce enough Obsidian for a Geode Robot each round, and
                    // - there are at least 2 rounds left (1 to produce and buy a Geode Robot, and 1 for that
                    // to produce).
                    // if time_remaining > 2 && inventory.ore >= self.obsidian_robot_ore_cost && inventory.clay >= self.obsidian_robot_clay_cost && inventory.obsidian_robots < self.geode_robot_obsidian_cost {
                    if time_remaining > 2 && inventory.ore >= self.obsidian_robot_ore_cost && inventory.clay >= self.obsidian_robot_clay_cost && more_robots_required(inventory.obsidian_robots, inventory.obsidian, self.geode_robot_obsidian_cost, time_remaining-1) {
                        let mut new_inventory = inventory;
                        new_inventory.collect();
                        new_inventory.ore -= self.obsidian_robot_ore_cost;
                        new_inventory.clay -= self.obsidian_robot_clay_cost;
                        new_inventory.obsidian_robots += 1;
                        open_set.push_back((new_inventory, time_remaining-1));
                    }
                }
            }
        }
        best
    }
}

#[derive(Copy, Clone, Debug)]
struct Inventory {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geodes: u8,
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,
}

impl Inventory {
    fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    fn collect(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }

    fn as_arr(&self) -> [u8;8] {
        [
            self.ore,
            self.clay,
            self.obsidian,
            self.geodes,
            self.ore_robots,
            self.clay_robots,
            self.obsidian_robots,
            self.geode_robots,
        ]
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let blueprints: Vec<Blueprint> = input.lines().map(Blueprint::try_from).collect::<Result<Vec<_>, _>>()?;
    
    let first = blueprints.iter()
        .map(|blueprint| blueprint.id * blueprint.collect_geodes(24) as usize)
        .sum();

    // let second = 0;
    let second = blueprints.iter()
        .take(3)
        .map(|blueprint| blueprint.collect_geodes(32) as usize)
        .product();

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
        assert_eq!(run(&sample_input), Ok((33, 3472)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((978, 15939)));
    }
}
