use std::num::ParseIntError;

fn get_coordinates(encrypted: &[isize], key: isize, rounds: u8) -> (isize, isize, isize) {
    let decrypted = shuffle_with_key(encrypted, key, rounds);

    let offset = decrypted.iter().position(|&x| x == 0).unwrap();
    let c1 = decrypted[(1000 + offset) % decrypted.len()];
    let c2 = decrypted[(2000 + offset) % decrypted.len()];
    let c3 = decrypted[(3000 + offset) % decrypted.len()];
    (c1, c2, c3)
}

fn shuffle_with_key(old: &[isize], key: isize, rounds: u8) -> Vec<isize> {
    let old_list: Vec<(usize, isize)> = old.iter().enumerate().map(|(idx, &i)| (idx, key * i)).collect();
    let mut new_list = old_list.clone();

    for _ in 0..rounds {
        old_list.iter().for_each(|(old_index, number)| {
            let old_index_new_list = new_list.iter()
                .position(|(index, _)| index == old_index)
                .unwrap();

            let mut new_index = old_index_new_list as isize + number;
            if new_index <= 0 {
                new_index = new_list.len() as isize - (new_index.abs() % (new_list.len() as isize - 1)) - 1;
            }
            if new_index >= new_list.len() as isize {
                new_index %= new_list.len() as isize - 1;
            }

            new_list.remove(old_index_new_list);
            new_list.insert(new_index as usize, (*old_index, *number));
        });
    }

    new_list.iter()
        .map(|(_, n)| *n)
        .collect()
}

pub fn run(input: &str) -> Result<(isize, isize), ParseIntError> {
    let encrypted: Vec<isize> = input.lines().map(|i| i.parse()).collect::<Result<Vec<_>, _>>()?;
    let (c1, c2, c3) = get_coordinates(&encrypted, 1, 1);
    let (d1, d2, d3) = get_coordinates(&encrypted, 811589153, 10);
    let first = c1+c2+c3;
    let second = d1+d2+d3;
    Ok((first, second))
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
        assert_eq!(run(&sample_input), Ok((3, 1623178306)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((8028, 8798438007673)));
    }
}
