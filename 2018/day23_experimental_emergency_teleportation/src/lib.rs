use std::collections::BTreeSet;

struct Bot {
    pos: (isize, isize, isize),
    range: usize,
}

impl From<&str> for Bot {
    fn from(value: &str) -> Self {
        let components: Vec<_> = value.split(&['<', ',', '>', '=']).collect();
        assert_eq!(components.len(), 8);
        
        Self { 
            pos: (components[2].parse().unwrap(), components[3].parse().unwrap(), components[4].parse().unwrap()),
            range: components[7].parse().unwrap(), 
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Cube {
    bot_count: usize,
    x_min: isize,
    y_min: isize,
    z_min: isize, 
    side_length: isize,
}

impl Cube {
    fn split(&self) -> [Self; 8] {
        let x_min = self.x_min;
        let y_min = self.y_min;
        let z_min = self.z_min;
        let x_mid = x_min+(self.side_length+1)/2;
        let y_mid = y_min+(self.side_length+1)/2;
        let z_mid = z_min+(self.side_length+1)/2;

        [
            Self {bot_count: 0, x_min, y_min, z_min, side_length: self.side_length/2}, // lll
            Self {bot_count: 0, x_min, y_min, z_min: z_mid, side_length: self.side_length/2}, // llh
            Self {bot_count: 0, x_min, y_min: y_mid, z_min, side_length: self.side_length/2}, // lhl
            Self {bot_count: 0, x_min: x_mid, y_min, z_min, side_length: self.side_length/2}, // hll
            Self {bot_count: 0, x_min, y_min: y_mid, z_min: z_mid, side_length: self.side_length/2}, // lhh
            Self {bot_count: 0, x_min: x_mid, y_min, z_min: z_mid, side_length: self.side_length/2}, // hlh
            Self {bot_count: 0, x_min: x_mid, y_min: y_mid, z_min, side_length: self.side_length/2}, // hhl
            Self {bot_count: 0, x_min: x_mid, y_min: y_mid, z_min: z_mid, side_length: self.side_length/2}, // hhh
        ]
    }

    fn bots_in_range(&self, bots: &[Bot]) -> usize {
        let x_mid = self.x_min+(self.side_length+1)/2;
        let y_mid = self.y_min+(self.side_length+1)/2;
        let z_mid = self.z_min+(self.side_length+1)/2;
        let mid = (x_mid, y_mid, z_mid);
        let delta = (3*self.side_length/2) as usize;

        bots.iter().filter(|bot| dist(mid, bot.pos) <= bot.range + delta).count()
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let bots: Vec<_> = input.lines().map(Bot::from).collect();
    let strongest = bots.iter().max_by_key(|bot| bot.range).unwrap();
    let first = bots.iter().filter(|bot| dist(bot.pos, strongest.pos) <= strongest.range).count();
    let second = dist_best_covered(&bots);
    (first, second)
}

fn dist_best_covered(bots: &[Bot]) -> usize {
    let mut best = Vec::new();
    let mut best_count = 0;

    let x_min = bots.iter().map(|bot| bot.pos.0).min().unwrap();
    let x_range = bots.iter().map(|bot| bot.pos.0).max().unwrap() - x_min;
    let y_min = bots.iter().map(|bot| bot.pos.1).min().unwrap();
    let y_range = bots.iter().map(|bot| bot.pos.1).max().unwrap() - y_min;
    let z_min = bots.iter().map(|bot| bot.pos.2).min().unwrap();
    let z_range = bots.iter().map(|bot| bot.pos.2).max().unwrap() - z_min;
    let side_length = x_range.max(y_range).max(z_range);
    let all = Cube { bot_count: bots.len(), x_min, y_min, z_min, side_length };

    let mut cubes = BTreeSet::from([all]);
    loop {
        let current = cubes.pop_last().unwrap();
        if current.bot_count < best_count {
            break;
        }
        if current.side_length < 1 {
            if current.bot_count > best_count {
                best_count = current.bot_count;
                best = Vec::new();
            }
            best.push((current.x_min, current.y_min, current.z_min));
            continue;
        }
        for sub in current.split().iter_mut() {
            let bot_count = sub.bots_in_range(bots);
            sub.bot_count = bot_count;
            cubes.insert(sub.clone());
        }
    }

    best.iter().map(|(x, y, z)| x.unsigned_abs() + y.unsigned_abs() + z.unsigned_abs()).min().unwrap()
}

fn dist((x0, y0, z0): (isize, isize, isize), (x1, y1, z1): (isize, isize, isize)) -> usize {
    x0.abs_diff(x1) + y0.abs_diff(y1) + z0.abs_diff(z1)
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
        assert_eq!(run(&sample_input), (6, 36));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (294, 88894457));
    }
}
