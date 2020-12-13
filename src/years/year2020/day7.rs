use std::collections::{HashMap, HashSet};
use regex::Regex;
use std::str::FromStr;
use itertools::Itertools;


fn solve_part_one(bagrules: Vec<BagRule>) -> usize {
    // we calculate the amount of bags that can contain a shiny gold bag.
    // sounds like a DP problem
    // let's first trim
    let mut results = HashSet::new();

    results.insert("shiny gold".to_string());
    loop {
        let found = bagrules.iter()
            .filter(|b| !results.contains(&b.name) &&
                b.children.iter().any(|(k, _)| results.contains(k)))
            .map(|b| b.name.to_string())
            .collect_vec();
        match found.len() {
            0 => break,
            _ => results.extend(found)
        }
    }
    results.len() - 1 // remove the shiny gold bag again
}

fn solve_part_two(bagrules: Vec<BagRule>) -> usize {
    let mut count = 0;
    let mapping: HashMap<String, &BagRule> = bagrules.iter().map(|b| (b.name.to_string(), b)).collect();
    let gold_bag = &mapping["shiny gold"];
    let mut cur_bag_types = vec![(gold_bag, 1)];
    while !cur_bag_types.is_empty() {
        let mut new_bag = vec![];
        for (bag, amount) in cur_bag_types {
            // println!("a shiny gold bag can hold {} of {}", amount, bag.name);
            count += amount;
            for (child, child_amount) in &bag.children {
                new_bag.push((&mapping[child], child_amount * amount));
            }
        }
        cur_bag_types = new_bag;
    }

    count - 1
}

#[derive(Debug)]
struct BagRule {
    /// The identifier of the bag
    name: String,
    /// child => amount
    children: HashMap<String, usize>,
}


impl BagRule {
    fn parse_line(line: &str) -> Self {
        lazy_static! {
            static ref BAG_REGEX: Regex = Regex::new(r"(\d+) (\w+\s\w+) bags?").unwrap();
        }

        let parts: Vec<_> = line.split("contain").collect();
        let name = &parts[0][..(parts[0].len() - " bags ".len())];
        // eprintln!("name = {:#?}", name);
        let mut sub_bags = HashMap::new();
        for caps in BAG_REGEX.captures_iter(parts[1]) {
            let key: String = (&caps[2]).to_string();
            sub_bags.insert(key, usize::from_str(&caps[1]).unwrap());
        }
        Self {
            name: name.to_string(),
            children: sub_bags,
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::io::Error;
    use super::*;

    fn get_input() -> Vec<BagRule> {
        let input = read_to_string("inputs/day7.txt").unwrap();
        let lines = input.lines();
        let bagrules = lines.map(|l| BagRule::parse_line(l)).collect();
        bagrules
    }

    #[test]
    fn test_part_one() -> Result<(), Error> {
        let bagrules = get_input();
        // eprintln!("bagrules = {:#?}", bagrules);
        let solution = solve_part_one(bagrules);
        println!("[day7] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let bagrules = get_input();
        let solution = solve_part_two(bagrules);
        println!("[day7] solution part 2: {}", solution);
        Ok(())
    }
}