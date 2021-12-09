use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::collections::BTreeSet;
use std::error::Error;
use std::str::FromStr;

type Day = Day9;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day9 {
    heightmap: Vec<Vec<usize>>,
}
impl Day {
    fn get_surrounding_points(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let (n, m) = (self.heightmap.len(), self.heightmap[0].len());
        let (x, y) = point;
        let above = if x > 0 { Some((x - 1, y)) } else { None };
        let below = if x < n - 1 { Some((x + 1, y)) } else { None };
        let left = if y > 0 { Some((x, y - 1)) } else { None };
        let right = if y < m - 1 { Some((x, y + 1)) } else { None };

        vec![above, below, left, right]
            .into_iter()
            .filter_map(|p| p)
            .collect()
    }

    fn find_low_points(&self) -> Vec<(usize, usize)> {
        let (n, m) = (self.heightmap.len(), self.heightmap[0].len());
        let mut low_points = vec![];
        for ii in 0..n {
            for jj in 0..m {
                if self
                    .get_surrounding_points((ii, jj))
                    .into_iter()
                    .all(|y| self.heightmap[ii][jj] < self.heightmap[y.0][y.1])
                {
                    low_points.push((ii, jj));
                }
            }
        }
        low_points
    }

    fn print_basins(&self, basins: &Vec<BTreeSet<(usize, usize)>>) -> () {
        let (n, m) = (self.heightmap.len(), self.heightmap[0].len());
        let mut map = vec![vec!['.'; m]; n];
        for (i, basin) in basins.iter().enumerate() {
            for point in basin {
                map[point.0][point.1] = format!("{}",  (60u8.wrapping_add(i as u8 % 40)) as char)
                    .chars()
                    .next()
                    .unwrap();
            }
        }
        for row in map {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

impl AOCDay for Day {
    const DAY: usize = 9;
    type Output = usize;

    fn part_one(&mut self) -> Self::Output {
        let low_points = self.find_low_points();
        low_points
            .iter()
            .map(|&(ii, jj)| self.heightmap[ii][jj])
            .map(|x| x + 1)
            .sum()
    }
    fn part_two(&mut self) -> Self::Output {
        // we want to find all the basins. There is one basin per low point?
        let low_points = self.find_low_points();

        let boundary_map: Vec<Vec<bool>> = self
            .heightmap
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&x| if x == 9 { true } else { false })
                    .collect()
            })
            .collect();
        let mut basins = vec![];
        for &(ii, jj) in &low_points {
            let mut basin = BTreeSet::new();
            let mut queue = vec![(ii, jj)];
            let mut visited: Vec<(usize, usize)> = vec![];
            while let Some(point) = queue.pop() {
                basin.insert(point);
                visited.push(point);
                let surrounding_points = self.get_surrounding_points(point);
                for p in surrounding_points.iter().filter(|p| !visited.contains(p)) {
                    if !boundary_map[p.0][p.1] {
                        queue.push(*p);
                    }
                }
            }
            basins.push(basin);
        }
        self.print_basins(&basins);

        basins
            .iter()
            .map(|x| x.len())
            .sorted()
            .rev()
            .take(3)
            .fold(1, |acc, x| acc * x)
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let heightmap = s
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|n| n.to_string().parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        Ok(Self { heightmap })
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
