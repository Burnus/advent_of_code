use std::fs;

fn parse_cal_list(list: String) -> Vec<u32> {
    let mut cals: Vec<u32> = list.split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|individual_list| individual_list.lines()
             .map(|n| n.parse::<u32>().unwrap_or(0))
             .sum::<u32>())
        .collect();

    cals.sort();
    cals.reverse();
    cals
}

fn main() {
    let list = read_file("input");

    let elves = parse_cal_list(list);

    println!("Max: {}", elves[0]);
    println!("Top 3: {}", elves.iter().take(3).sum::<u32>())
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

#[test]
fn sample_input() {
    let list = read_file("tests/sample_input");
    let elves = parse_cal_list(list);

    assert_eq!(elves[0], 24000);
    assert_eq!(elves.iter().take(3).sum::<u32>(), 45000);
}

#[test]
fn challenge_input() {
    let list = read_file("tests/input");
    let elves = parse_cal_list(list);

    assert_eq!(elves[0], 71780);
    assert_eq!(elves.iter().take(3).sum::<u32>(), 212489);
}
