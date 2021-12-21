use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day16;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day16 {
    bits: Vec<bool>,
}
fn print_bits(bits: &[bool]) {
    for bit in bits.iter() {
        print!("{}", if *bit { 1 } else { 0 });
    }
    println!();
}

impl Day {}

#[derive(Debug, Eq, PartialEq)]
enum PacketValue {
    Literal(usize),
    Operator(usize, Vec<Packet>),
}

impl PacketValue {
    fn from_bits(value_type: usize, bits: &[bool]) -> (PacketValue, usize) {
        match value_type {
            4 => {
                let mut cur_bit = 5;
                let mut chunks = bits.chunks(5);
                let mut value_bits = chunks
                    .take_while_ref(|t| t[0])
                    .map(|t| {
                        cur_bit += 5;
                        &t[1..]
                    })
                    .collect_vec();
                value_bits.push(&chunks.next().unwrap()[1..]);
                // println!("value bits: {:?}", &value_bits);
                let value_bits = value_bits.iter().flat_map(|t| *t).cloned().collect_vec();
                // print_bits(&value_bits);
                let value = PacketValue::Literal(bits_to_num(&value_bits));
                (value, cur_bit)
            }
            _ => {
                let length_type = bits[0];
                let mut cur_bit;
                let length = if length_type {
                    cur_bit = 12;
                    bits_to_num(&bits[1..12]) // total length
                } else {
                    cur_bit = 16;
                    bits_to_num(&bits[1..16]) // directly contained bits
                };
                // println!("length {} of type {}", length, length_type);
                let mut children = Vec::new();
                if !length_type {
                    // while (cur_bit) is not (length), we add children
                    while (cur_bit - 16) < length {
                        // println!("current bit: {}", cur_bit);
                        // print_bits(&bits[cur_bit..]);
                        let (child, increase) = Packet::from_bits(&bits[cur_bit..]);
                        cur_bit += increase;
                        children.push(child);
                    }
                } else {
                    for _ in 0..length {
                        let (child, increase) = Packet::from_bits(&bits[cur_bit..]);
                        cur_bit += increase;
                        children.push(child);
                    }
                }
                (PacketValue::Operator(value_type, children), cur_bit)
            }
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: u8,        // first three bits
    packet_type: u8,    // next three bits,
    value: PacketValue, // the remainder
}
fn bits_to_num(bits: &[bool]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, bit)| acc + if *bit { 1 << i } else { 0 })
}

impl Packet {
    fn new(version: u8, packet_type: u8, value: PacketValue) -> Packet {
        Packet {
            version,
            packet_type,
            value,
        }
    }

    fn from_bits(bits: &[bool]) -> (Packet, usize) {
        // println!("Reading in packet");
        let version = bits_to_num(&bits[..3]);
        let packet_type = bits_to_num(&bits[3..6]);
        // println!("packet version: {}", version);
        // println!("packet type: {}", packet_type);
        let (val, cur_bit) = PacketValue::from_bits(packet_type, &bits[6..]);
        // println!("packet value: {:?}", val);
        // println!("cur bit: {}", cur_bit);
        (
            Self {
                version: version as u8,
                packet_type: packet_type as u8,
                value: val,
            },
            6 + cur_bit,
        )
    }

    fn total_version(&self) -> usize {
        self.version as usize
            + match self.value {
                PacketValue::Literal(_) => 0,
                PacketValue::Operator(_, ref children) => {
                    children.iter().map(|c| c.total_version()).sum::<usize>()
                }
            }
    }

    fn solve(&self) -> isize {
        match self.value {
            PacketValue::Literal(val) => val as isize,
            PacketValue::Operator(op, ref children) => {
                let mut children_vals = Vec::new();
                for child in children {
                    children_vals.push(child.solve());
                }
                match op {
                    0 => children_vals.iter().sum::<isize>(),
                    2 => *children_vals.iter().min().unwrap(),
                    3 => *children_vals.iter().max().unwrap(),
                    1 => children_vals.iter().product::<isize>(),
                    5 => {
                        if children_vals[0] > children_vals[1] {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if children_vals[0] < children_vals[1] {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if children_vals[0] == children_vals[1] {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Unknown operator"),
                }
            }
        }
    }
}

impl AOCDay for Day {
    const DAY: usize = 16;
    type Output = usize;

    fn part_one(&mut self) -> Self::Output {
        // println!("{:?}", self);
        print_bits(self.bits.as_slice());
        let (packet, _) = Packet::from_bits(&self.bits);
        // println!("{:?}", packet);
        packet.total_version()
    }
    fn part_two(&mut self) -> Self::Output {
        let (packet, _) = Packet::from_bits(&self.bits);
        packet.solve() as usize
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let bits = s
            .chars()
            .flat_map(|c| {
                // println!("{}", c);
                format!(
                    "{:04b}",
                    isize::from_str_radix(c.to_string().as_str(), 16).expect("Failed to parse")
                )
                .chars()
                .map(|x| x == '1')
                .collect::<Vec<bool>>()
            })
            .collect::<Vec<bool>>();

        Ok(Self { bits })
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{test_day_part_one, test_day_part_two};
    use super::*;
    #[test]
    fn part_one_test() -> Result<(), std::io::Error> {
        test_day_part_one::<Day>(true)
    }
    #[test]
    fn part_one() -> Result<(), std::io::Error> {
        test_day_part_one::<Day>(false)
    }
    #[test]
    fn part_two_test() -> Result<(), std::io::Error> {
        test_day_part_two::<Day>(true)
    }
    #[test]
    fn part_two() -> Result<(), std::io::Error> {
        test_day_part_two::<Day>(false)
    }
}
