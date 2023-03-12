#[derive(PartialEq, Eq, Clone, Copy)]
enum Sector { Free, Unvisited, Visited }

impl Sector {
    fn arr_from_hex(byte: u8) -> [Self; 8] {
        [
            if byte & 0b10000000 == 0 { Self::Free } else { Self::Unvisited },
            if byte & 0b01000000 == 0 { Self::Free } else { Self::Unvisited },
            if byte & 0b00100000 == 0 { Self::Free } else { Self::Unvisited },
            if byte & 0b00010000 == 0 { Self::Free } else { Self::Unvisited },
            if byte & 0b00001000 == 0 { Self::Free } else { Self::Unvisited },
            if byte & 0b00000100 == 0 { Self::Free } else { Self::Unvisited },
            if byte & 0b00000010 == 0 { Self::Free } else { Self::Unvisited },
            if byte & 0b00000001 == 0 { Self::Free } else { Self::Unvisited },
        ]
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let mut disk: Vec<Vec<Sector>> = (0..128)
         .map(|row| knot_hash(&format!("{input}-{row}"), 255)
            .into_iter()
            .flat_map(Sector::arr_from_hex)
            .collect::<Vec<Sector>>())
        .collect();
    let mut regions = 0;
    (0..128).for_each(|row| {
        (0..128).for_each(|col| {
            if disk[row][col] == Sector::Unvisited {
                regions += 1;
                mark_visited(&mut disk, (row, col));
            }
        });
    });
    let sectors = disk.into_iter().flatten().filter(|s| s == &Sector::Visited).count();
    (sectors, regions)
}

fn mark_visited(disk: &mut [Vec<Sector>], (row, col): (usize, usize)) {
    disk[row][col] = Sector::Visited;
    let mut new_last_step = Vec::from([(row, col)]);
    while !new_last_step.is_empty() {
        let mut new_this_step = Vec::new();
        for (row, col) in &new_last_step {
            for next in [
                            (row.checked_sub(1),    Some(*col)), 
                            (Some(*row+1),          Some(*col)), 
                            (Some(*row),            col.checked_sub(1)), 
                            (Some(*row),            Some(*col+1))
                        ] {
                match next {
                    (Some(y), Some(x)) if y < 128 && x < 128 => {
                        if disk[y][x] == Sector::Unvisited {
                            disk[y][x] = Sector::Visited;
                            new_this_step.push((y, x));
                        }
                    },
                    _ => (),
                }
            }
        }
        std::mem::swap(&mut new_this_step, &mut new_last_step);
    }
}

fn knot_hash(input: &str, max: u8) -> Vec<u8> {
    let mut list: Vec<u8> = (0..=max).collect();
    let mut current_position = 0;
    let mut skip_size = 0;

    let ascii_elements: Vec<u8> = input.bytes().chain([17, 31, 73, 47, 23].into_iter()).collect();
    for _ in 0..64 {
        twist(&mut list, &ascii_elements, &mut current_position, &mut skip_size);
    }
    list.chunks(16)
        .map(|chunk| chunk.iter().cloned().reduce(|acc, i| acc ^ i).unwrap())
        .collect()
}

fn twist(list: &mut [u8], elements: &[u8], current_position: &mut usize, skip_size: &mut usize) {
    let max = list.len();
    elements.iter().for_each(|length| {
        let new_sub_list: Vec<u8> = (0..*length).rev().map(|i| list[(*current_position+i as usize) % max]).collect();
        new_sub_list.iter().enumerate().for_each(|(idx, new_elem)| list[(*current_position + idx) % max] = *new_elem);
        *current_position += *length as usize + *skip_size;
        *current_position %= max;
        *skip_size += 1;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = "flqrgnkx";
        assert_eq!(run(sample_input), (8108, 1242));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = "nbysizxe";
        assert_eq!(run(challenge_input), (8216, 1139));
    }
}
