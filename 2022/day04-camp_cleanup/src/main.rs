use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn fully_contained(pair: &str) -> bool {
    let ((l1, r1), (l2, r2)) = parse_into_tuples(pair);
    l1<=l2 && r1>=r2 || l2<=l1 && r2>=r1
}

fn overlapping(pair: &str) -> bool {
    let ((l1, r1), (l2, r2)) = parse_into_tuples(pair);
    l1<=l2 && r1>=l2 || l2<=l1 && r2>=l1
}

fn parse_into_tuples(pair: &str) -> ((u32, u32), (u32, u32)) {
    if let Some((first, second)) = pair.split_once(',') {
        if let Some ((l1, r1)) = first.split_once('-') {
            if let Some((l2, r2)) = second.split_once('-') {
                let l1 = l1.parse::<u32>().expect("Malformed ID: Not a number");
                let l2 = l2.parse::<u32>().expect("Malformed ID: Not a number");
                let r1 = r1.parse::<u32>().expect("Malformed ID: Not a number");
                let r2 = r2.parse::<u32>().expect("Malformed ID: Not a number");
                return ((l1, r1), (l2, r2));
            }
        }
    }
    panic!("Malformatted input");
}

fn main() {
    //let contents = read_file("sample_input");
    let contents = read_file("input");

    let fully_contained_count = contents.lines()
        .filter(|l| fully_contained(l))
        .count();

    let overlapping_count = contents.lines()
        .filter(|l| overlapping(l))
        .count();

    println!("For {fully_contained_count} pairs, one is fully contained within the other.");
    println!("{overlapping_count} pairs have at least some overlap.")
}

#[test]
fn sample_input() {
    let contents = read_file("tests/sample_input");
    assert_eq!(contents.lines().filter(|l| fully_contained(l)).count(), 2);
    assert_eq!(contents.lines().filter(|l| overlapping(l)).count(), 4);
}

#[test]
fn challenge_input() {
    let contents = read_file("tests/input");
    assert_eq!(contents.lines().filter(|l| fully_contained(l)).count(), 560);
    assert_eq!(contents.lines().filter(|l| overlapping(l)).count(), 839);
}
