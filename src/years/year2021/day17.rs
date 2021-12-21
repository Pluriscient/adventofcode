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
    fn simulate_other(&self, x_speed: isize, y_speed: isize) {
        // y(t) = (a-t) t + y(t-1)
        // y(t) = 1/6 (3a(t^2+t+2) - t(t+1)(2t+1))
        // x(t) = (b - max(b, t)) t + x(t-1)
        // x(t) = b + 1/6 ((3b-2t-1)t(t+1)-(3b-2b-1)b(b+1))
        // we can reverse the formulae to get the other speed
        // given a particular y_initial (a), we can get the x that would allow for an intersection
        // then we can see if that would happen at a full step or not
        let t = self.count_ysteps_till_intersection(10).unwrap() as isize;
        // let t = t as f64;
        // let b = (-18 * t
        //     ^ 3 - 27 * t
        //     ^ 2 + (3 as f64).sqrt()
        //         * sqrt(-729 * t ^ 4 - 1458 * t ^ 3 - 2493 * t ^ 2 - 1764 * t - 1372)
        //         - 9 * t)
        //     ^ (1 / 3) / (2 ^ (1 / 3) * 3 ^ (2 / 3))
        //         - ((2 / 3) ^ (1 / 3) * (-9 * t ^ 2 - 9 * t - 21))
        //             / (3 * (-18 * t
        //                 ^ 3 - 27 * t
        //                 ^ 2 + sqrt(3)
        //                     * sqrt(-729 * t ^ 4 - 1458 * t ^ 3 - 2493 * t ^ 2 - 1764 * t - 1372)
        //                     - 9 * t)
        //                 ^ (1 / 3));
        todo!()
    }

    fn count_ysteps_till_intersection(&self, y_speed: isize) -> Option<usize> {
        let mut y = 0;
        let mut steps = 0;
        while y < self.y_bounds.1 {
            y += y_speed;
            steps += 1;
            if y >= self.y_bounds.0 && y <= self.y_bounds.1 {
                return Some(steps);
            }
        }
        return None;
    }

    fn simulate(&self, x_speed: isize, y_speed: isize) -> Option<isize> {
        // simulates the movement of the sub till it passes the square
        let mut cur_x_speed = x_speed;
        let mut cur_y_speed = y_speed;
        let mut x = 0;
        let mut y = 0;
        let mut y_max = 0;
        // let mut step = 1;
        loop {
            x += cur_x_speed;
            y += cur_y_speed;
            // println!("at step {}: x={}, y={}", step, x, y);
            y_max = std::cmp::max(y_max, y);

            if x >= self.x_bounds.0
                && x <= self.x_bounds.1
                && y <= self.y_bounds.0
                && y >= self.y_bounds.1
            {
                // println!("Found square at step {}", step);
                return Some(y_max);
            } else if x > self.x_bounds.1 || y < self.y_bounds.1 {
                // println!("Stepped out of bounds at step {}", step);
                // println!("{:?}", (x, y));
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
            // step += 1;
        }
    }
}

impl AOCDay for Day {
    const DAY: usize = 17;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        println!("{:?}", self);
        // we know that our y  value should always be positive
        // we want to maximize the y value that we can have
        // we can do this by simulating the movement of the sub
        // and seeing when it reaches the square
        // or just bruteforce cus it isn't that many combinations
        let mut max_y = 0;
        for y_speed in 1..=self.x_bounds.0 {
            for x_speed in 1..=(self.x_bounds.0 / 2) {
                // let (x_speed, y_speed) = (7, 2);
                println!("Trying {} {}", x_speed, y_speed);
                let result = self.simulate(x_speed, y_speed);
                if let Some(y) = result {
                    max_y = std::cmp::max(max_y, y);
                }
                // break
            }
            // break
        }
        max_y
    }
    fn part_two(&mut self) -> Self::Output {
        let mut velos = vec![];
        for y_speed in (self.y_bounds.1)..=self.x_bounds.1 {
            for x_speed in 1..=(self.x_bounds.1) {
                // let (x_speed, y_speed) = (7, 2);
                // println!("Trying {} {}", x_speed, y_speed);
                let result = self.simulate(x_speed, y_speed);
                if let Some(_) = result {
                    // println!("success");
                    velos.push((x_speed, y_speed));
                }
                // break
            }
            // break
        }
        // velos.sort_unstable();
        // println!("{:?}", velos);
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
