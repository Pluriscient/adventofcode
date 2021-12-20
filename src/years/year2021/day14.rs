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
    steps: usize,
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
    fn step_more_efficiently(&mut self) {
        // println!("starting step");
        let mut templ = self.template.clone();
        // println!("cloned prev");
        let mut mut_index = 0;
        // println!("iterating over ");
        for window in self.template.windows(2) {
            if let Some(pair) = self.pairing.get(&(window[0], window[1])) {
                // println!("inserting");
                templ.insert(mut_index, *pair);
                // println!("done inserting");
                mut_index += 1;
            }
            mut_index += 1;
        }

        self.template = templ;
    }
    fn solve(&self, steps: usize) -> HashMap<(char, char), usize> {
        // first we establish the known pairs in a dictionary of counts
        let mut counter: HashMap<(char, char), usize> = HashMap::new();
        for pair in self.template.windows(2).map(|w| ((w[0], w[1]))) {
            let entry = counter.entry(pair).or_insert(0);
            *entry += 1;
        }

        // each step we add the pairs to the counter
        for _step in 0..steps {
            let last_counter = counter.clone();
            // println!("counter at {}: {:?}", _step, last_counter);
            for (pair, &insertion) in &self.pairing {
                if let Some(&count) = last_counter.get(&pair) {
                    *counter.entry((pair.0, insertion)).or_insert(0) += count;
                    *counter.entry((insertion, pair.1)).or_insert(0) += count;
                    *counter.entry(*pair).or_insert(0) -= count;
                }
            }
        }
        counter
    }
}

impl AOCDay for Day {
    const DAY: usize = 14;
    type Output = usize;

    fn part_one(&mut self) -> Self::Output {
        println!("{:?}", self.pairing);
        println!("{:?}", self.template);
        for _step in 0..self.steps {
            self.step();
        }
        // now we count the number of each char
        let mut counts = HashMap::new();
        for c in self.template.iter() {
            *counts.entry(c).or_insert(0) += 1;
        }
        let mut pair_counts = HashMap::new();
        for pair in self.template.windows(2) {
            *pair_counts.entry(pair).or_insert(0) += 1;
        }
        println!("step 1 pairs: {:?}", pair_counts);
        println!("step 1: {:?}", counts);
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

        let score = *most_common.unwrap() as Self::Output - *least_common.unwrap() as Self::Output;
        // println!("{:?}", score);
        score
    }
    fn part_two(&mut self) -> Self::Output {
        // bruteforce would be like this
        let counter = self.solve(40);
        // to calculate the number of times each character occurrs we use the following knowledge:
        // there is one first character and one last character that will be the first and last letter, but we already know which those are
        // as those will not change in the insertions
        // the rest of the characters will occur twice.
        println!("Step 2 pairs: {:?}", counter);
        let mut char_counter = HashMap::with_capacity(counter.len() * 2);
        for (&pair, occurrences) in counter.iter() {
            let a_entry = char_counter.entry(pair.0).or_insert(0);
            *a_entry += occurrences;
            let a_entry = char_counter.entry(pair.1).or_insert(0);
            *a_entry += occurrences;
        }

        // use the first and last characters
        char_counter.entry(self.template[0]).and_modify(|c| {
            *c += 1;
        });
        char_counter
            .entry(self.template[self.template.len() - 1])
            .and_modify(|c| {
                *c += 1;
            });
        for (_, val) in char_counter.iter_mut() {
            *val /= 2;
        }
        println!("step 2: {:?}", char_counter);
        if let itertools::MinMaxResult::MinMax(min, max) = char_counter.values().minmax() {
            max - min
        } else {
            unreachable!()
        }
        // but how can we calculate the steps more efficiently?
        // current solution is O(1) (lookup) * O(characters in template) * O(step)
        // number of characters in template can double each iteration in the worst case.
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
        Ok(Day14 {
            template,
            pairing,
            steps: 10,
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
