use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    PacketTooShort(String),
    ParseIntError(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PacketTooShort(v) => write!(f, "Packet is too short: {v}"),
            Self::ParseIntError(c) => write!(f, "Unable to parse {c} into integer"),
        }
    }
}

#[derive(Debug)]
enum PacketValue {
    Sum(Vec<Packet>),
    Mul(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Literal(usize),
    Greater(Vec<Packet>),
    Less(Vec<Packet>),
    Equal(Vec<Packet>),
}

impl PacketValue {
    fn from(type_id: u8, sub_packets: Vec<Packet>) -> Self {
        match type_id {
            0 => Self::Sum(sub_packets),
            1 => Self::Mul(sub_packets),
            2 => Self::Min(sub_packets),
            3 => Self::Max(sub_packets),
            5 => Self::Greater(sub_packets),
            6 => Self::Less(sub_packets),
            7 => Self::Equal(sub_packets),
            _ => panic!("Unexpected type id: {type_id}"),
        }
    }

    fn packets(&self) -> &Vec<Packet> {
        match self {
            PacketValue::Literal(_) => panic!("Tried to access packets of literal {self:?}"),
            PacketValue::Sum(p) => p,
            PacketValue::Mul(p) => p,
            PacketValue::Min(p) => p,
            PacketValue::Max(p) => p,
            PacketValue::Greater(p) => p,
            PacketValue::Less(p) => p,
            PacketValue::Equal(p) => p,
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    value: PacketValue,
}

impl TryFrom<&Vec<bool>> for Packet {
    type Error = ParseError;

    fn try_from(value: &Vec<bool>) -> Result<Self, Self::Error> {
        Ok(Self::parse(value)?.0)
    }
}

impl Packet {
    fn parse(value: &[bool]) -> Result<(Self, usize), ParseError> {
        if value.len() < 6 {
            return Err(ParseError::PacketTooShort(format!("{value:?}")));
        }
        let version = from_bits(&value[0..3]);
        let type_id = from_bits(&value[3..6]);
        if type_id == 4 {
            let (value, size) = decode_value(&value[6..]);

            Ok((Self { 
                version, 
                value: PacketValue::Literal(value),
            }, size+6))
        } else {
            let length_type = if value[6] {
                11
            } else {
                15
            };
            let mut next_idx = 7+length_type;
            let length = from_bits(&value[7..next_idx]);
            let mut sub_packets = Vec::new();
            if length_type == 11 {
                for _ in 0..length {
                    let sub_packet = Self::parse(&value[next_idx..])?;
                    sub_packets.push(sub_packet.0);
                    next_idx += sub_packet.1;
                }
            } else {
                let last_idx = next_idx+length;
                while next_idx < last_idx {
                    let sub_packet = Self::parse(&value[next_idx..last_idx])?;
                    sub_packets.push(sub_packet.0);
                    next_idx += sub_packet.1;
                }
            }
            Ok((Self {
                version, 
                value: PacketValue::from(type_id, sub_packets),
            }, next_idx))

        }
    }

    fn sum_version_numbers(&self) -> usize {
        self.version as usize + match &self.value {
            PacketValue::Literal(_) => 0,
            op => op.packets().iter().map(|p| p.sum_version_numbers()).sum(),
        }
    }

    fn evaluate(&self) -> usize {
        match &self.value {
            PacketValue::Literal(v) => *v,
            PacketValue::Sum(packets) => packets.iter().map(|p| p.evaluate()).sum(),
            PacketValue::Mul(packets) => packets.iter().map(|p| p.evaluate()).product(),
            PacketValue::Min(packets) => packets.iter().map(|p| p.evaluate()).min().unwrap(),
            PacketValue::Max(packets) => packets.iter().map(|p| p.evaluate()).max().unwrap(),
            PacketValue::Greater(packets) => if packets[0].evaluate() > packets[1].evaluate() { 1 } else { 0 },
            PacketValue::Less(packets) => if packets[0].evaluate() < packets[1].evaluate() { 1 } else { 0 },
            PacketValue::Equal(packets) => if packets[0].evaluate() == packets[1].evaluate() { 1 } else { 0 },
        }
    }
}

pub fn run(input: &str) -> Result<(usize, usize), ParseError> {
    let bits: Vec<_> = hex_to_bitstream(input)?;
    let packets = Packet::try_from(&bits)?;
    let first = packets.sum_version_numbers();
    let second = packets.evaluate();
    Ok((first, second))
}

fn hex_to_bitstream(value: &str) -> Result<Vec<bool>, ParseError> {
    let mut res = Vec::new();

    for c in value.chars() {
        if let Ok(bits) = try_to_bits(c) {
            res.append(&mut bits.clone());
        }
    }

    Ok(res)
}

fn try_to_bits(original_char: char) -> Result<Vec<bool>, ParseError> {
    if let Some(value) = original_char.to_digit(16) {
        Ok((0..4).map(|idx| (value & 2_u32.pow(3-idx)) > 0).collect())
    } else {
        Err(ParseError::ParseIntError(original_char))
    }
}

fn from_bits<T>(bits: &[bool]) -> T
where T: std::ops::Add<Output = T> + std::ops::Shl<Output = T> + std::convert::From<u8> {
    bits.iter().fold(T::from(0), |acc, cur| acc.shl(T::from(1)) + if *cur { T::from(1) } else { T::from(0) })
}

fn decode_value(bits: &[bool]) -> (usize, usize) {
    let mut res = 0;
    let mut size = 5;

    for c in bits.chunks(5) {
        res *= 16;
        res += from_bits::<usize>(&c[1..]);
        if !c[0] {
            break;
        }
        size += 5;
    }
    (res, size)
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
        let expected = [
            (16, 15),
            (12, 46),
            (23, 46),
            (31, 54),
            (14, 3),
            (8, 54),
            (15, 7),
            (11, 9),
            (13, 1),
            (19, 0),
            (16, 0),
            (20, 1),
        ];
        for (idx, line) in sample_input.lines().enumerate() {
            assert_eq!(run(line), Ok(expected[idx]));
        }
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((891, 673042777597)));
    }
}
