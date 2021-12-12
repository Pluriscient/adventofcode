use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day12;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day12 {
    connections: Vec<(String, String)>,
}

impl AOCDay for Day {
    const DAY: usize = 12;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        println!("connections : {:?}", self.connections);
        todo!()
    }
    fn part_two(&mut self) -> Self::Output {
        todo!()
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        Ok(Self {
            connections: s
                .lines()
                .map(|line| {
                    let words = line.split("-").collect::<Vec<_>>();
                    (words[0].to_string(), words[1].to_string())
                })
                .collect_vec(),
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
