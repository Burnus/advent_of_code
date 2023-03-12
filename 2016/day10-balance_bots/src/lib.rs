#[derive(Clone, Copy)]
enum Destination { Bot(usize), Output(usize) }

struct Bot {
    holding: [u8; 2],
    dest_hi: Destination,
    dest_lo: Destination,
}

impl Bot {
    fn new(dest_hi: Destination, dest_lo: Destination) -> Self {
        Self {
            holding: [0, 0],
            dest_hi,
            dest_lo,
        }
    }
}

pub struct Factory {
    bots: Vec<Bot>,
    outputs: [Vec<u8>; 20]
}

impl Factory {
    pub fn new(instructions: &str) -> Self {
        let mut bots = Vec::new();
        
        instructions.lines().for_each(|line| {
            let components: Vec<_> = line.split(' ').collect();
            match components.len() {
                6 => {
                        let bot_id = components[5].parse::<usize>().unwrap();
                        while bot_id >= bots.len() { bots.push(Bot::new(Destination::Output(3), Destination::Output(3))); }
                        bots[bot_id].holding[0] = components[1].parse().unwrap();
                        bots[bot_id].holding.sort();
                    },
                12 => {
                        let bot_id = components[1].parse::<usize>().unwrap();
                        while bot_id >= bots.len() { bots.push(Bot::new(Destination::Output(3), Destination::Output(3))); }
                        if components[5] == "bot" {
                            bots[bot_id].dest_lo = Destination::Bot(components[6].parse().unwrap());
                        } else {
                            bots[bot_id].dest_lo = Destination::Output(components[6].parse().unwrap());
                        }
                        if components[10] == "bot" {
                            bots[bot_id].dest_hi = Destination::Bot(components[11].parse().unwrap());
                        } else {
                            bots[bot_id].dest_hi = Destination::Output(components[11].parse().unwrap());
                        }
                    },
                _ => panic!("Unable to parse {line} with {} components", components.len()),
            }
        });

        Self {
            bots,
            outputs: Default::default(),
        }
    }

    fn exchange(&mut self, bot: usize) {
        let dest_lo = self.bots[bot].dest_lo;
        let dest_hi = self.bots[bot].dest_hi;
        let mut holding = self.bots[bot].holding;
        match dest_lo {
            Destination::Bot(other_bot) => {
                if self.bots[other_bot].holding[0] == 0 {
                    self.bots[other_bot].holding[0] = holding[0];
                    self.bots[other_bot].holding.sort();
                    holding[0] = 0;
                }
            },
            Destination::Output(out) => {
                eprintln!("Pushing {} to {}", holding[0], out);
                self.outputs[out].push(holding[0]);
                holding[0] = 0;
            }
        }
        match dest_hi {
            Destination::Bot(other_bot) => {
                if self.bots[other_bot].holding[0] == 0 {
                    self.bots[other_bot].holding[0] = holding[1];
                    self.bots[other_bot].holding.sort();
                    holding[0] = 0;
                }
            },
            Destination::Output(out) => {
                eprintln!("Pushing {} to {}", holding[1], out);
                self.outputs[out].push(holding[1]);
                holding[1] = 0;
            }
        }
        holding.sort();
        self.bots[bot].holding = holding;
    }

    pub fn produce(&mut self) {
        while self.outputs[0].is_empty() || self.outputs[1].is_empty() || self.outputs[2].is_empty() {
            for idx in 0..self.bots.len() {
                if self.bots[idx].holding[0] > 0 {
                    self.exchange(idx);
                }
            }
        }
    }

    pub fn compare_until(&mut self, (target_lo, target_hi): (u8, u8)) -> usize {
        loop {
            for idx in 0..self.bots.len() {
                let holding = self.bots[idx].holding;
                if holding == [target_lo, target_hi] {
                    return idx;
                }
                if holding[0] > 0 {
                    self.exchange(idx);
                }
            }
        }
    }
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
        let mut factory = Factory::new(&sample_input);
        assert_eq!(factory.compare_until((3, 5)), 0);
        assert_eq!(factory.outputs, [vec![], vec![2], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]]);
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        let mut factory = Factory::new(&challenge_input);
        assert_eq!(factory.compare_until((17, 61)), 56);
        factory.produce();
        assert_eq!(factory.outputs, [vec![7], vec![59], vec![19], vec![], vec![53], vec![], vec![23], vec![17], vec![11], vec![37], vec![41], vec![2], vec![], vec![31], vec![13], vec![29], vec![47], vec![5], vec![43], vec![3]]);
        assert_eq!(factory.outputs[0][0] as usize * factory.outputs[1][0] as usize * factory.outputs[2][0] as usize, 7847);
    } 
}
