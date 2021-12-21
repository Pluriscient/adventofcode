use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use regex::Regex;
use std::error::Error;
use std::str::FromStr;

type Day = Day17;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day17 {
    x_bounds: (isize, isize),
    y_bounds: (isize, isize),
}
impl Day {
    fn simulate(&self, x_speed: isize, y_speed: isize) -> Option<isize> {
        // simulates the movement of the sub till it passes the square
        let mut cur_x_speed = x_speed;
        let mut cur_y_speed = y_speed;
        let mut x = 0;
        let mut y = 0;
        let mut y_max = 0;
        loop {
            x += cur_x_speed;
            y += cur_y_speed;
            y_max = std::cmp::max(y_max, y);

            if x >= self.x_bounds.0
                && x <= self.x_bounds.1
                && y <= self.y_bounds.0
                && y >= self.y_bounds.1
            {
                return Some(y_max);
            } else if x > self.x_bounds.1 || y < self.y_bounds.1 {
                return None;
            }
            // x speed is slowed till it reaches zero
            if cur_x_speed > 0 {
                cur_x_speed -= 1;
            } else if cur_x_speed < 0 {
                cur_x_speed += 1;
            }
            // y speed keeps decreasing
            cur_y_speed -= 1;
        }
    }
}

impl AOCDay for Day {
    const DAY: usize = 17;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        // we know that our y  value should always be positive
        // we want to maximize the y value that we can have
        // we can do this by simulating the movement of the sub
        // and seeing when it reaches the square
        // or just bruteforce cus it isn't that many combinations
        let mut max_y = 0;
        for y_speed in 1..=self.x_bounds.0 {
            for x_speed in 1..=(self.x_bounds.0 / 2) {
                let result = self.simulate(x_speed, y_speed);
                if let Some(y) = result {
                    max_y = std::cmp::max(max_y, y);
                }
            }
        }
        max_y
    }
    fn part_two(&mut self) -> Self::Output {
        let mut velos = vec![];
        for y_speed in (self.y_bounds.1)..=self.x_bounds.1 {
            for x_speed in 1..=(self.x_bounds.1) {
                let result = self.simulate(x_speed, y_speed);
                if let Some(_) = result {
                    velos.push((x_speed, y_speed));
                }
            }
        }
        velos.len() as isize
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let regex = Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();
        let caps = regex.captures(s).unwrap();
        let caps = caps
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        println!("{:?}", caps);
        let x_bounds = (caps[0], caps[1]);
        let y_bounds = (caps[3], caps[2]);
        Ok(Self { x_bounds, y_bounds })
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
