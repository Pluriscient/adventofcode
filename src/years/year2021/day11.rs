use super::util::AResult;
use super::AOCDay;
use indicatif::ProgressIterator;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day11;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day11 {
    /// The grid of dumbo octopuses
    grid: Vec<Vec<(isize, bool)>>,
    /// The size of the grid
    /// (i.e. the number of rows and columns)
    size: usize,
}

impl Day {
    fn is_valid_point(&self, point: (isize, isize)) -> bool {
        (point.0 < self.size as isize && point.0 >= 0)
            && (point.1 < self.size as isize && point.1 >= 0)
    }

    fn adjacent_points(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
        let mut points = vec![];
        let (x, y) = p;
        // horizontal
        if self.is_valid_point((x as isize - 1, y as isize)) {
            points.push(((x as isize - 1) as usize, y));
        }
        if self.is_valid_point((x as isize + 1, y as isize)) {
            points.push(((x as isize + 1) as usize, y));
        }
        // vertical
        if self.is_valid_point((x as isize, y as isize - 1)) {
            points.push((x, (y as isize - 1) as usize));
        }
        if self.is_valid_point((x as isize, y as isize + 1)) {
            points.push((x, (y as isize + 1) as usize));
        }
        // diagonal
        if self.is_valid_point((x as isize - 1, y as isize - 1)) {
            points.push(((x as isize - 1) as usize, (y as isize - 1) as usize));
        }
        if self.is_valid_point((x as isize + 1, y as isize + 1)) {
            points.push(((x as isize + 1) as usize, (y as isize + 1) as usize));
        }
        if self.is_valid_point((x as isize - 1, y as isize + 1)) {
            points.push(((x as isize - 1) as usize, (y as isize + 1) as usize));
        }
        if self.is_valid_point((x as isize + 1, y as isize - 1)) {
            points.push(((x as isize + 1) as usize, (y as isize - 1) as usize));
        }

        points
    }

    fn print_grid(&self) {
        for row in &self.grid {
            for (i, (p, _)) in row.iter().enumerate() {
                if i == 0 {
                    print!("{:>1}", p);
                } else {
                    print!("{:>1}", p);
                }
            }
            println!();
        }
        println!();
    }

    fn step(&mut self) -> usize {
        // first we increase every point by 1 and make sure that the flashing hasn't happened.
        for ii in 0..self.size {
            for jj in 0..self.size {
                self.grid[ii][jj].0 += 1;
                assert!(!self.grid[ii][jj].1);
            }
        }
        // While any point can flash but hasn't done so yet, we continue looping
        while self.grid.iter().any(|row| {
            row.iter()
                .any(|&(charge, flashed)| (!flashed && charge > 9))
        }) {
            // As a sanity check, we verify the number of flashes

            for ii in 0..self.size {
                for jj in 0..self.size {
                    let point = self.grid[ii][jj];
                    if !point.1 && point.0 > 9 {
                        // flash this octopus
                        self.grid[ii][jj].1 = true;
                        // println!("Flashing point: {:?}", (ii, jj));
                        for p in self.adjacent_points((ii, jj)) {
                            if !self.grid[p.0][p.1].1 {
                                self.grid[p.0][p.1].0 += 1;
                            }
                        }
                    }
                }
            }
        }
        // We count the number of cells that have flashed and reset them.
        let mut flashes = 0;
        for ii in 0..self.size {
            for jj in 0..self.size {
                if self.grid[ii][jj].1 {
                    self.grid[ii][jj].0 = 0;
                    flashes += 1;
                    self.grid[ii][jj].1 = false;
                } else {
                    assert!(self.grid[ii][jj].0 <= 9);
                }
            }
        }
        flashes
    }
}

impl AOCDay for Day {
    const DAY: usize = 11;
    type Output = usize;

    fn part_one(&mut self) -> Self::Output {
        // println!("{:?}", self.grid);
        (0..100)
            .map(|s| {
                println!("Step {}", s);
                self.print_grid();
                self.step()
            })
            .sum()
    }
    fn part_two(&mut self) -> Self::Output {
        let target = self.size * self.size;
        (1..1000)
            .progress()
            .find(|_| self.step() == target)
            .expect("could not find the target")
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
                        .map(|x| (x.to_string().parse::<isize>().unwrap(), false))
                        .collect()
                })
                .collect(),
            size: s.lines().count(),
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
