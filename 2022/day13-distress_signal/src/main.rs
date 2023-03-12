use std::{fs, cmp::Ordering};

#[derive(Debug, Clone, PartialEq)]
enum PacketItem<T> { 
    Empty,
    Number(T),
    List(Vec<PacketItem<T>>),
}

#[derive(Debug)]
struct Pair {
    left: PacketItem<usize>,
    right: PacketItem<usize>,
}

impl Pair {
    fn from(read_string: &str) -> Self {
        let (first, second) = read_string.split_once('\n').expect("Failed to split input pair.");

        let left = parse_packet_item(first);
        let right = parse_packet_item(second);

        Self {
            left,
            right,
        }
    }

}

fn are_correctly_ordered(left: &PacketItem<usize>, right: &PacketItem<usize>) -> Option<bool> {
    // if both values are integer, the lower integer should come first
    if let PacketItem::Number(l) = left {
        if let PacketItem::Number(r) = right {
            return match l.cmp(r) {
                Ordering::Less => Some(true),
                Ordering::Greater => Some(false),
                Ordering::Equal => None,
            };
        }
    }
    // if both values are lists, compare element-wise
    if let PacketItem::List(l) = left {
        if let PacketItem::List(r) = right {
            for index in 0..l.len() {
                // if the right list runs out of items first, the inputs are not in the right order.
                if index >= r.len() {
                    return Some(false);
                }
                if let Some(result) = are_correctly_ordered(&l[index], &r[index]) {
                    return Some(result);
                }
            }
            if l.len() < r.len() {
                return Some(true);
            }
            return None;
        }
    }

    // If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison.
    if let PacketItem::Number(_) = left {
        if let PacketItem::List(_) = right {
            return are_correctly_ordered(&PacketItem::List(vec![left.clone()]), right);
        }
    }
    if let PacketItem::List(_) = left {
        if let PacketItem::Number(_) = right {
            return are_correctly_ordered(left, &PacketItem::List(vec![right.clone()]));
        }
    }

    if let PacketItem::Empty = left {
        if let PacketItem::Empty = right {
            return None;
        }
        return Some(true);
    }
    if let PacketItem::Empty = right {
        return Some(false);
    }
    None
}

fn parse_packet_item(string_representation: &str) -> PacketItem<usize> {
    if string_representation.is_empty() {
        return PacketItem::Empty;
    }
    if let Ok(int) = string_representation.trim().parse::<usize>() {
        return PacketItem::Number(int);
    }
    let mut sub_items = Vec::new();
    let mut nesting_level = 0;
    let mut this_item = String::new();
    for char in string_representation.chars() {
        match char {
            '[' => { 
                    nesting_level += 1; 
                    if nesting_level > 1 {
                        this_item += "[";
                    }
                },
            ']' => { 
                    nesting_level -= 1; 
                    if nesting_level > 0 {
                        this_item += "]"; 
                    }
                },
            ',' if nesting_level == 1 => { 
                sub_items.push(parse_packet_item(&this_item));
                this_item = String::new(); },
            c => this_item += &c.to_string(),
        }
    }
    if string_representation.starts_with('[') {
        sub_items.push(parse_packet_item(&this_item));
    }
    PacketItem::List(sub_items)
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn get_pairs(received: &str) -> Vec<Pair> {
    received.split("\n\n")
        .map(Pair::from)
        .collect()
}

fn get_pair_sum(pairs: &[Pair]) -> usize {
    pairs.iter()
        .enumerate()
        .filter(|(_, pair)| are_correctly_ordered(&pair.left, &pair.right) == Some(true))
        .map(|(index, _)| index+1)
        .sum::<usize>()
}

fn decode(pairs: &[Pair]) -> usize {
    let divider1 = &PacketItem::List(vec![PacketItem::List(vec![PacketItem::Number(2)])]);
    let divider2 = &PacketItem::List(vec![PacketItem::List(vec![PacketItem::Number(6)])]);
    let all_packets = &mut pairs.iter()
        .map(|pair| pair.left.clone())
        .collect::<Vec<PacketItem<usize>>>();
    all_packets.append(&mut pairs.iter()
                .map(|pair| pair.right.clone())
                .collect::<Vec<PacketItem<usize>>>());
    all_packets.append(&mut vec![divider1.clone(), divider2.clone()]);
    all_packets.sort_by(|a, b| match are_correctly_ordered(a, b) {
            Some(true) => std::cmp::Ordering::Less,
            Some(false) => std::cmp::Ordering::Greater,
            None => panic!("Unable to compare {:?} and {:?}", a, b),
        });
    
    all_packets.iter()
        .enumerate()
        .filter(|(_, packet)| *packet == divider1 || *packet == divider2)
        .map(|(index, _)| index + 1)
        .reduce(|a, b| a*b)
        .unwrap()
}

fn main() {
    let received = read_file("input");

    let pairs = get_pairs(&received);

    println!("The sum of the indexes of correctly ordered pairs is {}", get_pair_sum(&pairs));
    println!("The decoder key is {}", decode(&pairs));
}

#[test]
fn sample_input() {
    let received = read_file("tests/sample_input");
    let pairs = get_pairs(&received);

    assert_eq!(get_pair_sum(&pairs), 13);
    assert_eq!(decode(&pairs), 140);
}

#[test]
fn challenge_input() {
    let received = read_file("tests/input");
    let pairs = get_pairs(&received);

    assert_eq!(get_pair_sum(&pairs), 5659);
    assert_eq!(decode(&pairs), 22110);
}
