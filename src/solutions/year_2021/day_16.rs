use crate::input;
use std::collections::VecDeque;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Bit { One, Zero }

impl Add<u64> for Bit {
    type Output = u64;
    fn add(self, other: u64) -> Self::Output {
        other + match self {
            Self::One => 1, Self::Zero => 0
        }
    }
}


#[derive(Debug)]
enum Packet {
    Literal {
        version: u64,
        type_id: u64,
        value: u64
    },
    Operator {
        version: u64,
        type_id: u64,
        packets: Vec<Packet>
    },
}

pub fn run() {
    let mut bits = get_binary_input();
    let parent = parse_packet(&mut bits);

    println!(" Part 1: {}", add_versions(&parent));
    println!(" Part 2: {}", execute(&parent));
}

fn add_versions(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { version, .. } => 
            *version,

        Packet::Operator { version, packets, .. } => 
            packets.iter().fold(*version, |sum, p| sum + add_versions(&p)),
    }
}

fn execute(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { value, .. } => {
            *value
        },
        Packet::Operator { type_id: 0, packets, .. } => {
            packets.iter().fold(0, |sum, p| sum + execute(&p))
        },
        Packet::Operator { type_id: 1, packets, .. } => {
            packets.iter().fold(1, |sum, p| sum * execute(&p))
        },
        Packet::Operator { type_id: 2, packets, .. } => {
            packets
                .iter()
                .map(|p| execute(&p))
                .min()
                .expect("min operator had empty vector")
        },
        Packet::Operator { type_id: 3, packets, .. } => {
            packets
                .iter()
                .map(|p| execute(&p))
                .max()
                .expect("max operator had empty vector")
        },
        Packet::Operator { type_id: 5, packets, .. } => {
            if execute(&packets[0]) > execute(&packets[1]) {
                1
            } else {
                0
            }
        },
        Packet::Operator { type_id: 6, packets, .. } => {
            if execute(&packets[0]) < execute(&packets[1]) {
                1
            } else {
                0
            }
        },
        Packet::Operator { type_id: 7, packets, .. } => {
            if execute(&packets[0]) == execute(&packets[1]) {
                1
            } else {
                0
            }
        },
        _ => panic!("unrecognized operator")
    }
}

fn parse_packet(bits: &mut VecDeque<Bit>) -> Packet {
    let version = pull_n_bits(bits, 3);
    let type_id = pull_n_bits(bits, 3);

    match type_id {
        4 => Packet::Literal {
            version,
            type_id,
            value: pull_literal_val(bits),
        },

        _ => {
            let mut packets: Vec<Packet> = Vec::new();
            let length_type_id = pull_bit(bits);
            
            match length_type_id {
                Bit::Zero => {
                    let length = pull_n_bits(bits, 15) as usize;
                    let starting_length = bits.len();
                    while starting_length - bits.len() < length {
                        packets.push(parse_packet(bits));
                    }
                },

                Bit::One => {
                    let count = pull_n_bits(bits, 11);
                    for _ in 0..count {
                        packets.push(parse_packet(bits));
                    }
                }
            };

            Packet::Operator {
                version,
                type_id,
                packets,
            }
        }
    }
}

fn pull_bit(bits: &mut VecDeque<Bit>) -> Bit {
    if let Some(bit) = bits.pop_front() {
        bit
    } else {
        panic!("Unexpectedly ran out of bits")
    }
}

fn pull_n_bits(bits: &mut VecDeque<Bit>, count: usize) -> u64 {
    let mut val: u64 = 0;
    for _ in 0..count {
        let bit = pull_bit(bits);

        val = bit + val * 2;
    }

    val
}

fn pull_literal_val(bits: &mut VecDeque<Bit>) -> u64 {
    let mut val: u64 = 0;
    loop {
        let control = pull_n_bits(bits, 1);
        val = val * 16 + pull_n_bits(bits, 4);

        if control == 0 { break }
    }

    val
}

fn get_binary_input() -> VecDeque<Bit> {
    input::read_all("2021_16")
        .expect("couldn't read input")
        .chars()
        .map(hex_to_bin)
        .collect::<String>()
        .chars()
        .map(char_to_bit)
        .collect::<VecDeque<Bit>>()
}

fn hex_to_bin(ch: char) -> &'static str {
    match ch {
        '0' => "0000", '1' => "0001", '2' => "0010", '3' => "0011",
        '4' => "0100", '5' => "0101", '6' => "0110", '7' => "0111",
        '8' => "1000", '9' => "1001", 'A' => "1010", 'B' => "1011",
        'C' => "1100", 'D' => "1101", 'E' => "1110", 'F' => "1111",
        _ => panic!("unknown char"),
    }
}

fn char_to_bit(ch: char) -> Bit {
    match ch { 
        '0' => Bit::Zero, '1' => Bit::One, 
        _ => panic!("unkown char")
    }
}