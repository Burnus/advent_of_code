struct Disk {
    positions: usize,
    current: usize,
}

impl Disk {
    /*fn spin(&mut self) {
        let old = self.current;
        self.current = (old+1) % self.positions;
    }*/

    fn parse(line: &str) -> Self {
        let components: Vec<_> = line.split(' ').collect();
        assert_eq!(components.len(), 12);

        let pos = components[3];
        let cur = components[11];

        Self { 
            positions: pos.parse().unwrap(),
            current: cur[..cur.len()-1].parse().unwrap(),
        }
    }
}
pub fn run(input: &str) -> (usize, usize) {
    let mut disks: Vec<_> = input.lines().map(Disk::parse).collect();
    let first = (0..).find(|time| is_solution(&disks, *time)).unwrap();
    disks.push(Disk { positions: 11, current: 0 });
    let second = (0..).find(|time| is_solution(&disks, *time)).unwrap();
    (first, second)
}

fn is_solution(disks: &[Disk], time: usize) -> bool {
    for (idx, disk) in disks.iter().enumerate() {
        if (disk.current + time + idx + 1) % disk.positions != 0 {
            return false;
        }
    }
    true
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
        assert_eq!(run(&sample_input), (5, 85));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (203660, 2408135));
    }
}
