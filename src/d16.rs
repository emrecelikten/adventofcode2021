use crate::common::error::CommonError;

#[derive(PartialEq, Debug)]
struct ParseResult {
    result: Packet,
    pos: usize,
}

#[derive(PartialEq, Debug)]
enum Packet {
    Literal {
        version: u64,
        num: u64,
    },
    Operator {
        version: u64,
        type_id: PacketType,
        length_type_id: u64,
        sub_packets: Vec<Packet>,
    },
}

#[derive(PartialEq, Debug)]
enum PacketType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

impl TryFrom<u64> for PacketType {
    type Error = CommonError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            i if i == PacketType::Sum as u64 => Ok(PacketType::Sum),
            i if i == PacketType::Product as u64 => Ok(PacketType::Product),
            i if i == PacketType::Minimum as u64 => Ok(PacketType::Minimum),
            i if i == PacketType::Maximum as u64 => Ok(PacketType::Maximum),
            i if i == PacketType::Literal as u64 => Ok(PacketType::Literal),
            i if i == PacketType::GreaterThan as u64 => Ok(PacketType::GreaterThan),
            i if i == PacketType::LessThan as u64 => Ok(PacketType::LessThan),
            i if i == PacketType::EqualTo as u64 => Ok(PacketType::EqualTo),
            _ => Err(CommonError::Parse("Invalid operator type!")),
        }
    }
}

fn hexadecimal_str_to_binary<S: AsRef<str>>(hexes: S) -> Vec<char> {
    let mut result = Vec::new();
    for ch in hexes.as_ref().chars() {
        result.extend(format!("{:04b}", ch.to_digit(16).unwrap()).chars())
    }
    result
}

fn binary_to_decimal(bits: &[char]) -> u64 {
    bits.iter()
        .fold(0, |acc, e| (acc << 1) | (e.to_digit(2).unwrap() as u64))
}

fn parse_bits(bits: &[char]) -> Packet {
    fn parse_packet(bits: &[char]) -> ParseResult {
        let version = binary_to_decimal(&bits[0..3]);
        let type_id = binary_to_decimal(&bits[3..6]);

        if type_id == PacketType::Literal as u64 {
            let mut cur_pos = 6;
            let mut num: u64 = 0;
            while bits[cur_pos] != '0' {
                num = (num << 4) | binary_to_decimal(&bits[cur_pos + 1..cur_pos + 5]);
                cur_pos += 5;
            }
            num = (num << 4) | binary_to_decimal(&bits[cur_pos + 1..cur_pos + 5]);
            cur_pos += 5;

            ParseResult {
                result: Packet::Literal { version, num },
                pos: cur_pos,
            }
        } else {
            let length_type_id = binary_to_decimal(&bits[6..=6]);
            let mut sub_packets = Vec::new();
            if length_type_id == 0 {
                let mut cur_pos = 22;
                let until_pos = binary_to_decimal(&bits[7..22]) as usize + cur_pos;

                while cur_pos < until_pos {
                    let parse_result = parse_packet(&bits[cur_pos..]);
                    cur_pos += parse_result.pos;
                    sub_packets.push(parse_result.result);
                }

                ParseResult {
                    result: Packet::Operator {
                        version,
                        type_id: type_id.try_into().unwrap(),
                        length_type_id,
                        sub_packets,
                    },
                    pos: cur_pos,
                }
            } else {
                let length = binary_to_decimal(&bits[7..18]) as usize;
                let mut cur_command = 0;
                let mut cur_pos = 18;

                while cur_command < length {
                    let parse_result = parse_packet(&bits[cur_pos..]);
                    cur_pos += parse_result.pos;
                    cur_command += 1;
                    sub_packets.push(parse_result.result);
                }

                ParseResult {
                    result: Packet::Operator {
                        version,
                        type_id: type_id.try_into().unwrap(),
                        length_type_id,
                        sub_packets,
                    },
                    pos: cur_pos,
                }
            }
        }
    }
    parse_packet(bits).result
}

fn sum_versions(packet: &Packet) -> u64 {
    fn recursive_sum(packet: &Packet) -> u64 {
        match packet {
            Packet::Literal { version, .. } => *version,
            Packet::Operator {
                version,
                sub_packets,
                ..
            } => *version + sub_packets.iter().map(|p| recursive_sum(p)).sum::<u64>(),
        }
    }
    recursive_sum(packet)
}

