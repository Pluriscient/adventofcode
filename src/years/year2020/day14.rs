use std::collections::HashMap;
use itertools::Either::*;
use itertools::{Either, Itertools};

fn solve_part_one(instructions: Vec<Either<String, (u64, u64)>>) -> u64 {
    let mut or_mask = 0;
    let mut and_mask = 0;
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for either in instructions { // could be a fold, but as we have 3 vars to keep track of it'll be less clear
        match either {
            Left(mask) => {
                or_mask = create_mask(mask.clone(), false);
                and_mask = create_mask(mask.clone(), true);
            }
            Right((loc, val)) => {
                mem.insert(loc, (val & and_mask) | or_mask);
            }
        }
    }
    mem.values().sum()
}


fn create_mask(mask: String, ones: bool) -> u64 {
    u64::from_str_radix(mask.replace("X", if ones { "1" } else { "0" }).as_str(), 2).unwrap()
}


fn solve_part_two(instructions: Vec<Either<String, (u64, u64)>>) -> u64 {
    let mut mask = String::new();
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for either in instructions {
        match either {
            Left(mask_) => mask = mask_,
            Right((loc, val)) => {
                let addresses = gen_addresses(loc, &mask);
                for address in addresses {
                    mem.insert(address, val);
                }
            }
        }
    }
    mem.values().sum()
}

fn gen_addresses(loc: u64, mask: &str) -> Vec<u64> {
    let one_mask = create_mask(mask.parse().unwrap(), false);
    let x_mask: String = mask.chars().map(|c| if c == 'X' { '0' } else { '1' }).collect();
    let loc = loc | one_mask; // make all 1s 1
    let loc = loc & u64::from_str_radix(x_mask.as_str(), 2).unwrap(); // make all X'es 0
    let mut res = vec![];
    let x_locs = mask.char_indices()
        .filter_map(|(i, c)| if matches!(c, 'X') { Some(mask.len() - i - 1) } else { None }).collect_vec();
    for j in 0..1 << x_locs.len() { // go through all possible subsets
        let mut address = loc;
        for (k, x_loc) in x_locs.iter().enumerate() {
            if (j & (1 << k)) > 0 { // index k is part of this subset
                address |= 1 << x_loc; // turn on this bit
            }
        }
        // println!("address for subset {} is {:010b}", j, address);
        res.push(address);
    }
    res
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::error::Error;
    use super::*;
    use regex::Regex;
    use std::str::FromStr;

    fn parse_input() -> Result<Vec<Either<String, (u64, u64)>>, Box<dyn Error>> {
        let input = read_to_string("inputs/day14.txt")?;
        let lines = input.lines();
        let regex = Regex::new(r"^mem\[(\d+)] = (\d+)\s*$").unwrap();
        let mem_instructions = lines
            .filter(|l| !l.trim().is_empty())
            .map(|l| -> Result<Either<String, (u64, u64)>, Box<dyn Error>> {
                if let Some(caps) = regex.captures(l) {
                    Ok(Right((u64::from_str(caps.get(1).unwrap().as_str())?,
                              u64::from_str_radix(caps.get(2).unwrap().as_str(), 10)?)))
                } else {
                    Ok(Left(l.split("=").skip(1).next().ok_or("broken")?.trim().to_string()))
                }
            })
            .collect();
        mem_instructions
    }

    #[test]
    fn test_part_one() -> Result<(), Box<dyn Error>> {
        let mem_instructions = parse_input()?;
        let solution = solve_part_one(mem_instructions);
        println!("[day14] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        let mem_instructions = parse_input()?;
        let solution = solve_part_two(mem_instructions);
        println!("[day14] solution part 2: {}", solution);
        Ok(())
    }
}