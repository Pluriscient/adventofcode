use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day19;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct ScannerReport {
    id: usize,
    beacon_positions: Vec<Position>,
    position: Option<Position>,
}

impl FromStr for ScannerReport {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        //   let id = parts.next().unwrap().parse()?;
        // let positions = parts.next().unwrap().split(", ").map(|s| {
        //     let mut parts = s.split(", ");
        //     let x = parts.next().unwrap().parse()?;
        //     let y = parts.next().unwrap().parse()?;
        //     let z = parts.next().unwrap().parse()?;
        //     Ok(Position { x, y, z })
        // }).collect::<AResult<Vec<_>>>()?;
        todo!()
        // Ok(ScannerReport {
        //     id,
        //     beacon_positions: positions,
        //     position: None
        // })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day19 {
    reports: Vec<ScannerReport>,
}

impl AOCDay for Day {
    const DAY: usize = 19;
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
    fn from_str(s: &str) -> AResult<Self> {
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
