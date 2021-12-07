use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use itertools::MinMaxResult;
use std::error::Error;
use std::str::FromStr;

type Day = Day7;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day7 {
    positions: Vec<i32>,
}

impl AOCDay for Day {
    const DAY: usize = 7;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        // we need to minimize the number of steps towards the middle
        // for the sum of the positions
        let (min_steps, max_steps) = match self.positions.iter().minmax() {
            MinMaxResult::MinMax(min, max) => (min, max),
            _ => panic!("No two values in positions"),
        };
        ((*min_steps)..(*max_steps))
            .map(|i| self.positions.iter().map(|&x| (x - i).abs()).sum::<i32>())
            .min()
            .unwrap() as isize
    }
    fn part_two(&mut self) -> Self::Output {
        // we need to minimize the number of steps towards the middle
        // for the sum of the positions
        let (min_steps, max_steps) = match self.positions.iter().minmax() {
            MinMaxResult::MinMax(min, max) => (min, max),
            _ => panic!("No two values in positions"),
        };
        // let min_steps = self.positions.iter().min().unwrap();
        // let max_steps = self.positions.iter().max().unwrap();
        let mut min_steps_sum = i32::max_value();
        for i in (*min_steps)..=(*max_steps) {
            let mut sum = 0;
            for pos in &self.positions {
                let diff = (pos - i).abs();
                sum += (diff * (diff + 1)) / 2;
            }
            if sum < min_steps_sum {
                min_steps_sum = sum;
            }
        }
        min_steps_sum as isize
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let x = s
            .split(",")
            .map(|x| x.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;
        Ok(Day7 { positions: x })
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
