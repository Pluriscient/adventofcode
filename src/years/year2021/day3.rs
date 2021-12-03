use super::util::AResult;
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Output = isize;
const DAY: usize = 3;
fn to_u32(slice: &[bool]) -> u32 {
    slice
        .iter()
        // .rev()
        .fold(0, |acc, &b| if b { acc * 2 + 1 } else { acc * 2 } as u32)
}

fn extract_most_common(inputs: &[Input]) -> Vec<bool> {
    let input_len = inputs[0].bits.len();
    let input_counts: Vec<usize> =
        inputs
            .iter()
            .map(|input| &input.bits)
            .fold(vec![0; input_len], |mut acc, bits| {
                for (i, _) in bits.iter().enumerate().filter(|(_, bit)| **bit) {
                    acc[i] += 1;
                }
                acc
            });
    let all_ones = inputs.len();
    let gamma = input_counts
        .iter()
        // .enumerate()
        // .inspect(|(i, &count)| println!("{}: {}", i, count))
        // .map(|(_, count)| count)
        .map(|&count| count * 2 >= all_ones)
        // .enumerate()
        // .inspect(|(i, x)| println!("{}: {}", i, x))
        // .map(|(_, x)| x)
        .collect_vec();
    gamma
}

fn solve_part_one(inputs: &[Input]) -> Output {
    let input_len = inputs[0].bits.len();
    let input_counts: Vec<usize> =
        inputs
            .iter()
            .map(|input| &input.bits)
            .fold(vec![0; input_len], |mut acc, bits| {
                for (i, _) in bits.iter().enumerate().filter(|(_, bit)| **bit) {
                    acc[i] += 1;
                }
                acc
            });
    let all_ones = inputs.len();
    let gamma = input_counts
        .iter()
        .map(|&count| count > all_ones / 2)
        .collect_vec();
    let epsilon = (&gamma).iter().map(|g| !g).collect_vec();
    // println!("gamma: {:?}", gamma);
    // println!("epsilon: {:?}", epsilon);
    (to_u32(&gamma) * to_u32(&epsilon)) as isize
}

fn solve_part_two(inputs: &[Input]) -> Output {
    // let most_common = extract_most_common(inputs);
    let bit_length = inputs[0].bits.len();
    let mut remaining: Vec<Input> = inputs.to_vec();
    let mut oxygen = vec![false; bit_length];
    for i in 0..bit_length {
        let most_common = extract_most_common(&remaining);
        remaining.retain(|input| input.bits[i] == most_common[i]);
        // println!(
        //     "at position {} ({}), remaining: {:?}",
        //     i,
        //     most_common[i],
        //     remaining.len()
        // );
        // println!("{:?}", remaining);
        if remaining.len() == 1 {
            oxygen = remaining[0].bits.clone();
            break;
        }
    }
    remaining = inputs.to_vec();
    let mut co2 = vec![false; bit_length];
    for i in 0..bit_length {
        let most_common = extract_most_common(&remaining);
        remaining.retain(|input| input.bits[i] != most_common[i]);
        if remaining.len() == 1 {
            co2 = remaining[0].bits.clone();
            break;
        }
    }
    // println!("oxygen: {:?}", oxygen);
    // println!("co2: {:?}", co2);
    (to_u32(&oxygen) * to_u32(&co2)) as Output
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    bits: Vec<bool>,
}
impl Input {}

impl FromStr for Input {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> AResult<Self> {
        let bits = s.chars().map(|c| c == '1').collect_vec();
        Ok(Self { bits })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::super::util::{read_input, read_input_test};
    use super::*;
    use super::{solve_part_one, solve_part_two};
    use std::io::Error;
    use std::str::FromStr;

    fn parse_input() -> std::result::Result<Vec<Input>, Error> {
        let input = read_input(super::DAY)?;
        Ok(input
            .trim()
            .lines()
            .map(Input::from_str)
            .collect::<AResult<Vec<Input>>>()
            .unwrap())
    }

    #[test]
    fn test_part_one() -> Result<(), Error> {
        let input = parse_input()?;
        let solution = solve_part_one(&input);
        println!("solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let input = parse_input()?;
        let solution = solve_part_two(&input);
        println!("solution part 2: {}", solution);
        Ok(())
    }
}
