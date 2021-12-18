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
impl Day {
    fn print_bits(&self) {
        for bit in self.bits.iter() {
            print!("{}", if *bit { 1 } else { 0 });
        }
        println!();
    }
}
enum PacketValue {
    Literal(usize),
    Operator(usize, Vec<Packet>),
}

impl PacketValue {
    fn from_bits(value_type: usize, bits: &[bool]) -> PacketValue {
        match value_type {
            4 => PacketValue::Literal(bits_to_num(
                &bits
                    .windows(5)
                    .flat_map(|win| &win[1..])
                    .cloned()
                    .collect_vec(),
            )),
            _ => {
                let length_type = bits[0];
                let mut cur_bit = 1;
                let length = if length_type {
                    cur_bit = 15;
                    bits_to_num(&bits[1..15]) // total length
                } else {
                    cur_bit = 11;
                    bits_to_num(&bits[1..11]) // directly contained bits
                };
                // let mut children = Vec::new();
                // if length_type {

                // }
                // PacketValue::Operator(value_type, )
                todo!()
            } // _ => panic!("Invalid value type"),
        }
    }
}

struct Packet {
    version: u8,        // first three bits
    packet_type: u8,    // next three bits,
    value: PacketValue, // the remainder
}
fn bits_to_num(bits: &[bool]) -> usize {
    bits.iter()
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
    fn from_bits(bits: &[bool]) -> Packet {
        let version = bits_to_num(&bits[..3]);
        let packet_type = bits_to_num(&bits[3..6]);
        let value = match bits_to_num(&bits[6..]) {
            0 => PacketValue::Literal(bits_to_num(&bits[6..])),
            _ => PacketValue::Operator(bits_to_num(&bits[6..]), vec![]),
        };
        todo!()
    }
}

impl AOCDay for Day {
    const DAY: usize = 16;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        self.print_bits();

        0
    }
    fn part_two(&mut self) -> Self::Output {
        todo!()
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
