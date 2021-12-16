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
        let (packet, _) = parse_packet(&self.bits).unwrap();
        Answer::Integer(version_sum(&packet))
    }

    fn part_2(&self) -> Answer {
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
        } => {
            *v + s.iter().map(version_sum).sum::<usize>()
        }
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
            break
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
            type_id,
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
        type_id: usize,
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

#[cfg(test)]
mod test {
    use super::*;

    static INPUT_1: &str = "8A004A801A8002F478";
    static INPUT_2: &str = "620080001611562C8802118E34";
    static INPUT_3: &str = "C0015000016115A2E0802F182340";
    static INPUT_4: &str = "A0016C880162017C3686B18A3D4780";
    
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
        let runner = Day16::new(INPUT_1.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(16));

        let runner = Day16::new(INPUT_2.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(12));

        let runner = Day16::new(INPUT_3.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(23));

        let runner = Day16::new(INPUT_4.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(31));
    }

    // #[test]
    // fn part_2() {
        // let runner = Day16::new(INPUT_1.lines().map(|s| s.to_owned())).unwrap();
        // assert_eq!(runner.part_2(), Answer::None);
    // }
}
