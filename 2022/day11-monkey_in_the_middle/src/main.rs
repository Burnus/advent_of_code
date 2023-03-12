use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum WorryLevelBehaviour { DevidedByThree, Constant }

enum Operator { Add, Sub, Mul, Div, Pot }

struct Operation {
    operator: Operator,
    operand: u128,
}

impl Operation {
    fn from(op_string: &str) -> Self {
        if op_string == "* old" {
            Self { operator: Operator::Pot, operand: 2 }
        } else {
            let operator = match &op_string[0..1] {
                "*" => Operator::Mul,
                "/" => Operator::Div,
                "+" => Operator::Add,
                "-" => Operator::Sub,
                _ => panic!("Unknown Operator in {op_string}"),
            };
            let operand = op_string[2..].parse().unwrap();

            Self {
                operator,
                operand,
            }
        }
    }

    fn perform(&self, on: u128) -> u128 {
        match self.operator {
            Operator::Add => on + self.operand,
            Operator::Sub => on - self.operand,
            Operator::Mul => on * self.operand,
            Operator::Div => on / self.operand,
            Operator::Pot => on.pow(self.operand as u32),
        }
    }
}

struct Monkey {
    id: usize,
    items: Vec<u128>,
    operation: Operation,
    divisibility_test: u128,
    true_target: usize,
    false_target: usize,
    inspected_items: u128,
}

impl Monkey {
    fn from(monkey_string: &str) -> Self {
        let lines = monkey_string.split('\n').collect::<Vec<&str>>();

        let id = lines[0][7..=7].parse().unwrap();
        let items = lines[1][18..].split(", ").map(|i| i.parse().unwrap()).collect();
        let operation = Operation::from(&lines[2][23..]);
        let divisibility_test = lines[3][21..].parse().unwrap();
        let true_target = lines[4][29..].parse().unwrap();
        let false_target = lines[5][30..].parse().unwrap();

        Self { 
            id,
            items,
            operation,
            divisibility_test, 
            true_target,
            false_target,
            inspected_items: 0,
        }
    }

    fn receive(&mut self, queue: &mut Vec<u128>) {
        self.items.append(queue);
        queue.clear();
    }

    fn send(&mut self, item: u128, monkey: usize, queues: &mut [Vec<u128>]) {
        queues[monkey].push(item);
        self.items.remove(0);
    }

    fn play(&mut self, queues: &mut [Vec<u128>], worry_level_behaviour: WorryLevelBehaviour, lcm: u128) {
        self.receive(&mut queues[self.id]);
        while !self.items.is_empty() {
            let mut item = self.items[0];
            self.inspected_items += 1;
            item = (self.operation).perform(item);
            if worry_level_behaviour == WorryLevelBehaviour::DevidedByThree {
                item /= 3;
            } else {
                item %= lcm;
            }
            if item % self.divisibility_test == 0 {
                self.send(item, self.true_target, queues);
            } else {
                self.send(item, self.false_target, queues);
            }
        }
    }
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not Found")
}

fn lcm(first: u128, second: u128) -> u128 {
    first * second / gcd(first, second)
}

fn gcd(first: u128, second: u128) -> u128 {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn get_inspections(monkey_list: &str, rounds: usize, behaviour: WorryLevelBehaviour) -> u128 {
    let mut monkeys = Vec::new();
    let mut queues: Vec<Vec<u128>> = Vec::new();
    for monkey_str in monkey_list.split("\n\n") {
        monkeys.push(Monkey::from(monkey_str));
        queues.push(Vec::new());
    }

    let lcm = monkeys.iter()
                    .map(|monkey| monkey.divisibility_test)
                    .reduce(lcm)
                    .unwrap();

    for _ in 0..rounds {
        for monkey in &mut monkeys {
            monkey.play(&mut queues, behaviour, lcm);
        }
    }

    let mut inspections = monkeys.iter()
        .map(|monkey| monkey.inspected_items)
        .collect::<Vec<u128>>();
    inspections.sort();
    inspections.reverse();

    inspections[0] * inspections[1]
}

fn main() {
    let monkey_list = read_file("input");

    println!("Before your worries increase, the top monkeys' inspections multiply into {}.", get_inspections(&monkey_list, 20, WorryLevelBehaviour::DevidedByThree));
    println!("After your worries increase, the top monkeys' inspections multiply into {}.", get_inspections(&monkey_list, 10_000, WorryLevelBehaviour::Constant));
}

#[test]
fn sample_input() {
    let monkey_list = read_file("tests/sample_input");

    assert_eq!(get_inspections(&monkey_list, 20, WorryLevelBehaviour::DevidedByThree), 10605);
    assert_eq!(get_inspections(&monkey_list, 10_000, WorryLevelBehaviour::Constant), 2713310158);
}

#[test]
fn challenge_input() {
    let monkey_list = read_file("tests/input");

    assert_eq!(get_inspections(&monkey_list, 20, WorryLevelBehaviour::DevidedByThree), 69918);
    assert_eq!(get_inspections(&monkey_list, 10_000, WorryLevelBehaviour::Constant), 19573408701);
}

