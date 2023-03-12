use std::collections::HashMap;

pub fn run(input: &str) -> (u32, u32) {
    let real_rooms: Vec<_> = input.lines().map(parse_room_list).filter(|(room, _seq, checksum)| is_real(room, checksum)).collect();
    let first = real_rooms.iter().map(|(_, seq, _)| seq).sum();
    let second = real_rooms.iter().find(|(room, seq, _)| decrypt(room, *seq) == "northpole object storage").map(|(_, seq, _)| *seq).unwrap_or(0); 
    (first, second)
}

fn decrypt(room: &str, seq: u32) -> String {
    room.chars().map(|c| {
        // convert dashes to spaces and rotate letters by seq (wrapping around). Magic number 97 is
        // the numeric value of lowercase a.
            match c {
                '-' => ' ',
                l => char::from_u32((l as u32 + seq - 97) % 26 + 97 ).unwrap(),
            }
        }).collect()
}

fn is_real(room: &str, checksum: &str) -> bool {
    let mut char_map = HashMap::new();
    room.chars().for_each(|c| {
        if c != '-' {
            char_map.entry(c).and_modify(|v| *v+=1).or_insert(0);
        }
    });
    let mut sorted: Vec<_> = char_map.iter().collect();
    sorted.sort_by(|a, b| {
            match b.1.cmp(a.1) {
                std::cmp::Ordering::Equal => a.0.cmp(b.0),
                _ => b.1.cmp(a.1),
            }
        });
    for (idx, char) in checksum.chars().enumerate() {
        if *sorted[idx].0 != char {
            return false;
        }
    }
    true
}

fn parse_room_list(line: &str) -> (&str, u32, &str) {
    let (room, rest) = line.rsplit_once('-').unwrap();
    let (seq, checksum) = rest.split_once('[').unwrap();
    ( room, seq.parse().unwrap(), &checksum[..checksum.len()-1] )
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
        assert_eq!(run(&sample_input), (1857, 0));
        assert_eq!(decrypt("qzmt-zixmtkozy-ivhz", 343), "very encrypted name".to_string());
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), (245102, 324));
    }
}
