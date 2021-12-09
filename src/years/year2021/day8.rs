use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};
use std::error::Error;
use std::str::FromStr;
extern crate single;
use single::Single;

type CharSet = BTreeSet<char>;
type Day = Day8;

lazy_static! {
    static ref SEGMENTS: HashMap<u8, CharSet> = {
        let mut map = std::collections::HashMap::new();
        map.insert(0, ['a', 'b', 'c', 'e', 'f', 'g'].iter().cloned().collect());
        map.insert(1, ['c', 'f'].iter().cloned().collect());
        map.insert(2, ['a', 'c', 'd', 'e', 'g'].iter().cloned().collect());
        map.insert(3, ['a', 'c', 'd', 'f', 'g'].iter().cloned().collect());
        map.insert(4, ['b', 'c', 'd', 'f'].iter().cloned().collect());
        map.insert(5, ['a', 'b', 'd', 'f', 'g'].iter().cloned().collect());
        map.insert(6, ['a', 'b', 'd', 'e', 'f', 'g'].iter().cloned().collect());
        map.insert(7, ['a', 'c', 'f'].iter().cloned().collect());
        map.insert(
            8,
            ['a', 'b', 'c', 'd', 'e', 'f', 'g']
                .iter()
                .cloned()
                .collect(),
        );
        map.insert(9, ['a', 'b', 'c', 'd', 'f', 'g'].iter().cloned().collect());
        map
    };
}
#[derive(Debug, Clone, PartialEq)]
struct Entry {
    patterns: Vec<CharSet>, // patterns for each digit
    output: Vec<CharSet>,   // output for each digit
}
impl FromStr for Entry {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace().into_iter();
        let patterns: Vec<CharSet> = split
            .take_while_ref(|&x| x != "|")
            .map(|x| x.chars().collect())
            .collect();
        split.next();
        let output: Vec<CharSet> = split.map(|x| x.chars().collect()).collect();
        if !matches!((patterns.len(), output.len()), (10, 4)) {
            return Err("Invalid input".into());
        }
        Ok(Self { patterns, output })
    }
}

impl Entry {
    fn derive_mapping(&self) -> Vec<CharSet> {
        // the easy ones allow us to find a few locations

        let seg7 = self.patterns.iter().find(|x| x.len() == 3).unwrap();
        let seg1 = self.patterns.iter().find(|x| x.len() == 2).unwrap();
        let seg4 = self.patterns.iter().find(|x| x.len() == 4).unwrap();
        let seg8 = self.patterns.iter().find(|x| x.len() == 7).unwrap();
        // the segs that have a length of 6 are [0,6,9]
        let seg069 = self
            .patterns
            .iter()
            .filter(|x| x.len() == 6)
            .collect::<BTreeSet<_>>();
        // the segs that have a length of 5 are [2,5,3]
        let seg235 = self
            .patterns
            .iter()
            .filter(|x| x.len() == 5)
            .collect::<BTreeSet<_>>();

        let seteg = &(seg8 - seg4) - seg7;
        // let seteg = seteg - seg7;

        let seg0 = seg069
            .iter()
            .find(|&&x| ((&seteg - x).len() == 0) && (seg1 - x).len() == 0)
            .unwrap();
        let seg69 = seg069
            .iter()
            .filter(|&x| x != seg0)
            .collect::<BTreeSet<_>>();

        let seg9 = *seg69
            .iter()
            .filter(|&&x| (seg1 - x).len() == 0)
            .single()
            .unwrap();
        let seg6 = *seg69.iter().filter(|&&x| x != seg9).single().unwrap();
        let seg2 = *seg235
            .iter()
            .filter(|&&x| (x - seg9).len() == 1)
            .single()
            .unwrap();
        let seg35 = seg235
            .iter()
            .filter(|&&x| x != seg2)
            .collect::<BTreeSet<_>>();
        let seg3 = *seg35
            .iter()
            .filter(|&&x| (*x - *seg6).len() == 1)
            .single()
            .unwrap();
        let seg5 = *seg35.iter().filter(|&&x| x != seg3).single().unwrap();
        [
            seg0.clone().to_owned(),
            seg1.clone(),
            seg2.clone().to_owned(),
            seg3.clone().to_owned(),
            seg4.clone(),
            seg5.clone().to_owned(),
            seg6.clone().to_owned(),
            seg7.clone(),
            seg8.clone(),
            seg9.clone().to_owned(),
        ]
        .iter()
        .cloned()
        .collect::<Vec<CharSet>>()
    }

    fn translate(&self) -> usize {
        let mapping = self.derive_mapping();
        assert_eq!(mapping.len(), mapping.iter().unique().count());
        let digits = self
            .output
            .iter()
            .map(|x| {
                mapping
                    .iter()
                    .enumerate()
                    .find_map(|(i, y)| if x == y { Some(i) } else { None })
            })
            .collect::<Vec<_>>();
        let number = digits.iter().fold(0, |acc, x| acc * 10 + x.unwrap());
        number
    }

    fn count_unique_digits(&self) -> usize {
        // we want to create a mapping from the new to the old digits
        // for now we just want to find those digits that are unique
        self.output
            .iter()
            .map(|x| x.len())
            .map(|x| match x {
                2 => 1,
                4 => 4,
                3 => 7,
                7 => 8,
                _ => 0,
            })
            .filter(|&x| x > 0)
            .count()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Day8 {
    entries: Vec<Entry>,
}

impl AOCDay for Day {
    const DAY: usize = 8;
    type Output = usize;

    fn part_one(&mut self) -> Self::Output {
        self.entries.iter().map(|x| x.count_unique_digits()).sum()
    }
    fn part_two(&mut self) -> Self::Output {
        self.entries.iter().map(|x| x.translate()).sum()
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let entries = s
            .lines()
            .map(|x| x.parse::<Entry>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { entries })
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
        //1076219 too high
    }
}
