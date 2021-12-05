use super::util::AResult;
use super::AOCDay;
use std::error::Error;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
struct DayX {}

impl AOCDay for DayX {
    const DAY: usize = 0;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        todo!()
    }
    fn part_two(&mut self) -> Self::Output {
        todo!()
    }
}
impl FromStr for DayX {
    type Err = Box<dyn Error>;
    fn from_str(_: &str) -> AResult<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{test_day_part_one, test_day_part_two};
    use super::*;
    type Day = DayX;
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
