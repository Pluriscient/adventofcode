use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day25;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum CucumberState {
    Empty,
    South,
    East,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day25 {
    width: usize,
    height: usize,
    data: Box<[CucumberState]>,
}

impl Day {
    fn step(&mut self) -> usize {
        // first the cucumbers oriented to the east move
        // then the south ones
        let mut movements = 0;
        let mut next_data = self.data.clone();
        for ii in 0..self.height {
            for jj in 0..self.width {
                let state = self.data[ii * self.width + jj];
                if state == CucumberState::East {
                    let next_jj = if jj + 1 < self.width { jj + 1 } else { 0 };
                    if self.data[ii * self.width + next_jj] == CucumberState::Empty {
                        next_data[ii * self.width + next_jj] = CucumberState::East;
                        next_data[ii * self.width + jj] = CucumberState::Empty;
                        movements += 1;
                    }
                }
            }
        }
        self.data = next_data.clone();
        for ii in 0..self.height {
            for jj in 0..self.width {
                let state = self.data[ii * self.width + jj];
                if state == CucumberState::South {
                    let next_ii = if ii + 1 < self.height { ii + 1 } else { 0 };
                    if self.data[next_ii * self.width + jj] == CucumberState::Empty {
                        next_data[next_ii * self.width + jj] = CucumberState::South;
                        next_data[ii * self.width + jj] = CucumberState::Empty;
                        movements += 1;
                    }
                }
            }
        }
        self.data = next_data;
        movements
    }
}

impl AOCDay for Day {
    const DAY: usize = 25;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        (1..).find(|_| self.step() == 0).unwrap()
    }
    fn part_two(&mut self) -> Self::Output {
        todo!()
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let lines: Vec<_> = s.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let mut data = Vec::with_capacity(height * width);
        for line in lines {
            for c in line.chars() {
                data.push(match c {
                    'v' => CucumberState::South,
                    '.' => CucumberState::Empty,
                    '>' => CucumberState::East,
                    _ => return Err(From::from(format!("Invalid character: {}", c))),
                });
            }
        }
        Ok(Self {
            width,
            height,
            data: data.into_boxed_slice(),
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
