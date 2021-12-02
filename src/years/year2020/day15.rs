// use itertools::Itertools;
// use prgrs::{Prgrs};
use std::collections::HashMap;
use indicatif::ProgressIterator;
use fnv::{FnvHashMap, FnvBuildHasher};
// use itertools::Itertools;

fn solve_part_one(starting_numbers: Vec<u64>) -> usize {
    let turns = 2020;
    // let mut mem = Vec::with_capacity(turns);
    // mem.extend(starting_numbers);
    // while mem.len() <= turns {
    //     let last_spoken = mem.last().unwrap().clone();
    //     if let Some(pos) = find_last(&mem[..mem.len() - 1], last_spoken) {
    //         mem.push(mem.len() - 1 - pos);
    //     } else {
    //         mem.push(0);
    //     }
    // }
    // mem[turns - 1]
    turns
}

fn find_last(mem: &[usize], x: usize) -> Option<usize> {
    for i in (0..mem.len()).rev() {
        if mem[i] == x {
            return Some(i);
        }
    }
    None
}


fn solve_part_two(starting_numbers: Vec<u64>) -> usize {
    let turns = 30000000;
    let mut memory = Memory::new(&starting_numbers);
    let (turn, last) = starting_numbers.iter().fold((0, 0), |(turn, _last), num| {
        (turn + 1, *num)
    });

    let mut last = last;
    for turn in turn..turns {
        let turn = turn + 1;
        let new_number = match memory.recall(&last) {
            (0, 0) => 0,
            (i_recent, 0) if i_recent != 0 => 0,
            (ir, ipr) if ir != 0 && ipr != 0 => (ir - ipr) as u64,
            _ => unreachable!()
        };
        memory.speak(turn, new_number);
        last = new_number;
        // println!("turn {} we have {}", turn, last);
    }
    last as usize
}

#[derive(Clone)]
struct Memory {
    number_to_turns: FnvHashMap<u64, (usize, usize)>,
    zero_to_turns: (usize, usize),
}

impl Memory {
    fn new(starting_numbers: &[u64]) -> Self {
        let mut number_to_turns = FnvHashMap::with_hasher(FnvBuildHasher::default());
        for (i, num) in starting_numbers.iter().enumerate() {
            // let num = num.parse::<u64>().unwrap();
            number_to_turns.insert(*num, (i + 1, 0));
        }
        let zero_to_turns: (usize, usize) = *number_to_turns.get(&0).unwrap_or(&(0, 0));

        Self {
            number_to_turns,
            zero_to_turns,
        }
    }
    fn speak(&mut self, turn: usize, number: u64) {
        if number == 0 {
            let i_rec = self.zero_to_turns.0;
            self.zero_to_turns.1 = i_rec;
            self.zero_to_turns.0 = turn;
        } else {
            let new_record = match self.number_to_turns.get(&number) {
                None => (turn, 0),
                Some((0, _)) => unreachable!("Impossible record"),
                Some((i_r, _)) => (turn, *i_r)
            };
            self.number_to_turns.insert(number, new_record);
        }
    }

    fn recall(&self, number: &u64) -> (usize, usize) {
        if *number == 0 {
            self.zero_to_turns
        } else {
            if let Some(prev) = self.number_to_turns.get(number) {
                (prev.0, prev.1)
            } else {
                (0, 0)
            }
        }
    }
}


fn solve_optimized(starting_numbers: Vec<usize>) -> usize {
    // keeps track of last positions with a map
    let turns = 30000000;
    let mut map: HashMap<usize, usize> = HashMap::with_capacity(10000);
    for (i, s) in starting_numbers[..starting_numbers.len() - 1].iter().enumerate() {
        map.insert(*s, i);
    }
    // println!("map: {:?}", map);
    let mut most_recent: usize = starting_numbers[starting_numbers.len() - 1];
    let mut next;
    for i in ((starting_numbers.len())..turns).progress() {
        // println!("at turn {}, most recent was {}", i+2, most_recent);
        // let next;
        if let Some(before) = map.get_mut(&most_recent) {
            next = i - 1 - *before;
        } else {
            // println!("{} is new! Adding", most_recent);
            next = 0;
            map.insert(most_recent, 0);
        }
        // println!("at turn {} , most recent is {}, so speaking {}", i + 1, most_recent, next);
        most_recent = next;
    }
    most_recent
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::error::Error;
    use super::*;
    // use itertools::Itertools;
    use std::str::FromStr;

    fn parse_input() -> Result<Vec<u64>, Box<dyn Error>> {
        let input = read_to_string("../../../inputs/year2020/day15.txt")?;
        let res = input.split(",").map(|l| u64::from_str(l)).collect::<Result<Vec<_>, _>>()?;
        Ok(res)
    }

    #[test]
    fn test_part_one() -> Result<(), Box<dyn Error>> {
        let starting_numbers = parse_input()?;
        let solution = solve_part_one(starting_numbers);
        println!("[day15] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        let starting_numbers = parse_input()?;
        let solution = solve_part_two(starting_numbers);
        println!("[day15] solution part 2: {}", solution);
        Ok(())
    }
}