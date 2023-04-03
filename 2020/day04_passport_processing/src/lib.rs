pub fn run(input: &str) -> (usize, usize) {
    let documents: Vec<_> = input.split("\n\n").collect();
    let first = documents.iter().filter(|doc| is_valid_passport_or_npc(doc, true)).count();
    let second = documents.iter().filter(|doc| is_valid_passport_or_npc(doc, false)).count();
    (first, second)
}

fn is_valid_passport_or_npc(document: &str, skip_validation: bool) -> bool {
    let components: Vec<&str> = document.split_whitespace().collect();
    let required_fields = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];
    required_fields.iter().all(|field| components.iter().any(|comp| comp.starts_with(field) && (skip_validation || is_valid_field(comp))))
}

fn is_valid_field(field: &str) -> bool {
    let (key, value) = field.split_once(':').unwrap();
    let num_value = value.parse::<usize>();
    match key {
        "byr" => num_value.is_ok() && (1920..=2002).contains(&num_value.unwrap()),
        "iyr" => num_value.is_ok() && (2010..=2020).contains(&num_value.unwrap()),
        "eyr" => num_value.is_ok() && (2020..=2030).contains(&num_value.unwrap()),
        "hgt" => is_valid_height(value),
        "hcl" => is_hex(value),
        "ecl" => ["amb", "blu", "brn", "grn", "gry", "hzl", "oth"].contains(&value),
        "pid" => num_value.is_ok() && value.len() == 9,
        _ => false,
    }
}

fn is_valid_height(value: &str) -> bool {
    if let Some(v) = value.strip_suffix("cm") {
        if let Ok(h) = v.parse::<usize>() {
            return (150..=193).contains(&h);
        }
    } else if let Some(v) = value.strip_suffix("in") {
        if let Ok(h) = v.parse::<usize>() {
            return (59..=76).contains(&h);
        }
    }
    false
}

fn is_hex(value: &str) -> bool {
    value.len() == 7 && value.starts_with('#') && value.bytes().skip(1).all(|b| b.is_ascii_hexdigit())
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
        assert_eq!(run(&sample_input), (10, 6));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (237, 172));
    }
}
