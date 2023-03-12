use std::collections::HashSet;
use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn item_priority(item: char) -> u32 {
    match item {
        lc if ('a'..='z').contains(&lc) => lc as u32 - 96,
        uc if ('A'..='Z').contains(&uc) => uc as u32 - 38,
        _ => panic!("Unexpected Token"),
    }
}

fn duplicate_prio(line: &str) -> u32 {
    if line.len() % 2 != 0 {
        panic!("Odd number of items!");
    }
    let comp1 = &line[..line.len()/2].chars().collect::<HashSet<char>>();
    let comp2 = &line[line.len()/2..].chars().collect::<HashSet<char>>();

    comp1.iter()
        .filter(|c| comp2.contains(*c))
        .map(|c| item_priority(*c))
        .sum()
}

fn badge_prio(e1: &str, e2: &str, e3: &str) -> u32 {
    e1.chars()
        .filter(|c| e2.contains(*c) && e3.contains(*c))
        .map(item_priority)
        .max()
        .unwrap()
}

fn get_badge_prios(list: &str) -> u32 {
    let mut badge_prios = 0;
    let mut iter = list.lines();
    while let (Some(e1), Some(e2), Some(e3)) = (iter.next(), iter.next(), iter.next()) { 
        badge_prios += badge_prio(e1, e2, e3);
    }
    badge_prios
}

fn main() {
    let contents = read_file("input");

    let duplicate_prios: u32 = contents.lines().map(duplicate_prio).sum();
    let badge_prios = get_badge_prios(&contents);
    println!("Priorities of Duplicates: {duplicate_prios}");
    println!("Badge Priorities: {badge_prios}");
}

#[test]
fn sample_input() {
    let contents = read_file("tests/sample_input");
    assert_eq!(contents.lines().map(duplicate_prio).sum::<u32>(), 157);
    assert_eq!(get_badge_prios(&contents), 70);
}

#[test]
fn challenge_input() {
    let contents = read_file("tests/input");
    assert_eq!(contents.lines().map(duplicate_prio).sum::<u32>(), 7746);
    assert_eq!(get_badge_prios(&contents), 2604);
}
