use std::{fs, collections::HashMap};

#[derive(Clone)]
enum Operator { Add, Sub, Mul, Div }

#[derive(Clone)]
struct Operation {
    left: String,
    operator: Operator,
    right: String,
} 

impl Operation {
    fn from(left: String, operator: String, right: String) -> Self {
        Self {
            left,
            operator: match &operator[..] {
                    "+" => Operator::Add,
                    "-" => Operator::Sub,
                    "*" => Operator::Mul,
                    "/" => Operator::Div,
                    _ => panic!("Unknown Operator: {operator}"),
                },
            right,
        }
    }

    fn perform(&self, monkeys: &HashMap<String, Monkey>) -> isize {
        let left = monkeys.get(&self.left).unwrap().get_number(monkeys);
        let right = monkeys.get(&self.right).unwrap().get_number(monkeys);
        match self.operator {
            Operator::Add => left + right,
            Operator::Sub => left - right,
            Operator::Mul => left * right,
            Operator::Div => left / right,
        }
    }
}

#[derive(Clone)]
struct Monkey {
    number: Option<isize>,
    operation: Operation,
}

impl Monkey {
    fn get_number(&self, monkeys: &HashMap<String, Monkey>) -> isize {
        if let Some(number) = self.number {
            number
        } else {
            self.operation.perform(monkeys)
        }
    }
}

fn read_file(path: &str) -> HashMap<String, Monkey> {
    let mut monkeys = HashMap::new();
    fs::read_to_string(path)
        .expect("File not Found")
        .lines()
        .for_each(|line| {
            let components = line.split(' ').collect::<Vec<&str>>();
            let name = components[0][..components[0].len()-1].to_string();
            let (number, operation) = match components.len() {
                2 => (Some(components[1].parse().unwrap()), Operation { left: name.to_string(), operator: Operator::Add, right: "none".to_string() }),
                4 => (None, Operation::from(components[1].to_string(), components[2].to_string(), components[3].to_string())),
                _ => panic!("Unexpected number of components in {line}"),
            };
            monkeys.insert(name, Monkey { number, operation });
        });
    monkeys
}

fn guess_number(left: Monkey, rigth: Monkey, last_guess: isize, monkeys: &mut HashMap<String, Monkey>) -> isize {
    monkeys.insert("humn".to_string(), Monkey { number: Some(last_guess), operation: Operation { left: "none".to_string(), operator: Operator::Add, right: "none".to_string() } });
    let diff0 = rigth.get_number(monkeys) - left.get_number(monkeys);

    if diff0 == 0 {
        return last_guess;
    }
    monkeys.insert("humn".to_string(), Monkey { number: Some(last_guess+1), operation: Operation { left: "none".to_string(), operator: Operator::Add, right: "none".to_string() } });
    let diff1 = rigth.get_number(monkeys) - left.get_number(monkeys);
    
    if diff1 == 0 {
        return last_guess+1;
    }

    if diff0 == diff1 {
        return  guess_number(left, rigth, last_guess-1, monkeys);
    }

    let mut next_guess = last_guess + diff0/(diff0-diff1);
    if next_guess == last_guess {
        next_guess -= 1;
    }
    guess_number(left, rigth, next_guess, monkeys)
}

fn main() {
    let mut monkeys = read_file("input");

    let root = monkeys.get(&"root".to_string()).unwrap();
    println!("The root number is {}", root.get_number(&monkeys));

    let rl = monkeys.get(&root.operation.left).unwrap().clone() ;
    let rr = monkeys.get(&root.operation.right).unwrap().clone();

    println!("You should yell {}", guess_number(rl, rr, 0, &mut monkeys));
}

#[test]
fn sample_input() {
    let mut monkeys = read_file("tests/sample_input");
    let root = monkeys.get(&"root".to_string()).unwrap();

    assert_eq!(root.get_number(&monkeys), 152);
    assert_eq!(guess_number(monkeys.get(&root.operation.left).unwrap().clone(), monkeys.get(&root.operation.right).unwrap().clone(), 0, &mut monkeys), 301);
}

#[test]
fn challenge_input() {
    let mut monkeys = read_file("tests/input");
    let root = monkeys.get(&"root".to_string()).unwrap();

    assert_eq!(root.get_number(&monkeys), 158661812617812);
    // There are actually multiple solutions for my input. I orginally found 3352886133831 (the smallest of
    // them). This algorithm finds 3352886133834 (the largest) if seeded with 0. 3352886133832 is also valid
    assert!((3352886133831..=3352886133834).contains(&guess_number(monkeys.get(&root.operation.left).unwrap().clone(), monkeys.get(&root.operation.right).unwrap().clone(), 0, &mut monkeys)));
}
