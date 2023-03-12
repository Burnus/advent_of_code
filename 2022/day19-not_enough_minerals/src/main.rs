use std::{fs, collections::HashMap};

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

impl Blueprint {
    fn parse(line: &str) -> Self {
        let components: Vec<&str> = line.split(' ').collect();
        if components.len() != 32 {
            panic!("{line} does not have 32 components.");
        }
        Self {
            id: components[1][..components[1].len()-1].parse().unwrap(),
            ore_robot_cost: components[6].parse().unwrap(),
            clay_robot_cost: components[12].parse().unwrap(),
            obsidian_robot_ore_cost: components[18].parse().unwrap(),
            obsidian_robot_clay_cost: components[21].parse().unwrap(), 
            geode_robot_ore_cost: components[27].parse().unwrap(), 
            geode_robot_obsidian_cost: components[30].parse().unwrap(), 
        }
    }

    fn collect_geodes(&self, time: u8) -> u8 {
        let best = self.try_all(&Inventory::new(), time, &mut HashMap::new());
        println!("Best Result for Blueprint {} is {}", self.id, best);
        best
    }

    fn try_all(&self, inventory: &Inventory, time_remaining: u8, mem: &mut HashMap<[u8;8], u8>) -> u8 {
        if let Some(best_time) = mem.get(&inventory.as_arr()) {
            if *best_time >= time_remaining && time_remaining > 1 {
                return 0;
            }
        } 
        mem.insert(inventory.as_arr(), time_remaining); 
        if time_remaining == 0 {
            return inventory.geodes;
        }
        let mut scores = Vec::new();
        // branch
        if time_remaining < 4 || inventory.ore < *[self.ore_robot_cost, self.clay_robot_cost, self.obsidian_robot_ore_cost, self.geode_robot_ore_cost].iter().max().unwrap() || 
            inventory.clay < self.obsidian_robot_clay_cost && inventory.clay_robots > 0 ||  
            inventory.obsidian < self.geode_robot_obsidian_cost && inventory.obsidian_robots > 0
            {
                let mut new_inventory = *inventory;
                new_inventory.collect();
                scores.push(self.try_all(&new_inventory, time_remaining-1, mem));
            }
        if time_remaining > 2 && inventory.ore >= self.ore_robot_cost && inventory.ore_robots < *[self.ore_robot_cost, self.clay_robot_cost, self.obsidian_robot_ore_cost, self.geode_robot_ore_cost].iter().max().unwrap() {
            let mut new_inventory = *inventory;
            new_inventory.collect();
            new_inventory.ore -= self.ore_robot_cost;
            new_inventory.ore_robots += 1;
            scores.push(self.try_all(&new_inventory, time_remaining-1, mem));
        }
        if time_remaining > 3 && inventory.ore >= self.clay_robot_cost && inventory.clay_robots < self.obsidian_robot_clay_cost {
            let mut new_inventory = *inventory;
            new_inventory.collect();
            new_inventory.ore -= self.clay_robot_cost;
            new_inventory.clay_robots += 1;
            scores.push(self.try_all(&new_inventory, time_remaining-1, mem));
        }
        if time_remaining > 2 && inventory.ore >= self.obsidian_robot_ore_cost && inventory.clay >= self.obsidian_robot_clay_cost && inventory.obsidian_robots < self.geode_robot_obsidian_cost {
            let mut new_inventory = *inventory;
            new_inventory.collect();
            new_inventory.ore -= self.obsidian_robot_ore_cost;
            new_inventory.clay -= self.obsidian_robot_clay_cost;
            new_inventory.obsidian_robots += 1;
            scores.push(self.try_all(&new_inventory, time_remaining-1, mem));
        }
        if time_remaining > 1 && inventory.ore >= self.geode_robot_ore_cost && inventory.obsidian >= self.geode_robot_obsidian_cost {
            let mut new_inventory = *inventory;
            new_inventory.collect();
            new_inventory.ore -= self.geode_robot_ore_cost;
            new_inventory.obsidian -= self.geode_robot_obsidian_cost;
            new_inventory.geode_robots += 1;
            scores.push(self.try_all(&new_inventory, time_remaining-1, mem));
        }
        scores.iter().cloned().max().unwrap()
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

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn main() {
    let blueprints: Vec<Blueprint> = read_file("input").lines().map(Blueprint::parse).collect();
    
    let quality_level: usize = blueprints.iter()
        .map(|blueprint| blueprint.id * blueprint.collect_geodes(24) as usize)
        .sum();

    println!("The sum of all of our quality levels is {quality_level}"); // should be 33 for the sample_input

    let max_score: usize = blueprints.iter()
        .take(3)
        .map(|blueprint| blueprint.collect_geodes(32) as usize)
        .product();

    println!("With added time, the remaining blueprints multiply to {max_score}.");
}

#[test]
fn sample_input() {
    let blueprints: Vec<_>= read_file("tests/sample_input").lines().map(Blueprint::parse).collect();

    let quality_level: usize = blueprints.iter().map(|b| b.id * b.collect_geodes(24) as usize).sum();
    let max_score: usize = blueprints.iter().take(3).map(|b| b.collect_geodes(32) as usize).product();
    assert_eq!(quality_level, 33);
    assert_eq!(max_score, 62);
}
#[test]
fn challenge_input() {
    let blueprints: Vec<_>= read_file("tests/input").lines().map(Blueprint::parse).collect();

    let quality_level: usize = blueprints.iter().map(|b| b.id * b.collect_geodes(24) as usize).sum();
    let max_score: usize = blueprints.iter().take(3).map(|b| b.collect_geodes(32) as usize).product();
    assert_eq!(quality_level, 978);
    assert_eq!(max_score, 15939);
} 
