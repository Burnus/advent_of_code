use std::{collections::{HashMap, HashSet}, u8};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Mode{ Easy, Hard }

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Character {
    hit_points: i8,
    damage: i8,
    armor: i8,
    mana: isize,
    poison_duration_left: u8,
    shield_duration_left: u8,
    recharge_duration_left: u8
}

impl Character {
    pub fn parse(input: &str) -> Self {
        let mut hit_points = 0;
        let mut damage = 0;
        let mut armor = 0;

        input.lines().for_each(|line| {
            let (key, value) = line.rsplit_once(' ').unwrap();
            match key {
                "Hit Points:" => { hit_points = value.parse().unwrap(); },
                "Damage:" => { damage = value.parse().unwrap(); },
                "Armor:" => { armor = value.parse().unwrap(); },
                _ => panic!("Unexpected key: {key}"),
            }
        });

        Self {
            hit_points,
            damage,
            armor,
            mana: 500,
            poison_duration_left: 0,
            shield_duration_left: 0,
            recharge_duration_left: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Spell {
    mana_cost: isize,
    instant_damage: i8,
    instant_heal: i8,
    effect_duration: u8,
    damage_over_time: i8,
    mana_over_time: isize,
    armor_bonus: i8,
}

impl Spell {
    fn magic_missile() -> Self {
        Self {
            mana_cost: 53,
            instant_damage: 4, 
            instant_heal: 0, 
            effect_duration: 0, 
            damage_over_time: 0, 
            mana_over_time: 0, 
            armor_bonus: 0,
        }
    }

    fn drain() -> Self {
        Self {
            mana_cost: 73,
            instant_damage: 2, 
            instant_heal: 2, 
            effect_duration: 0, 
            damage_over_time: 0, 
            mana_over_time: 0, 
            armor_bonus: 0,
        }
    }

    fn shield() -> Self {
        Self {
            mana_cost: 113,
            instant_damage: 0, 
            instant_heal: 0, 
            effect_duration: 6,
            damage_over_time: 0, 
            mana_over_time: 0, 
            armor_bonus: 7,
        }
    }

    fn poison() -> Self {
        Self {
            mana_cost: 173,
            instant_damage: 0, 
            instant_heal: 0, 
            effect_duration: 6, 
            damage_over_time: 3, 
            mana_over_time: 0, 
            armor_bonus: 0,
        }
    }

    fn recharge() -> Self {
        Self {
            mana_cost: 229,
            instant_damage: 0, 
            instant_heal: 0, 
            effect_duration: 5, 
            damage_over_time: 0, 
            mana_over_time: 101, 
            armor_bonus: 0,
        }
    }

}

pub fn run(boss: &Character) -> (isize, isize) {
        let player = Character { hit_points: 50, damage: 0, armor: 0, mana: 500, poison_duration_left: 0, shield_duration_left: 0, recharge_duration_left: 0 };
        let first = a_star_search(&player, boss, Mode::Easy);
        let second = a_star_search(&player, boss, Mode::Hard);
    (first, second)
}

fn a_star_search(player: &Character, boss: &Character, mode: Mode) -> isize {
    // The set of discovered nodes. Initially only the start node is known.
    let mut open_set = HashSet::from([(*player, *boss)]);
    // A map from a node to its lowest known costs.
    let mut g_score = HashMap::from([((*player, *boss), 0)]);
    // Estimated costs of each path (f = g+h). We set h to boss' HP * 3/2, since this is the max
    // number of rounds the fight can go on (we must deal at least 2 damage every 3 rounds,
    // otherwise we'd have to stack non-damaging spells, which we aren't allowed to).
    let mut f_score = HashMap::from([((*player, *boss), (boss.hit_points as isize)*3/2)]);

    while !open_set.is_empty() {
        let current = open_set.iter()
            .min_by(|&a, &b| f_score.get(a).unwrap().cmp(f_score.get(b).unwrap()))
            .unwrap().to_owned();
        if current.1.hit_points <= 0 {
            return *g_score.get(&current).unwrap();
        }
        open_set.remove(&current);
        for spell in available_spells() {
            let mut this_player = current.0;
            let mut this_boss = current.1;
            if fight(&mut this_player, &mut this_boss, &spell, mode) {
                let tentative_g_score = g_score.get(&current).unwrap() + spell.mana_cost;
                let current_g_score = *g_score.get(&(this_player, this_boss)).unwrap_or(&isize::MAX);
                if tentative_g_score < current_g_score {
                    g_score.insert((this_player, this_boss), tentative_g_score);
                    f_score.insert((this_player, this_boss), tentative_g_score + (this_boss.hit_points as isize)*3/2);
                    open_set.insert((this_player, this_boss));
                }
            }
        }
    }
    panic!("This matchup is unwinnable.")
}


fn fight(player: &mut Character, boss: &mut Character, new_spell: &Spell, mode: Mode) -> bool {
    // Player's Turn
    if mode == Mode::Hard {
        player.hit_points -= 1;
        if player.hit_points == 0 {
            return false;
        }
    }
    boss.hit_points -= new_spell.instant_damage;
    if player.poison_duration_left > 0 {
        boss.hit_points -= Spell::poison().damage_over_time;
        player.poison_duration_left -= 1;
    }
    player.hit_points += new_spell.instant_heal;
    player.mana -= new_spell.mana_cost;
    if player.recharge_duration_left > 0 {
        player.mana += Spell::recharge().mana_over_time;
        player.recharge_duration_left -= 1;
    }
    if player.shield_duration_left > 0 {
        player.shield_duration_left -= 1;
    }
    if new_spell.effect_duration > 0 {
        // Ensure we don't apply spell effects that are already active. Otherwise set the
        // appropriate duration.
        if new_spell.armor_bonus > 0 {
            if player.shield_duration_left > 0 {
                return false;
            } else {
                player.shield_duration_left = new_spell.effect_duration;
            }
        } else if new_spell.damage_over_time > 0 {
            if player.poison_duration_left > 0 {
                return false;
            } else {
                player.poison_duration_left = new_spell.effect_duration;
            }
        // else this must be a Recharge Spell
        } else if player.recharge_duration_left > 0 {
            return false;
        } else {
            player.recharge_duration_left = new_spell.effect_duration;
        }
    }
    
    if player.mana < 0 {
        // We lose because we didn't have enough Mana to cast this spell.
        return false;
    }

    // Boss' Turn
    if player.poison_duration_left > 0{
        boss.hit_points -= Spell::poison().damage_over_time;
        player.poison_duration_left -= 1;
    }
    if player.recharge_duration_left > 0 {
        player.mana += Spell::recharge().mana_over_time;
        player.recharge_duration_left -= 1;
    }
    player.hit_points -= boss.damage;
    if player.shield_duration_left > 0 {
        player.hit_points += Spell::shield().armor_bonus.min(boss.damage-1);
        player.shield_duration_left -= 1;
    }

    // we win if we are alive or the boss is dead (because they would already have been before
    // their attack – hence we should still be alive).
    player.hit_points>0 || boss.hit_points<=0
}

fn available_spells() -> [Spell; 5] {
    [
        Spell::magic_missile(),
        Spell::drain(),
        Spell::shield(),
        Spell::poison(),
        Spell::recharge(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
    }

    #[test]
    fn test_sample_1() {
        let mut boss = Character { hit_points: 13, damage: 8, armor: 0, mana: 0, poison_duration_left: 0, recharge_duration_left: 0, shield_duration_left: 0 };
        let mut player = Character { hit_points: 10, damage: 0, armor: 0, mana: 250, poison_duration_left: 0, recharge_duration_left: 0, shield_duration_left: 0 };
        let available_spells = available_spells();
        let expected = [
            (3, (2, 10)),
            (0, (-6, 0)),
        ];
        for round in expected {
            assert!(fight(&mut player, &mut boss, &available_spells[round.0].clone(), Mode::Easy));
            assert_eq!((player.hit_points, boss.hit_points), round.1);
        }
    }

    #[test]
    fn test_sample_2() {
        let mut boss = Character { hit_points: 14, damage: 8, armor: 0, mana: 0, poison_duration_left: 0, recharge_duration_left: 0, shield_duration_left: 0 };
        let mut player = Character { hit_points: 10, damage: 0, armor: 0, mana: 250, poison_duration_left: 0, shield_duration_left: 0, recharge_duration_left: 0 };
        let available_spells = available_spells();
        let expected = [
            (4, (2, 14)),
            (2, (1, 14)),
            (1, (2, 12)),
            (3, (1, 9)),
            (0, (-7, -1)),
        ];
        for round in expected {
            assert!(fight(&mut player, &mut boss, &available_spells[round.0].clone(), Mode::Easy));
            assert_eq!((player.hit_points, boss.hit_points), round.1);
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        let boss = Character::parse(&challenge_input);
        assert_eq!(run(&boss), (953, 1289));
    }
}