fn process_packet(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { num, .. } => *num as u64,
        Packet::Operator {
            type_id,
            sub_packets,
            ..
        } if *type_id == PacketType::Sum => sub_packets.iter().map(|p| process_packet(p)).sum(),
        Packet::Operator {
            type_id,
            sub_packets,
            ..
        } if *type_id == PacketType::Product => {
            sub_packets.iter().map(|p| process_packet(p)).product()
        }
        Packet::Operator {
            type_id,
            sub_packets,
            ..
        } if *type_id == PacketType::Minimum => {
            sub_packets.iter().map(|p| process_packet(p)).min().unwrap()
        }
        Packet::Operator {
            type_id,
            sub_packets,
            ..
        } if *type_id == PacketType::Maximum => {
            sub_packets.iter().map(|p| process_packet(p)).max().unwrap()
        }
        Packet::Operator {
            type_id,
            sub_packets,
            ..
        } if *type_id == PacketType::GreaterThan => {
            let mut results = sub_packets.iter().map(|p| process_packet(p));
            if results.next().unwrap() > results.next().unwrap() {
                1
            } else {
                0
            }
        }
        Packet::Operator {
            type_id,
            sub_packets,
            ..
        } if *type_id == PacketType::LessThan => {
            let mut results = sub_packets.iter().map(|p| process_packet(p));
            if results.next().unwrap() < results.next().unwrap() {
                1
            } else {
                0
            }
        }
        Packet::Operator {
            type_id,
            sub_packets,
            ..
        } if *type_id == PacketType::EqualTo => {
            let mut results = sub_packets.iter().map(|p| process_packet(p));
            if results.next().unwrap() == results.next().unwrap() {
                1
            } else {
                0
            }
        }
        _ => {
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_parse_literal() {
        let data: Vec<char> = "110100101111111000101000".chars().collect();
        let result = parse_bits(&data);
        assert_eq!(
            result,
            Packet::Literal {
                version: 6,
                num: 2021
            }
        );
    }

    #[test]
    fn test_parse_command_type_0() {
        let data: Vec<char> = "00111000000000000110111101000101001010010001001000000000"
            .chars()
            .collect();
        let result = parse_bits(&data);
        assert_eq!(
            result,
            Packet::Operator {
                version: 1,
                type_id: 6.try_into().unwrap(),
                length_type_id: 0,
                sub_packets: vec![
                    Packet::Literal {
                        version: 6,
                        num: 10,
                    },
                    Packet::Literal {
                        version: 2,
                        num: 20
                    }
                ]
            }
        );
    }

    #[test]
    fn test_parse_command_type_1() {
        let data: Vec<char> = "11101110000000001101010000001100100000100011000001100000"
            .chars()
            .collect();
        let result = parse_bits(&data);
        assert_eq!(
            result,
            Packet::Operator {
                version: 7,
                type_id: 3.try_into().unwrap(),
                length_type_id: 1,
                sub_packets: vec![
                    Packet::Literal { version: 2, num: 1 },
                    Packet::Literal { version: 4, num: 2 },
                    Packet::Literal { version: 1, num: 3 },
                ]
            }
        );
    }

    #[test]
    fn test_parse_and_sum_versions() {
        fn sum_helper(data: &str) -> u64 {
            let bits = hexadecimal_str_to_binary(&data);
            let packet = parse_bits(&bits);
            sum_versions(&packet)
        }
        let data1 = "8A004A801A8002F478";
        assert_eq!(sum_helper(data1), 16);

        let data2 = "620080001611562C8802118E34";
        assert_eq!(sum_helper(data2), 12);

        let data3 = "C0015000016115A2E0802F182340";
        assert_eq!(sum_helper(data3), 23);

        let data4 = "A0016C880162017C3686B18A3D4780";
        assert_eq!(sum_helper(data4), 31);
    }

    #[test]
    fn test_process_packet() {
        fn process_helper(data: &str) -> u64 {
            let bits = hexadecimal_str_to_binary(&data);
            let packet = parse_bits(&bits);
            process_packet(&packet)
        }
        let data1 = "C200B40A82";
        assert_eq!(process_helper(data1), 3);

        let data2 = "04005AC33890";
        assert_eq!(process_helper(data2), 54);

        let data3 = "CE00C43D881120";
        assert_eq!(process_helper(data3), 9);

        let data4 = "D8005AC2A8F0";
        assert_eq!(process_helper(data4), 1);

        let data5 = "F600BC2D8F";
        assert_eq!(process_helper(data5), 0);

        let data6 = "9C005AC2F8F0";
        assert_eq!(process_helper(data6), 0);

        let data7 = "9C0141080250320F1802104A08";
        assert_eq!(process_helper(data7), 1);
    }

    #[test]
    fn test_d16() {
        let data = read_to_string("inputs/d16").unwrap();
        let bits = hexadecimal_str_to_binary(&data.trim());
        let packet = parse_bits(&bits);
        let version_sum = sum_versions(&packet);

        println!("Day 16 result #1: {}", version_sum);

        let processed_result = process_packet(&packet);
        println!("Day 16 result #2: {}", processed_result);
    }
}
