#[derive(PartialEq, Clone)]
enum Faction { ImmuneSystem, Infection }

#[derive(Clone)]
struct Group {
    faction: Faction,
    units: usize,
    hit_points: usize,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
    attack_dmg: usize,
    attack_type: String,
    initiative: usize,
    chosen_target: Option<usize>,
    targeted: bool,
}

impl Group {
    fn effective_power(&self) -> usize {
        self.units * self.attack_dmg
    }

    fn from(line: &str, faction: Faction) -> Self {
        let outer: Vec<_> = line.split(&['(', ';', ')']).collect();
        let left: Vec<_> = outer[0].split_whitespace().collect();
        let atk: Vec<_> = if outer.len() == 1 {
            left[7..].to_vec()
        } else {
            outer[outer.len()-1].split_whitespace().collect()
        };
        let units = left[0].parse().unwrap();
        let hit_points = left[4].parse().unwrap();
        let attack_dmg = atk[5].parse().unwrap();
        let attack_type = atk[6].to_string();
        let initiative = atk[10].parse().unwrap();
        let mut weaknesses = Vec::new();
        let mut immunities = Vec::new();
        if outer.len() > 2 {
            for inner in &outer[1..outer.len()-1] {
                let line: Vec<_> = inner.trim().split(&[' ', ',']).collect();
                match line[0] {
                    "weak" => {
                        for w in line.iter().skip(2).step_by(2) {
                            weaknesses.push(w.to_string());
                        }
                    },
                    "immune" => {
                        for i in line.iter().skip(2).step_by(2) {
                            immunities.push(i.to_string());
                        }
                    },
                    _ => panic!("Unexpected line: {inner}"),
                }
            }
        }
        Self {
            faction,
            hit_points,
            units,
            attack_dmg,
            attack_type,
            initiative,
            weaknesses,
            immunities,
            chosen_target: None,
            targeted: false,
        }
    }
}

#[derive(Clone)]
struct Battle {
    groups: Vec<Group>,
}

impl Battle {
    fn fight(&mut self) -> bool {
        while self.groups.iter().any(|group| group.faction == Faction::ImmuneSystem) && self.groups.iter().any(|group| group.faction == Faction::Infection) {
            let units_before = self.groups.iter().map(|group| group.units).sum::<usize>();
            self.combat_round();
            if units_before == self.groups.iter().map(|group| group.units).sum::<usize>() {
                return false;
            }
        }
        true
    }

    fn combat_round(&mut self) {
        // Target Selection
        self.groups.sort_by(|a, b| b.effective_power().cmp(&a.effective_power()).then_with(|| b.initiative.cmp(&a.initiative)));
        for attacker_idx in 0..self.groups.len() {
            let attacker = &self.groups[attacker_idx];
            let mut possible_targets: Vec<_> = self.groups.iter().enumerate().filter(|(_idx, t)| t.faction != attacker.faction && !t.targeted && !t.immunities.contains(&attacker.attack_type)).collect();
            let weak_targets: Vec<_> = possible_targets.iter().cloned().filter(|(_idx, t)| t.weaknesses.contains(&attacker.attack_type)).collect();
            if !weak_targets.is_empty() {
                possible_targets = weak_targets;
            }
            if !possible_targets.is_empty() {
                let target_idx = possible_targets[0].0;
                self.groups[attacker_idx].chosen_target = Some(target_idx);
                self.groups[target_idx].targeted = true;
            }
        }

        // Attacking
        let mut attackers: Vec<usize> = (0..self.groups.len()).collect();
        attackers.sort_by_key(|idx| usize::MAX - self.groups[*idx].initiative);
        for attacker_idx in attackers {
            let attacker = &self.groups[attacker_idx];
            if let Some(defender_idx) = attacker.chosen_target {
                let mut defender = self.groups[defender_idx].clone();
                let mut damage = attacker.effective_power();
                if defender.weaknesses.contains(&attacker.attack_type) {
                    damage *= 2;
                }
                defender.units = defender.units.saturating_sub(damage / defender.hit_points);
                self.groups[defender_idx].units = defender.units;
                self.groups[defender_idx].targeted = false;
                self.groups[attacker_idx].chosen_target = None;
            }
        }

        // Filter out dead groups
        self.groups.retain(|group| group.units > 0);
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let (immune_system, infection) = input.split_once("\n\n").unwrap();
    let armies = Battle { groups: immune_system.lines().skip(1).map(|line| Group::from(line, Faction::ImmuneSystem)).chain(infection.lines().skip(1).map(|line| Group::from(line, Faction::Infection))).collect() };
    let mut armies_1 = armies.clone();
    armies_1.fight();
    let first = armies_1.groups.iter().map(|group| group.units).sum();
    let mut armies_2 = armies.clone();
    for boost in 1.. {
        armies_2.groups.iter_mut().for_each(|group| {
            if group.faction == Faction::ImmuneSystem {
                group.attack_dmg += boost;
            }
        });
        if armies_2.fight() && armies_2.groups[0].faction == Faction::ImmuneSystem {
            break;
        }
        armies_2 = armies.clone()
    }
    let second = armies_2.groups.iter().map(|group| group.units).sum();
    (first, second)
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
        assert_eq!(run(&sample_input), (5216, 51));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (28976, 3534));
    }
}
