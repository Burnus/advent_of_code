use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn get_coordinates(encrypted: &[isize], key: isize, rounds: u8) -> (isize, isize, isize) {
    let decrypted = shuffle_with_key(encrypted, key, rounds);

    let offset = decrypted.iter().position(|&x| x == 0).unwrap() as usize;
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

fn main() {
    let contents = read_file("input");
    
    let encrypted: Vec<isize> = contents.lines().map(|i| i.parse().unwrap()).collect();
    
    let (c1, c2, c3) = get_coordinates(&encrypted, 1, 1);
    println!("The relevant numbers are {}, {} and {}, totalling {}.", c1, c2, c3, c1+c2+c3);

    let (d1, d2, d3) = get_coordinates(&encrypted, 811589153, 10);
    println!("With Key, the relevant numbers are {}, {} and {}, totalling {}.", d1, d2, d3, d1+d2+d3);
}

#[test]
fn sample_input() {
    let contents = read_file("tests/sample_input");
    let encrypted: Vec<isize> = contents.lines().map(|i| i.parse().unwrap()).collect();

    let (c1, c2, c3) = get_coordinates(&encrypted, 1, 1);
    let (d1, d2, d3) = get_coordinates(&encrypted, 811589153, 10);

    assert_eq!((c1, c2, c3), (4, -3, 2));
    assert_eq!((d1, d2, d3), (811589153, 2434767459, -1623178306));
}

#[test]
fn challenge_input() {
    let contents = read_file("tests/input");
    let encrypted: Vec<isize> = contents.lines().map(|i| i.parse().unwrap()).collect();

    let (c1, c2, c3) = get_coordinates(&encrypted, 1, 1);
    let (d1, d2, d3) = get_coordinates(&encrypted, 811589153, 10);

    assert_eq!((c1, c2, c3), (6790, 9749, -8511));
    assert_eq!((d1, d2, d3), (6447264231432, 3708150840057, -1356977063816));
}
