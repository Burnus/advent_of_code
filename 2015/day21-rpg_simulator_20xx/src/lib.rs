#[derive(Clone)]
pub struct Character {
    hit_points: i8,
    damage: i8,
    armor: i8,
}

impl Character {
    fn parse(input: &str) -> Self {
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
        }
    }
}

pub fn run(boss: &Character) -> (usize, usize) {
    get_min_max_items(boss)
}

fn get_min_max_items(boss: &Character) -> (usize, usize) {
    let weapons = [
            (8, 4, 0),
            (10, 5, 0),
            (25, 6, 0),
            (40, 7, 0),
            (74, 8, 0),
        ];
    let armors = [
            (0, 0, 0),
            (13, 0, 1),
            (31, 0, 2),
            (53, 0, 3),
            (75, 0, 4),
            (102, 0, 5),
        ];
    let rings = [
            (0, 0, 0),
            (0, 0, 0),
            (25, 1, 0),
            (50, 2, 0),
            (100, 3, 0),
            (20, 0, 1),
            (40, 0, 2),
            (80, 0, 3),
        ];
    let mut min_gold = usize::MAX;
    let mut max_gold = 0;

    weapons.into_iter().for_each(|weapon| {
        armors.into_iter().for_each(|armor| {
            rings.into_iter().enumerate().for_each(|(idx_1, ring_1)| {
                rings.into_iter().skip(idx_1 + 1).for_each(|ring_2| {
                    match try_equipment(&[weapon, armor, ring_1, ring_2], boss) {
                        (true, gold) if gold < min_gold => { min_gold = gold; },
                        (false, gold) if gold > max_gold => { max_gold = gold; },
                        _ => (),
                    }
                });
            });
        });
    });
    (min_gold, max_gold)
}

fn try_equipment(items: &[(usize, i8, i8); 4], boss: &Character) -> (bool, usize) {
    let mut this_player = Character {
        hit_points: 100,
        damage: items.iter().map(|i| i.1).sum(),
        armor: items.iter().map(|i| i.2).sum(),
    };
    let mut this_boss = boss.clone();
    loop {
        let (player_alive, boss_alive) = fight(&mut this_player,&mut this_boss);
        if !player_alive || !boss_alive {
            return (player_alive, items.iter().map(|i| i.0).sum());
        }
    }
}

fn fight(player: &mut Character, boss: &mut Character) -> (bool, bool) {
    boss.hit_points -= (player.damage - boss.armor).max(1);
    player.hit_points -= (boss.damage - player.armor).max(1);

    (player.hit_points>0 || boss.hit_points<=0, boss.hit_points>0)
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
        let mut boss = Character::parse(&sample_input);
        let mut player = Character { hit_points: 8, damage: 5, armor: 5 };
        let expected = [
            (8, 12),
            (6, 9),
            (4, 6),
            (2, 3),
            (0, 0),
        ];
        let (mut player_alive, mut boss_alive) = (true, true);
        for expected_hp in expected {
            assert_eq!((player.hit_points, boss.hit_points), expected_hp);
            (player_alive, boss_alive) = fight(&mut player, &mut boss);
        }
        assert_eq!((player_alive, boss_alive), (true, false));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        let boss = Character::parse(&challenge_input);
        assert_eq!(run(&boss), (111, 188));
    }
}
