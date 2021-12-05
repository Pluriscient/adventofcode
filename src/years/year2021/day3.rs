use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day3;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day3 {
    inputs: Vec<Vec<bool>>,
}
/// Parse the vector of bools into an integer (most significant bit first)
fn to_u32(slice: &[bool]) -> u32 {
    slice
        .iter()
        .fold(0, |acc, &b| if b { acc * 2 + 1 } else { acc * 2 } as u32)
}

fn extract_most_common(inputs: &[Vec<bool>]) -> Vec<bool> {
    let input_len = inputs[0].len();
    let input_counts: Vec<usize> =
        inputs
            .iter()
            .map(|input| &input[..])
            .fold(vec![0; input_len], |mut acc, bits| {
                for (i, _) in bits.iter().enumerate().filter(|(_, bit)| **bit) {
                    acc[i] += 1;
                }
                acc
            });
    let all_ones = inputs.len();
    let most_common = input_counts
        .iter()
        .map(|&count| count * 2 >= all_ones)
        .collect_vec();
    most_common
}
impl AOCDay for Day {
    const DAY: usize = 3;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        let input_len = self.inputs[0].len();
        let input_counts: Vec<usize> =
            self.inputs
                .iter()
                .map(|input| &input[..])
                .fold(vec![0; input_len], |mut acc, bits| {
                    for (i, _) in bits.iter().enumerate().filter(|(_, bit)| **bit) {
                        acc[i] += 1;
                    }
                    acc
                });
        let all_ones = self.inputs.len();
        let gamma = input_counts
            .iter()
            .map(|&count| count > all_ones / 2)
            .collect_vec();
        let epsilon = (&gamma).iter().map(|g| !g).collect_vec();
        (to_u32(&gamma) * to_u32(&epsilon)) as isize
    }
    fn part_two(&mut self) -> Self::Output {
        let bit_length = self.inputs[0].len();
        let mut remaining: Vec<Vec<bool>> = self.inputs.to_vec();
        let mut oxygen = vec![false; bit_length];
        for i in 0..bit_length {
            let most_common = extract_most_common(&remaining);
            remaining.retain(|input| input[i] == most_common[i]);
            if remaining.len() == 1 {
                oxygen = remaining[0].clone();
                break;
            }
        }
        remaining = self.inputs.to_vec();
        let mut co2 = vec![false; bit_length];
        for i in 0..bit_length {
            let most_common = extract_most_common(&remaining);
            remaining.retain(|input| input[i] != most_common[i]);
            if remaining.len() == 1 {
                co2 = remaining[0].clone();
                break;
            }
        }
        (to_u32(&oxygen) * to_u32(&co2)) as Self::Output
    }
}
impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let inputs = s
            .lines()
            .map(|line| line.chars().map(|c| c == '1').collect_vec());
        Ok(Self {
            inputs: inputs.collect_vec(),
        })
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
