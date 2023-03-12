pub fn run(input: &str, length: usize) -> (usize, String) {
    let mut list: Vec<usize> = (0..length).collect();
    let elements: Vec<usize> = input.split(',').map(|i| i.parse::<usize>().unwrap()).collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    twist(&mut list, &elements, &mut current_position, &mut skip_size);
    let first = list[0] * list[1];

    list = (0..length).collect();
    let mut ascii_elements: Vec<usize> = input.bytes().map(|i| i as usize).collect();
    ascii_elements.append(&mut vec![17, 31, 73, 47, 23]);
    current_position = 0;
    skip_size = 0;
    for _ in 0..64 {
        twist(&mut list, &ascii_elements, &mut current_position, &mut skip_size);
    }
    let dense_hash: Vec<usize> = list.chunks(16)
        .map(|chunk| chunk.iter().cloned().reduce(|acc, i| acc ^ i).unwrap())
        .collect::<Vec<usize>>();

    let mut second = String::new();
    for chunk in dense_hash {
        second += &format!("{:02x}", chunk);
    }
    (first, second)
}

fn twist(list: &mut [usize], elements: &[usize], current_position: &mut usize, skip_size: &mut usize) {
    let max = list.len();
    elements.iter().for_each(|length| {
        let new_sub_list: Vec<usize> = (0..*length).rev().map(|i| list[(*current_position+i) % max]).collect();
        new_sub_list.iter().enumerate().for_each(|(idx, new_elem)| list[(*current_position+idx) % max] = *new_elem);
        *current_position += length + *skip_size;
        *current_position %= max;
        *skip_size += 1;
    });
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
        assert_eq!(run(&sample_input, 5), (12, "04".to_string()));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input, 256), (6909, "9d5f4561367d379cfbf04f8c471c0095".to_string()));
    }
}
