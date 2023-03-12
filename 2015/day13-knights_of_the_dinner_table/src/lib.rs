use std::collections::HashMap;

pub fn run(input: &str) -> (isize, isize) {
    let mut happiness_table = get_happiness_table(input);
    let first = get_max_happiness(&happiness_table);
    append_ambivalent(&mut happiness_table);
    let second = get_max_happiness(&happiness_table);
    (first, second)
}

fn append_ambivalent(table: &mut HashMap<(u16, u16), isize>) {
    let new = table.keys().map(|k| k.0).max().unwrap() * 2;
    let mut this = 1;
    while this < new {
        table.insert((this, new), 0);
        table.insert((new, this), 0);
        this *= 2;
    }
}

fn get_max_happiness(table: &HashMap<(u16, u16), isize>) -> isize {
    // since the table is round, the first placement ist arbitrary. Place attendee 1 there, since
    // they are guaranteed to exist.
    let current = 1;
    try_all(table, current, current)
}

fn try_all(table: &HashMap<(u16, u16), isize>, current: u16, seated: u16) -> isize {
    let to_place: Vec<_> = table.keys().filter(|(s, o)| *s == current && *o & seated == 0).map(|(_s, o)| *o).collect();
    if to_place.len() > 1 {
        to_place.iter()
            .map(|next| table.get(&(current, *next)).unwrap() +
                        table.get(&(*next, current)).unwrap() +
                        try_all(table, *next, seated | *next))
            .max()
            .unwrap()
    } else {
        let next = to_place[0];
        table.get(&(current, next)).unwrap() +
            table.get(&(next, current)).unwrap() +
            table.get(&(next, 1)).unwrap() +
            table.get(&(1, next)).unwrap()
    }
}

fn get_happiness_table(input: &str) -> HashMap<(u16, u16), isize> {
    let mut attendees = HashMap::new();
    let mut table = HashMap::new();
    for line in input.lines() {
        let components: Vec<_> = line.split(' ').collect();
        assert_eq!(components.len(), 11);

        let next = 2_u16.pow(attendees.len() as u32);
        let subject = *attendees.entry(components[0]).or_insert(next);
        let next = 2_u16.pow(attendees.len() as u32);
        let object = *attendees.entry(&components[10][..components[10].len()-1]).or_insert(next);
        let sign = match components[2] {
            "gain" => 1,
            "lose" => -1,
            _ => panic!("unexpected token {} in line {}", components[2], input),
        };
        let amount: isize = components[3].parse().unwrap();
        table.insert((subject, object), sign*amount);
    }
    table
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
        assert_eq!(run(&sample_input), (330, 286));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (618, 601));
    }
}
