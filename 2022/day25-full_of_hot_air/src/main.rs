use std::fs;

fn from_snafu(snafu_num: &str) -> isize {
    let mut number = 0;

    snafu_num.chars().for_each(|c| {
        number *= 5;
        match &c {
            '0' | '1' | '2' => number += (c as u8 - b'0') as isize,
            '-' => number -= 1,
            '=' => number -= 2,
            _ => panic!("Unexpected character: {c} should not be part of a SNAFU number."),
        }
    });
    number
}

fn to_snafu(number: isize) -> String {
    let mut snafu_num = String::new();

    let mut temp = number;
    while temp != 0 {
        let digit = ( temp % 5) as u8;
        match digit {
            0 | 1 | 2 => snafu_num.push((digit + b'0') as char),
            3 => {
                    snafu_num.push('=');
                    temp += 2;
                },
            _ => {
                    snafu_num.push('-');
                    temp += 2;
                },
        }
        temp /= 5;
    }

    snafu_num.chars().rev().collect()
}

fn read_file(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("File not Found")
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let list = read_file("input");
    
    let total = list.iter()
        .map(|snafu_num| from_snafu(snafu_num))
        .sum::<isize>();

    println!("The total Fuel Usage is {total}, which is {} in SNAFU numbers.", to_snafu(total));
}

#[test]
fn sample_input() {
    let list = read_file("tests/sample_input");
    let total = list.iter().map(|s| from_snafu(s)).sum::<isize>();

    assert_eq!(total, 4890);
    assert_eq!(to_snafu(total), "2=-1=0");
}

#[test]
fn challenge_input() {
    let list = read_file("tests/input");
    let total = list.iter().map(|s| from_snafu(s)).sum::<isize>();

    assert_eq!(total, 34061028947237);
    assert_eq!(to_snafu(total), "2-0=11=-0-2-1==1=-22");
}
