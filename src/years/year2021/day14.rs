use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

type Day = Day14;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day14 {
    template: Vec<char>,
    pairing: HashMap<(char, char), char>,
    steps: usize
}

impl Day14 {
    fn step(&mut self) {
        let mut new_template = Vec::new();
        for (i, c) in self.template.iter().enumerate() {
            if i > 0 {
                let left = self.template[i - 1];
                let right = self.template[i];
                if let Some(pair) = self.pairing.get(&(left, right)) {
                    new_template.push(*pair);
                }
            }
            new_template.push(*c);
        }
        self.template = new_template;
    }
}

impl AOCDay for Day {
    const DAY: usize = 14;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        println!("{:?}", self.pairing);
        println!("{:?}", self.template);
        for _ in 0..self.steps {
            self.step();
            // println!("{:?}", self.template);
        }
        // now we count the number of each char
        let mut counts = HashMap::new();
        for c in self.template.iter() {
            *counts.entry(c).or_insert(0) += 1;
        }
        println!("{:?}", counts);
        // our return score is the most common  - least common
        let mut most_common = None;
        let mut least_common = None;
        for (_c, count) in counts.iter() {
            if most_common.is_none() || count > most_common.unwrap() {
                most_common = Some(count);
            }
            if least_common.is_none() || count < least_common.unwrap() {
                least_common = Some(count);
            }
        }

        let score = *most_common.unwrap() as isize - *least_common.unwrap() as isize;
        println!("{:?}", score);
        score
    }
    fn part_two(&mut self) -> Self::Output {
        self.steps = 40;
        self.part_one()
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let template = s.lines().next().unwrap().trim().chars().collect::<Vec<_>>();
        let pairing = s
            .lines()
            .skip(2)
            .map(|line| {
                let mut iter = line.trim().split(" -> ");
                let left = iter.next().unwrap().chars().collect_vec();
                let right = iter.next().unwrap().chars().next().unwrap();
                ((left[0], left[1]), right)
            })
            .collect::<HashMap<_, _>>();
        Ok(Day14 { template, pairing, steps: 10 })
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
