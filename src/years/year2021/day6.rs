use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day6;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day6 {
    lanternfish: Vec<isize>,
}
impl Day6 {
    fn f(x: u128, d: u128) -> u128 {
        if x >= d {
            1
        } else {
            Self::f(0, d.saturating_sub(x + 1 + 8)) + Self::f(0, d.saturating_sub(x + 1 + 6))
        }
    }

    fn mem_lanterns(&self, d: usize) -> u128 {
        let mut mem = vec![0; d as usize];
        mem[0] = 1;
        for i in 1..d {
            mem[i] = mem[i.saturating_sub(9)] + mem[i.saturating_sub(7)];
        }
        self.lanternfish.iter().map(|&x| mem[d - x as usize]).sum()
    }
}

const DAYS_TO_SIMULATE: usize = 80;
impl AOCDay for Day {
    const DAY: usize = 6;
    type Output = u128;

    fn part_one(&mut self) -> Self::Output {
        // each lanternfish can be simulated separately
        for _ in 0..DAYS_TO_SIMULATE {
            let mut new_fish = 0;
            self.lanternfish.iter_mut().for_each(|lanternfish| {
                *lanternfish -= 1;
                if *lanternfish < 0 {
                    *lanternfish = 6;
                    new_fish += 1;
                }
            });
            self.lanternfish.append(&mut vec![8; new_fish]);
        }
        self.lanternfish.len() as u128
    }
    fn part_two(&mut self) -> Self::Output {
        self.mem_lanterns(256)
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let lanternfish = s
            .split(",")
            .map(|x| x.trim().parse::<isize>())
            .collect::<Result<Vec<isize>, _>>()?;
        Ok(Day6 { lanternfish })
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
