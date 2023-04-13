use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ParseIntError(std::num::ParseIntError),
    LineMalformed(String),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseIntError(e) => write!(f, "Unable to parse into integer: {e}"),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

enum Operator {
    Add,
    Mul,
}

enum SubExpression {
    Expr(Box<Expression>),
    Num(usize),
}

impl SubExpression {
    fn eval(&self) -> usize {
        match self {
            Self::Expr(e) => e.eval(),
            Self::Num(n) => *n,
        }
    }
}

struct Expression {
    op: Operator,
    lhs: SubExpression,
    rhs: SubExpression,
}

impl TryFrom<&str> for Expression {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let components: Vec<_> = value.split_whitespace().collect();
        let last = components.len()-1;
        if last < 2 {
            Err(Self::Error::LineMalformed(value.to_string()))
        } else {
            if components[last].ends_with(')') {
                let mut recursion_level = components[last].matches(')').count();
                let mut rhs_components = 0;
                while recursion_level > 0 {
                    rhs_components += 1;
                    recursion_level += components[last-rhs_components].matches(')').count();
                    recursion_level -= components[last-rhs_components].matches('(').count();
                }
                if rhs_components == last {
                    Expression::try_from(&value[1..value.len()-1])
                } else {
                    let rhs_str = &components[last-rhs_components..].join(" ")[..];
                    let rhs = SubExpression::Expr(Box::new(Expression::try_from(&rhs_str[1..rhs_str.len()-1])?));
                let op = match components[last-rhs_components-1] {
                    "+" => Operator::Add,
                    "*" => Operator::Mul,
                    _ => return Err(Self::Error::LineMalformed(value.to_string())),
                };
                let lhs = match last-rhs_components {
                    2 => SubExpression::Num(components[0].parse()?),
                    _ => SubExpression::Expr(Box::new(Expression::try_from(&components[..last-rhs_components-1].join(" ")[..])?)),
                };
                Ok(Self { op, lhs, rhs })
                }
            } else {
                let rhs = SubExpression::Num(components[last].parse()?);
                let op = match components[last-1] {
                    "+" => Operator::Add,
                    "*" => Operator::Mul,
                    _ => return Err(Self::Error::LineMalformed(value.to_string())),
                };
                let lhs = match last {
                    2 => SubExpression::Num(components[0].parse()?),
                    _ => SubExpression::Expr(Box::new(Expression::try_from(&components[..last-1].join(" ")[..])?)),
                };
                Ok(Self { op, lhs, rhs })
            }
        }
    }
}

impl Expression {
    fn eval(&self) -> usize {
        match self.op {
            Operator::Mul => self.lhs.eval() * self.rhs.eval(),
            Operator::Add => self.lhs.eval() + self.rhs.eval(),
        }
    }

    fn try_from_advanced(value: &str) -> Result<Self, ParseError> {
        let components: Vec<_> = value.split_whitespace().collect();
        let last = components.len()-1;
        if last < 2 {
            Err(ParseError::LineMalformed(value.to_string()))
        } else {
            if components[last].ends_with(')') {
                let mut recursion_level = components[last].matches(')').count();
                let mut rhs_components = 0;
                while recursion_level > 0 {
                    rhs_components += 1;
                    recursion_level += components[last-rhs_components].matches(')').count();
                    recursion_level -= components[last-rhs_components].matches('(').count();
                }
                if rhs_components == last {
                    Expression::try_from_advanced(&value[1..value.len()-1])
                } else {
                    let mut recursion_level = 0;
                    for idx in (0..last-rhs_components).rev() {
                        recursion_level += components[idx].matches(')').count();
                        recursion_level -= components[idx].matches('(').count();
                        if recursion_level == 0 && components[idx] == "*" {
                            let op = Operator::Mul;
                            let lhs = if idx == 1 {
                                SubExpression::Num(components[0].parse()?)
                            } else {
                                SubExpression::Expr(Box::new(Expression::try_from_advanced(&components[..idx].join(" ")[..])?))
                            };
                            let rhs = SubExpression::Expr(Box::new(Expression::try_from_advanced(&components[idx+1..].join(" ")[..])?));
                            return Ok(Self { op, lhs, rhs });
                        }
                    }
                    let op = Operator::Add;
                    let rhs = SubExpression::Expr(Box::new(Expression::try_from_advanced(&components[last-rhs_components..].join(" ")[..])?));
                    let lhs = match last-rhs_components {
                        2 => SubExpression::Num(components[0].parse()?),
                        _ => SubExpression::Expr(Box::new(Expression::try_from_advanced(&components[..last-rhs_components-1].join(" ")[..])?)),
                    };

                    Ok(Self { op, lhs, rhs })
                }
            } else {
                let mut recursion_level = 0;
                for idx in (0..=last).rev() {
                    recursion_level += components[idx].matches(')').count();
                    recursion_level -= components[idx].matches('(').count();
                    if recursion_level == 0 && components[idx] == "*" {
                        let op = Operator::Mul;
                        let lhs = if idx == 1 {
                            SubExpression::Num(components[0].parse()?)
                        } else {
                            SubExpression::Expr(Box::new(Expression::try_from_advanced(&components[..idx].join(" ")[..])?))
                        };
                        let rhs = if idx == last-1 {
                            SubExpression::Num(components[components.len()-1].parse()?)
                        } else {
                            SubExpression::Expr(Box::new(Expression::try_from_advanced(&components[idx+1..].join(" ")[..])?))
                        };
                        return Ok(Self { op, lhs, rhs });
                    }
                }
                let op = Operator::Add;
                let rhs = SubExpression::Num(components[last].parse()?);
                let lhs = match last {
                    2 => SubExpression::Num(components[0].parse()?),
                    _ => SubExpression::Expr(Box::new(Expression::try_from_advanced(&components[..last-1].join(" ")[..])?)),
                };

                Ok(Self { op, lhs, rhs })
            }
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let formulas_1: Vec<_> = input.lines().map(Expression::try_from).collect::<Result<Vec<_>, _>>()?;
    let formulas_2: Vec<_> = input.lines().map(Expression::try_from_advanced).collect::<Result<Vec<_>, _>>()?;
    let first = formulas_1.iter().map(|f| f.eval()).sum();
    let second = formulas_2.iter().map(|f| f.eval()).sum();
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
        assert_eq!(run(&sample_input), Ok((26386, 693942)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((24650385570008, 158183007916215)));
    }
}
