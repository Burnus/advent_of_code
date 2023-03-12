use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn find_start_marker(message: &Vec<char>, distinct_character_count: usize) -> usize {
    'char_iterator: for index in distinct_character_count-1..message.len() {
        let mut found: Vec<char> = Vec::with_capacity(distinct_character_count);
        for offset in 0..distinct_character_count {
            let this_char = message[index+offset+1-distinct_character_count];
            if found.contains(&this_char) { continue 'char_iterator; }
            found.push(this_char);
        }
        return index+1;
    }
    panic!("No start found");
}

fn main() {
    //let datastream = read_file("sample_input");
    let datastream = read_file("input");

    let chars = datastream.chars().collect::<Vec<char>>();
    println!("Start of Packet: {}", find_start_marker(&chars, 4));
    println!("Start of Message: {}", find_start_marker(&chars, 14));
}

#[test]
fn sample_input() {
    let datastream = read_file("tests/sample_input");

    let chars = datastream.chars().collect::<Vec<_>>();
    assert_eq!(find_start_marker(&chars, 4), 7);
    assert_eq!(find_start_marker(&chars, 14), 19);
}

#[test]
fn challenge_input() {
    let datastream = read_file("tests/input");

    let chars = datastream.chars().collect::<Vec<_>>();
    assert_eq!(find_start_marker(&chars, 4), 1702);
    assert_eq!(find_start_marker(&chars, 14), 3559);
}
