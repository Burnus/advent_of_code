use core::fmt::Display;
use std::num::ParseIntError;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    LineMalformed(&'a str),
    MissingRoot,
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
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::MissingRoot => write!(f, "Input didn't contain a root monkey"),
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
        }
    }
}


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

    fn perform(&self, monkeys: &HashMap<String, Monkey>) -> f64 {
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
    number: Option<f64>,
    operation: Operation,
}

impl Monkey {
    fn get_number(&self, monkeys: &HashMap<String, Monkey>) -> f64 {
        if let Some(number) = self.number {
            number
        } else {
            self.operation.perform(monkeys)
        }
    }
}

fn try_parse(input: &str) -> Result<HashMap<String, Monkey>, ParseError> {
    let mut monkeys = HashMap::new();
    for line in input.lines() {
            let components = line.split(' ').collect::<Vec<&str>>();
            let name = components[0][..components[0].len()-1].to_string();
            let (number, operation) = match components.len() {
                2 => (Some(components[1].parse().unwrap()), Operation { left: name.to_string(), operator: Operator::Add, right: "none".to_string() }),
                4 => (None, Operation::from(components[1].to_string(), components[2].to_string(), components[3].to_string())),
                _ => return Err(ParseError::LineMalformed(line)),
            };
            monkeys.insert(name, Monkey { number, operation });
        }
    Ok(monkeys)
}

fn guess_number(left: Monkey, rigth: Monkey, last_guess: f64, monkeys: &mut HashMap<String, Monkey>) -> isize {
    monkeys.insert("humn".to_string(), Monkey { number: Some(last_guess), operation: Operation { left: "none".to_string(), operator: Operator::Add, right: "none".to_string() } });
    let diff0 = rigth.get_number(monkeys) - left.get_number(monkeys);

    if diff0 == 0.0 {
        return last_guess as isize;
    }
    monkeys.insert("humn".to_string(), Monkey { number: Some(last_guess+1.0), operation: Operation { left: "none".to_string(), operator: Operator::Add, right: "none".to_string() } });
    let diff1 = rigth.get_number(monkeys) - left.get_number(monkeys);
    
    if diff1 == 0.0 {
        return last_guess as isize + 1;
    }

    if diff0 == diff1 {
        return  guess_number(left, rigth, last_guess-1.0, monkeys);
    }

    let mut next_guess = (last_guess + diff0/(diff0-diff1)).round();
    if next_guess == last_guess {
        next_guess -= 1.0;
    }
    guess_number(left, rigth, next_guess, monkeys)
}

pub fn run(input: &str) -> Result<(isize, isize), ParseError> {
    let mut monkeys = try_parse(input)?;
    let root = monkeys.get(&"root".to_string()).ok_or(ParseError::MissingRoot)?;
    let rl = monkeys.get(&root.operation.left).unwrap().clone();
    let rr = monkeys.get(&root.operation.right).unwrap().clone();

    let first = root.get_number(&monkeys) as isize;
    let second = guess_number(rl, rr, 0.0, &mut monkeys);
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
        assert_eq!(run(&sample_input), Ok((152, 301)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((158661812617812, 3352886133831)));
    }
}
