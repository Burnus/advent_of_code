use std::collections::BTreeSet;

#[derive(Clone)]
struct Particle {
    p: [isize; 3],
    v: [isize; 3],
    a: [isize; 3],
}

impl Particle {
    fn parse(line: &str) -> Self {
        let components: Vec<_> = line.split(", ").collect();
        assert_eq!(components.len(), 3);

        let vectors: Vec<_> = components.iter().map(|comp| {
            let coords: Vec<_> = comp.split(',').collect();
            [
                coords[0][3..].parse().unwrap(),
                coords[1].parse().unwrap(),
                coords[2][..coords[2].len()-1].parse().unwrap(),
            ]
        }).collect();

        Self { 
            p: vectors[0], 
            v: vectors[1],
            a: vectors[2],
        }
    }
    
    fn update(&mut self) {
        for idx in 0..3 {
            self.v[idx] += self.a[idx];
            self.p[idx] += self.v[idx];
        }
    }

    fn distance_from_origin(&self) -> usize {
        self.p.iter().map(|i| i.unsigned_abs()).sum()
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let mut particles: Vec<_> = input.lines().map(Particle::parse).collect();
    let mut particles_2 = particles.clone();
    for _ in 0..500 {
        for particle in particles.iter_mut() {
            particle.update();
        }
    }
    for _ in 0..500 {
        for particle in particles_2.iter_mut() {
            particle.update();
        }
        let mut to_remove = BTreeSet::new();
        for (idx, particle) in particles_2.iter().enumerate() {
            let new: Vec<_> = particles_2.iter().enumerate().skip(idx+1).filter(|(_idx, other)| other.p == particle.p).map(|(idx, _p)| idx).collect();
            if !new.is_empty() {
                to_remove.insert(idx);
                for this in new {
                    to_remove.insert(this);
                }
            }
        }
        for i in to_remove.iter().rev() {
            particles_2.remove(*i);
        }
    }
    let first = particles.iter().enumerate().min_by_key(|(_idx, p)| p.distance_from_origin()).map(|(idx, _p)| idx).unwrap();
    let second = particles_2.len();
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
        assert_eq!(run(&sample_input), (3, 1));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (91, 567));
    }
}
