#[derive(Debug)]
struct Aunt {
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}

impl Aunt{
    fn new(line: &str) -> Self {
        let components: Vec<_> = line.split(' ').collect();
        assert_eq!(components.len(), 8);
        let (mut children, mut cats, mut samoyeds, mut pomeranians, mut akitas,
mut vizslas, mut goldfish, mut trees, mut cars, mut perfumes) = (None, None, None, None, None, None, None, None, None, None);
        for i in 0..2 {
            match components[2*i+2] {
                "children:" => children = Some(strip_last_char(components[2*i+3]).parse().unwrap()),
                "cats:" => cats = Some(strip_last_char(components[2*i+3]).parse().unwrap()),
                "samoyeds:" => samoyeds = Some(strip_last_char(components[2*i+3]).parse().unwrap()),
                "pomeranians:" => pomeranians = Some(strip_last_char(components[2*i+3]).parse().unwrap()),
                "akitas:" => akitas = Some(strip_last_char(components[2*i+3]).parse().unwrap()),
                "vizslas:" => vizslas = Some(strip_last_char(components[2*i+3]).parse().unwrap()),
                "goldfish:" => goldfish = Some(strip_last_char(components[2*i+3]).parse().unwrap()),
                "trees:" => trees = Some(strip_last_char(components[2*i+3]).parse().unwrap()),
                "cars:" => cars = Some(strip_last_char(components[2*i+3]).parse().unwrap()),
                "perfumes:" => perfumes = Some(strip_last_char(components[2*i+3]).parse().unwrap()),
                _ => panic!("Unknown component: {}", components[2*i+2]),
            }
        }
        match components[6] {
            "children:" => children = Some(components[7].parse().unwrap()),
            "cats:" => cats = Some(components[7].parse().unwrap()),
            "samoyeds:" => samoyeds = Some(components[7].parse().unwrap()),
            "pomeranians:" => pomeranians = Some(components[7].parse().unwrap()),
            "akitas:" => akitas = Some(components[7].parse().unwrap()),
            "vizslas:" => vizslas = Some(components[7].parse().unwrap()),
            "goldfish:" => goldfish = Some(components[7].parse().unwrap()),
            "trees:" => trees = Some(components[7].parse().unwrap()),
            "cars:" => cars = Some(components[7].parse().unwrap()),
            "perfumes:" => perfumes = Some(components[7].parse().unwrap()),
            _ => panic!("Unknown component: {}", components[6]),
        }

        Self { 
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars, 
            perfumes,
        }
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let detected = Aunt {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1)
    };
    let first = input.lines().map(Aunt::new).position(|a| 
                        (a.children.is_none() || a.children == detected.children) &&
                        (a.cats.is_none() || a.cats == detected.cats) &&
                        (a.samoyeds.is_none() || a.samoyeds == detected.samoyeds) &&
                        (a.pomeranians.is_none() || a.pomeranians == detected.pomeranians) &&
                        (a.akitas.is_none() || a.akitas == detected.akitas) &&
                        (a.vizslas.is_none() || a.vizslas == detected.vizslas) &&
                        (a.goldfish.is_none() || a.goldfish == detected.goldfish) &&
                        (a.trees.is_none() || a.trees == detected.trees) &&
                        (a.cars.is_none() || a.cars == detected.cars) &&
                        (a.perfumes.is_none() || a.perfumes == detected.perfumes) ).unwrap() + 1;
    let second = input.lines().map(Aunt::new).position(|a| 
                        (a.children.is_none() || a.children == detected.children) &&
                        (a.cats.is_none() || a.cats > detected.cats) &&
                        (a.samoyeds.is_none() || a.samoyeds == detected.samoyeds) &&
                        (a.pomeranians.is_none() || a.pomeranians < detected.pomeranians) &&
                        (a.akitas.is_none() || a.akitas == detected.akitas) &&
                        (a.vizslas.is_none() || a.vizslas == detected.vizslas) &&
                        (a.goldfish.is_none() || a.goldfish < detected.goldfish) &&
                        (a.trees.is_none() || a.trees > detected.trees) &&
                        (a.cars.is_none() || a.cars == detected.cars) &&
                        (a.perfumes.is_none() || a.perfumes == detected.perfumes) ).unwrap() + 1;
    (first, second)
}

fn strip_last_char(string: &str) -> &str {
    &string[..string.len()-1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (103, 405));
    }
}
