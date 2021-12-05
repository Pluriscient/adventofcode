use super::util::AResult;
use super::AOCDay;
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day1 {
    measurements: Vec<i32>,
}

impl AOCDay for Day1 {
    const DAY: usize = 1;
    type Output = usize;

    fn part_one(&mut self) -> Self::Output {
        self.measurements
            .windows(2)
            .filter(|window| window[1] > window[0])
            .count()
    }
    fn part_two(&mut self) -> Self::Output {
        self.measurements = self
            .measurements
            .windows(3)
            .map(|window| window.iter().sum())
            .collect_vec();
        self.part_one()
    }
}
impl FromStr for Day1 {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let measurements: Vec<i32> = s
            .lines()
            .map(|line| i32::from_str(line.trim()))
            .collect::<Result<Vec<i32>, _>>()
            .unwrap();
        Ok(Self { measurements })
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{test_day_part_one, test_day_part_two};
    use super::*;
    type Day = Day1;
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
