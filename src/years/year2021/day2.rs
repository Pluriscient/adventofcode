use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day2;
#[derive(Clone, Debug, Eq, PartialEq)]
struct Day2 {
    steps: Vec<Step>,
}

impl AOCDay for Day {
    const DAY: usize = 2;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        let final_position = self
            .steps
            .iter()
            .fold((0, 0), |pos, step| step.apply_step_a(pos));
        final_position.0 * final_position.1
    }
    fn part_two(&mut self) -> Self::Output {
        let final_position = self
            .steps
            .iter()
            .fold((0, 0, 0), |pos, step| step.apply_step_b(pos));
        final_position.0 * final_position.1
    }
}
impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let steps: Vec<Step> = s
            .lines()
            .map(|line| line.parse::<Step>().unwrap())
            .collect();
        Ok(Day2 { steps })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Direction {
    Forward,
    Up,
    Down,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Step {
    size: isize,
    direction: Direction,
}
impl Step {
    fn apply_step_a(&self, position: (isize, isize)) -> (isize, isize) {
        match self.direction {
            Direction::Forward => (position.0 + self.size, position.1),
            Direction::Up => (position.0, position.1 - self.size),
            Direction::Down => (position.0, position.1 + self.size),
        }
    }
    fn apply_step_b(&self, position: (isize, isize, isize)) -> (isize, isize, isize) {
        match self.direction {
            Direction::Forward => (
                position.0 + self.size,
                position.1 + self.size * position.2,
                position.2,
            ),
            Direction::Up => (position.0, position.1, position.2 - self.size),
            Direction::Down => (position.0, position.1, position.2 + self.size),
        }
    }
}

impl FromStr for Step {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> AResult<Self> {
        let words: Vec<&str> = s.split_whitespace().collect_vec();
        let size = words[1].parse::<isize>()?;
        let direction = match words[0] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            other => return Err(format!("Invalid direction {}", other).into()),
        };
        Ok(Self { size, direction })
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
