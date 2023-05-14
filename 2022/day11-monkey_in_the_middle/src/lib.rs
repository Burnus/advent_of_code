use core::fmt::Display;
use std::{num::ParseIntError, collections::VecDeque};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    MalformedInput(&'a str),
    ParseIntError(std::num::ParseIntError),
}

impl From<ParseIntError> for ParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MalformedInput(v) => write!(f, "Monkey is malformed: {v}"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum WorryLevelBehaviour { DevidedByThree, Constant }

#[derive(Clone)]
enum Operator { Add, Sub, Mul, Div, Pot }

#[derive(Clone)]
struct Operation {
    operator: Operator,
    operand: usize,
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

    fn perform(&self, on: usize) -> usize {
        match self.operator {
            Operator::Add => on + self.operand,
            Operator::Sub => on - self.operand,
            Operator::Mul => on * self.operand,
            Operator::Div => on / self.operand,
            Operator::Pot => on.pow(self.operand as u32),
        }
    }
}

#[derive(Clone)]
struct Monkey {
    id: usize,
    items: VecDeque<usize>,
    operation: Operation,
    divisibility_test: usize,
    true_target: usize,
    false_target: usize,
    inspected_items: usize,
}

impl <'a> TryFrom<&'a str> for Monkey {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let lines = value.lines().collect::<Vec<&str>>();
        if lines.len() != 6 ||
            lines[0].len() < 7 ||
            lines[1].len() < 18 ||
            lines[2].len() < 23 ||
            lines[3].len() < 21 ||
            lines[4].len() < 29 ||
            lines[5].len() < 30
        {
            return Err(Self::Error::MalformedInput(value));
        }

        let id = lines[0][7..=7].parse()?;
        let items = lines[1][18..].split(", ").map(|i| i.parse()).collect::<Result<VecDeque<_>, _>>()?;
        let operation = Operation::from(&lines[2][23..]);
        let divisibility_test = lines[3][21..].parse()?;
        let true_target = lines[4][29..].parse()?;
        let false_target = lines[5][30..].parse()?;

        Ok(Self { 
            id,
            items,
            operation,
            divisibility_test, 
            true_target,
            false_target,
            inspected_items: 0,
        })
    }
}

impl Monkey {
    fn play(&mut self, queues: &mut [VecDeque<usize>], worry_level_behaviour: WorryLevelBehaviour, lcm: usize) {
        self.items.append(&mut queues[self.id]);
        while let Some(mut item) = self.items.pop_front() {
            self.inspected_items += 1;
            item = (self.operation).perform(item);
            if worry_level_behaviour == WorryLevelBehaviour::DevidedByThree {
                item /= 3;
            } else {
                item %= lcm;
            }
            if item % self.divisibility_test == 0 {
                queues[self.true_target].push_back(item);
            } else {
                queues[self.false_target].push_back(item);
            }
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let mut monkeys_1 = input.split("\n\n").map(Monkey::try_from).collect::<Result<Vec<_>, _>>()?;
    let mut monkeys_2 = monkeys_1.to_vec();
    let lcm = monkeys_1.iter().map(|monkey| monkey.divisibility_test).reduce(lcm).expect("Unable to find an lcm");
    let first = get_inspections(&mut monkeys_1, lcm, 20, WorryLevelBehaviour::DevidedByThree);
    let second = get_inspections(&mut monkeys_2, lcm, 10_000, WorryLevelBehaviour::Constant);
    Ok((first, second))
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
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

fn get_inspections(monkeys: &mut [Monkey], lcm: usize, rounds: usize, behaviour: WorryLevelBehaviour) -> usize {
    let mut queues: Vec<VecDeque<usize>> = vec![VecDeque::new(); monkeys.len()];

    for _ in 0..rounds {
        for monkey in monkeys.iter_mut() {
            monkey.play(&mut queues, behaviour, lcm);
        }
    }

    let mut inspections = monkeys.iter()
        .map(|monkey| monkey.inspected_items)
        .collect::<Vec<usize>>();
    inspections.sort_by_key(|i| std::cmp::Reverse(*i));

    inspections[0] * inspections[1]
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
        assert_eq!(run(&sample_input), Ok((10605, 2713310158)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((69918, 19573408701)));
    }
}
