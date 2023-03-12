pub fn run(input: &str) -> usize {
    let components: Vec<_> = input.split(' ').collect();
    assert_eq!(components.len(), 19);
    let row = components[16][..components[16].len()-1].parse::<usize>().unwrap();
    let col = components[18][..components[18].len()-2].parse::<usize>().unwrap();
    code(get_sequence_number(row, col))
}

pub fn get_sequence_number(row: usize, col: usize) -> usize {
     if row == 1 {
        (col)*(col+1)/2    
    } else {
        // there is probably some way to calculate this directly, but I'm to tired to find it..
        get_sequence_number(row-1, col+1) - 1
    }
}

fn code(sequence_number: usize) -> usize {
    let mut res = 20151125;
    for _ in 1..sequence_number {
        res = (res * 252533) % 33554393;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {}", name)[..])
    }

    #[test]
    fn find_id() {
        let expected = vec![
                vec![1, 3, 6, 10, 15, 21],
                vec![2, 5, 9, 14, 20],
                vec![4, 8, 13, 19],
                vec![7, 12, 18],
                vec![11, 17],
                vec![16]
            ];
        for row in 0..expected.len() {
            for col in 0..expected[row].len() {
                assert_eq!(get_sequence_number(row+1, col+1), expected[row][col]);
            }
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), 9132360);
    }
}
