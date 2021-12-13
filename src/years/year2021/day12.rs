use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day12;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cave {
    Big(String),
    Small(String),
    Start,
    End,
}

impl FromStr for Cave {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "start" {
            Ok(Cave::Start)
        } else if s == "end" {
            Ok(Cave::End)
        } else if s.chars().next().unwrap().is_uppercase() {
            Ok(Cave::Big(s.to_string()))
        } else {
            Ok(Cave::Small(s.to_string()))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day12 {
    connections: Vec<(Cave, Cave)>,
}
impl Day {
    fn nodes(&self) -> Vec<Cave> {
        let nodes = self
            .connections
            .iter()
            .flat_map(|(a, b)| vec![a.clone(), b.clone()])
            .unique()
            .collect::<Vec<_>>();
        nodes
    }
    fn connections(&self, cave: &Cave) -> Vec<Cave> {
        self.connections
            .iter()
            .filter(|(a, b)| a == cave || b == cave)
            .map(|(a, b)| if a == cave { b.clone() } else { a.clone() })
            .collect::<Vec<_>>()
    }

    fn count_path(&self, cur_path: Vec<Cave>) -> usize {
        let last = cur_path.last().unwrap();
        if last == &Cave::End {
            return 1;
        }
        let mut count = 0;
        for next in self.connections(last) {
            match &next {
                Cave::Big(_) => (),
                Cave::End => (),
                Cave::Small(_) => {
                    if cur_path.contains(&next) {
                        continue;
                    }
                }
                Cave::Start => continue,
            }
            let mut new_path = cur_path.clone();
            new_path.push(next.clone());
            count += self.count_path(new_path);
        }
        count
    }

    fn count_path_2(&self, cur_path: Vec<Cave>, twice: bool) -> usize {
        let last = cur_path.last().unwrap();
        if last == &Cave::End {
            return 1;
        }
        let mut count = 0;
        for next in self.connections(last) {
            let mut twicer = twice;

            if matches!(next, Cave::Start) {
                continue;
            }
            if matches!(&next, Cave::Small(_)) {
                if twice {
                    if cur_path.contains(&next) {
                        continue;
                    }
                } else {
                    let occurrences = cur_path.iter().filter(|&x| x == &next).count();
                    if occurrences == 1 {
                        twicer = true;
                    }
                }
            }
            let mut new_path = cur_path.clone();
            new_path.push(next.clone());
            // println!("{:?}", new_path);
            count += self.count_path_2(new_path, twicer);
        }
        count
    }
}
impl AOCDay for Day {
    const DAY: usize = 12;
    type Output = usize;

    fn part_one(&mut self) -> Self::Output {
        self.count_path(vec![Cave::Start])
    }
    fn part_two(&mut self) -> Self::Output {
        self.count_path_2(vec![Cave::Start], false)
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        Ok(Self {
            connections: s
                .lines()
                .map(|line| {
                    let words = line.split("-").collect::<Vec<_>>();
                    (
                        Cave::from_str(words[0]).unwrap(),
                        Cave::from_str(words[1]).unwrap(),
                    )
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
