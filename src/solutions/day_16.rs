use bitvec::prelude::*;

use crate::prelude::*;

pub struct Day16 {
    bits: BitVec,
}

impl Day for Day16 {
    fn new(mut input: impl Iterator<Item = String>) -> Result<Self, AOCError> {
        let input = input.next().ok_or(AOCError::NoInput)?;
        let mut bits: BitVec = BitVec::with_capacity(4 * input.len());
        for c in input.chars() {
            let mut bit = 4;
            let digit = c.to_digit(16).ok_or(AOCError::ParseError)?;
            bits.resize_with(4 + bits.len(), || {
                bit -= 1;
                (1 << bit) & digit != 0
            });
        }
        Ok(Self { bits })
    }

    fn part_1(&self) -> Answer {
        if let Some((packet, _)) = parse_packet(&self.bits) {
            Answer::Integer(version_sum(&packet))
        } else {
            Answer::None
        }
    }

    fn part_2(&self) -> Answer {
        if let Some((packet, _)) = parse_packet(&self.bits) {
            if let Some(v) = evaluate(&packet) {
                return Answer::Integer(v);
            }
        }
        Answer::None
    }
}

fn version_sum(packet: &Packet) -> usize {
    match packet {
        Packet::Literal { version: v, .. } => *v,
        Packet::Operator {
            version: v,
            subpackets: s,
            ..
        } => *v + s.iter().map(version_sum).sum::<usize>(),
    }
}

fn evaluate(packet: &Packet) -> Option<usize> {
    match packet {
        Packet::Literal { value: v, .. } => Some(*v),
        Packet::Operator {
            type_id: t,
            subpackets: s,
            ..
        } => match t {
            OperatorType::Sum => s.iter().map(evaluate).sum(),
            OperatorType::Product => s.iter().map(evaluate).product(),
            OperatorType::Minimum => s.iter().map(evaluate).min().flatten(),
            OperatorType::Maximum => s.iter().map(evaluate).max().flatten(),
            OperatorType::GreaterThan => match evaluate(s.get(0)?) > evaluate(s.get(1)?) {
                true => Some(1),
                false => Some(0),
            },
            OperatorType::LessThan => match evaluate(s.get(0)?) < evaluate(s.get(1)?) {
                true => Some(1),
                false => Some(0),
            },
            OperatorType::EqualTo => match evaluate(s.get(0)?) == evaluate(s.get(1)?) {
                true => Some(1),
                false => Some(0),
            },
        },
    }
}

fn parse_packet(bits: &BitSlice) -> Option<(Packet, &BitSlice)> {
    // Version
    if bits.len() < 3 {
        return None;
    }
    let (version_bits, bits) = bits.split_at(3);
    let version = to_int(version_bits);

    // Type ID
    if bits.len() < 3 {
        return None;
    }
    let (type_id_bits, bits) = bits.split_at(3);
    let type_id = TypeID::from_int(to_int(type_id_bits));

    // Parse packet
    match type_id {
        TypeID::Literal => parse_literal(version, bits),
        TypeID::Operator(type_id) => parse_operator(version, type_id, bits),
    }
}

fn parse_literal(version: usize, mut bits: &BitSlice) -> Option<(Packet, &BitSlice)> {
    let mut n = 0;
    loop {
        if bits.len() < 5 {
            return None;
        }

        let temp = bits.split_at(5);
        let cur_bits = temp.0;
        bits = temp.1;

        n <<= 4;
        n |= to_int(&cur_bits[1..5]);

        if !cur_bits[0] {
            break;
        }
    }

    Some((Packet::Literal { version, value: n }, bits))
}

fn parse_operator(version: usize, type_id: usize, bits: &BitSlice) -> Option<(Packet, &BitSlice)> {
    if bits.is_empty() {
        return None;
    }
    let (length_type_id_bits, mut bits) = bits.split_at(1);

    let mut subpackets = Vec::new();
    match length_type_id_bits[0] {
        true => {
            // Next 11 bits contains number of subpackets
            if bits.len() < 11 {
                return None;
            }
            let temp = bits.split_at(11);
            let num_subpackets = to_int(temp.0);
            bits = temp.1;

            // Parse subpackets
            for _ in 0..num_subpackets {
                let temp = parse_packet(bits).unwrap();
                let packet = temp.0;
                bits = temp.1;
                subpackets.push(packet);
            }
        }
        false => {
            // Next 15 bits contains total length in bits of subpackets
            if bits.len() < 15 {
                return None;
            }
            let temp = bits.split_at(15);
            let subpackets_len = to_int(temp.0);
            bits = temp.1;

            // Split off subpackets
            if bits.len() < subpackets_len {
                return None;
            }
            let temp = bits.split_at(subpackets_len);
            let mut packet_bits = temp.0;
            bits = temp.1;

            // Parse subpackets
            while !packet_bits.is_empty() {
                let temp = parse_packet(packet_bits).unwrap();
                let packet = temp.0;
                packet_bits = temp.1;
                subpackets.push(packet);
            }
        }
    }

    Some((
        Packet::Operator {
            version,
            type_id: type_id.try_into().ok()?,
            subpackets,
        },
        bits,
    ))
}

fn to_int(bits: &BitSlice) -> usize {
    let mut n = 0;
    for b in bits {
        n <<= 1;
        n |= match *b {
            true => 1,
            false => 0,
        };
    }
    n
}

#[derive(Debug)]
enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operator {
        version: usize,
        type_id: OperatorType,
        subpackets: Vec<Packet>,
    },
}

#[derive(Debug)]
enum TypeID {
    Literal,
    Operator(usize),
}

impl TypeID {
    fn from_int(n: usize) -> Self {
        match n {
            4 => Self::Literal,
            _ => Self::Operator(n),
        }
    }
}

#[derive(Debug)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl TryFrom<usize> for OperatorType {
    type Error = AOCError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Sum),
            1 => Ok(Self::Product),
            2 => Ok(Self::Minimum),
            3 => Ok(Self::Maximum),
            5 => Ok(Self::GreaterThan),
            6 => Ok(Self::LessThan),
            7 => Ok(Self::EqualTo),
            _ => Err(AOCError::SolutionError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn len_type_0() {
        let runner = Day16::new("38006F45291200".lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(9));
    }

    #[test]
    fn len_type_1() {
        let runner = Day16::new("EE00D40C823060".lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(14));
    }

    #[test]
    fn part_1() {
        let runner = Day16::new("8A004A801A8002F478".lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(16));

        let runner =
            Day16::new("620080001611562C8802118E34".lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(12));

        let runner =
            Day16::new("C0015000016115A2E0802F182340".lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(23));

        let runner = Day16::new(
            "A0016C880162017C3686B18A3D4780"
                .lines()
                .map(|s| s.to_owned()),
        )
        .unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(31));
    }

    #[test]
    fn part_2() {
        let runner = Day16::new("C200B40A82".lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(3));

        let runner = Day16::new("04005AC33890".lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(54));

        let runner = Day16::new("880086C3E88112".lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(7));

        let runner = Day16::new("CE00C43D881120".lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(9));

        let runner = Day16::new("D8005AC2A8F0".lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(1));

        let runner = Day16::new("F600BC2D8F".lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(0));

        let runner = Day16::new("9C005AC2F8F0".lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(0));

        let runner =
            Day16::new("9C0141080250320F1802104A08".lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(1));
    }
}
