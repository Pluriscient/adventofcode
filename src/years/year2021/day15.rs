use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day15;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day15 {
    grid: Vec<Vec<usize>>,
}

impl Day {
    fn print_grid(grid: &[Vec<usize>]) {
        for row in grid {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
        println!();
    }
}

impl AOCDay for Day {
    const DAY: usize = 15;
    type Output = usize;

    fn part_one(&mut self) -> Self::Output {
        // println!("{:?}", self);
        // this is a problem we can use dynamic programming for
        // only unfortunate part is its dimensionality, I figure we shoud use a 3d memory array
        // let's first try with 2D
        // mem[n, n] = final point if all diagonal?
        // mem[0, 0] = initial position, can choose either left or right
        // we first create the memory matrix before then finding the best path through it
        let n = self.grid.len();
        let mut mem = vec![vec![0; n]; n];
        // first we fill the all right and all down
        for ii in 1..n {
            mem[ii][0] = mem[ii - 1][0] + self.grid[ii][0];
            mem[0][ii] = mem[0][ii - 1] + self.grid[0][ii];
        }
        for ii in 1..n {
            for jj in 1..n {
                let from_up = mem[ii - 1][jj] + self.grid[ii][jj];
                let from_left = mem[ii][jj - 1] + self.grid[ii][jj];
                mem[ii][jj] = from_up.min(from_left);
            }
        }

        // println!("{:?}", mem);

        mem[n - 1][n - 1]
    }
    fn part_two(&mut self) -> Self::Output {
        // we modify the grid so that it's repeated
        // first in the x direction

        let n = self.grid.len();
        let mut new_grid = vec![vec![0; n * 5]; n * 5];
        // rows
        for tile in 0..5 {
            for ii in 0..n {
                for jj in 0..n {
                    // wrap around from 9 to 1
                    // f(8) = 9
                    // f(9) = 1
                    new_grid[n * tile + ii][jj] = ((self.grid[ii][jj] + tile - 1) % 9) + 1
                }
            }
        }
        // cols
        for tile in 1..5 {
            for ii in 0..(5 * n) {
                for jj in 0..n {
                    new_grid[ii][n * tile + jj] = ((new_grid[ii][jj] + tile - 1) % 9) + 1
                }
            }
        }
        Self::print_grid(&new_grid);
        self.grid = new_grid;
        self.part_one()
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        Ok(Self {
            grid: s
                .lines()
                .map(|line| {
                    line.trim()
                        .chars()
                        .map(|c| c.to_string().parse::<usize>())
                        .collect::<Result<Vec<_>, _>>()
                        .unwrap()
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
