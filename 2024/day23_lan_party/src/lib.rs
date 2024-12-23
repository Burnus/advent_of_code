use core::fmt::Display;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    ComputerName(&'a str),
    LineMalformed(&'a str),
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComputerName(e) => write!(f, "Computer name doesn't consist of two ascii characters: \"{e}\"."),
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
        }
    }
}

type Connection = u32;

fn try_connection_from(value: &str) -> Result<Connection, ParseError> {
    if let Some((lhs, rhs)) = value.split_once('-') {
        if lhs.len() != 2 {
            return Err(ParseError::ComputerName(lhs));
        }
        let lhs_bytes = lhs.as_bytes();
        let lhs = ((lhs_bytes[0] as u32) << 8) | (lhs_bytes[1] as u32);
        if rhs.len() != 2 {
            return Err(ParseError::ComputerName(rhs));
        }
        let rhs_bytes = rhs.as_bytes();
        let rhs = ((rhs_bytes[0] as u32) << 8) | rhs_bytes[1] as u32;

        Ok((lhs.min(rhs) << 16) | (lhs.max(rhs)))
    } else {
        Err(ParseError::LineMalformed(value))
    }
}

fn triples(conns: &HashSet<Connection>) -> Vec<(u16, u16, u16)> {
    let mut res = Vec::new();
    conns.iter().for_each(|conn| {
        let (lhs, rhs) = ((conn >> 16) as u16, (conn & 0xFFFF) as u16);
        conns.iter().filter(|&&other| (other >> 16) as u16 == lhs).for_each(|new| {
            let new = new & 0xFFFF;
            if conns.contains(&(((rhs as u32) << 16) | new)) {
                res.push((lhs, rhs, new as u16));
            }
        });
    });
    res
}

fn largest_clique(triples: &[(u16, u16, u16)], conns: &HashSet<u32>) -> Vec<u16> {
    let mut cliques: Vec<_> = triples.iter().map(|(l, m, r)| vec![*l, *m, *r]).collect();
    let mut next_cliques = Vec::with_capacity(cliques.len());
    while cliques.len() > 1 {
        for clique in cliques.iter() {
            conns.iter()
                .filter(|&&conn| (conn >> 16) as u16 == clique[0])
                .for_each(|&conn| {
                    let new = (conn & 0xFFFF) as u16;
                    if clique.iter().skip(1).all(|old| conns.contains(&((*old as u32) << 16 | new as u32))) {
                        let mut new_clique = clique.to_vec();
                        new_clique.push(new);
                        next_cliques.push(new_clique);
                    }
                });
        }
        std::mem::swap(&mut cliques, &mut next_cliques);
        next_cliques.clear();
    }
    cliques[0].to_vec()
}

fn password(computers: &[u16]) -> String {
    let computers = computers.to_vec();
    computers
        .iter()
        .map(|n| String::from_utf8(vec![(n >> 8) as u8, (n & 0xFF) as u8]).unwrap())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn run(input: &str) -> Result<(usize, String), ParseError> {
    const FIRST_LETTER_T: u16 = (b't' as u16) << 8;
    let conns: HashSet<_> = input.lines().map(try_connection_from).collect::<Result<HashSet<_>, _>>()?;
    let triples = triples(&conns);
    let first = triples
        .iter()
        .filter(|(l, m, r)|
            l & 0xFF00 == FIRST_LETTER_T ||
            m & 0xFF00 == FIRST_LETTER_T ||
            r & 0xFF00 == FIRST_LETTER_T)
        .count();
    let party = largest_clique(&triples, &conns);
    let second = password(&party);
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
        assert_eq!(run(&sample_input), Ok((7, "co,de,ka,ta".to_string())));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((1304, "ao,es,fe,if,in,io,ky,qq,rd,rn,rv,vc,vl".to_string())));
    }
}
