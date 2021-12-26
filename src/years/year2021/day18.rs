use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day18;

#[derive(Clone, Debug, Eq, PartialEq)]
enum SnailFishNumber {
    Number(isize),
    Pair(Box<SnailFishNumber>, Box<SnailFishNumber>),
}

impl SnailFishNumber {
    fn explode(&self) -> Self {
        todo!()
    }

    fn reduce(&self, nest_number: usize) -> SnailFishNumber {
        match self {
            SnailFishNumber::Number(n) if *n > 10 => {
                let (a, b) = (n / 2, (n / 2 + n % 2));
                SnailFishNumber::Pair(
                    Box::new(SnailFishNumber::Number(a)),
                    Box::new(SnailFishNumber::Number(b)),
                ).reduce(nest_number+1)
            }
            SnailFishNumber::Pair(a, b) => {
                if nest_number >= 4 { // or ==?
                    let _a = a.explode();
                }
                let a = a.reduce(nest_number + 1);
                let b = b.reduce(nest_number + 1);

                if a == b {
                    a
                } else {
                    SnailFishNumber::Pair(Box::new(a), Box::new(b))
                }
            }
            _ => self.clone()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day18 {}

impl AOCDay for Day {
    const DAY: usize = 18;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        todo!()
    }
    fn part_two(&mut self) -> Self::Output {
        todo!()
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(_: &str) -> AResult<Self> {
        todo!()
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
