use std::collections::HashMap;
use itertools::Itertools;
use std::str::{FromStr, Lines};
use crate::years::year2020::day19::Rule::{And,  Or, Text};
use itertools::__std_iter::FromIterator;
// use std::io::Lines;

fn solve_part_one(ruleset: RuleSet, messages: Vec<String>) -> usize {
    let actual_rule = ruleset.rules.get(&0).unwrap();
    // messages.iter().filter(|x| ruleset.fits_rule(x, actual_rule)).count()
    todo!()
}

fn solve_part_two(ruleset: RuleSet, messages: Vec<String>) -> usize {
    todo!()
}

struct RuleSet {
    rules: HashMap<usize, Rule>
}


impl RuleSet {
    fn fits_rule(&self, current: &str, rule: &Rule) -> Option<String> {
        if current.is_empty() {
            return None;
        }
        match rule {
            Text(s) => {
                if let Some(rest) = current.strip_prefix(s) {
                    return Some(rest.to_string());
                }
            }
            And(rule_refs) => {
                let mut remainder = current.clone();
                for rule_ref in rule_refs {
                    let r = &self.rules[rule_ref];
                    if let Some(rest) = self.fits_rule(remainder, r) {
                        // remainder = rest.as_str();
                    } else {
                        return None;
                    }
                }
            }
            Or(left, right) => {

            }
        }

        None
    }
}

enum Rule {
    Text(String),
    /// usize refers to the index of the rule
    And(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}

impl<'a> FromIterator<&'a str> for RuleSet {
    fn from_iter<T: IntoIterator<Item=&'a str>>(iterator: T) -> Self {
        let mut map: HashMap<usize, Rule> = HashMap::new();
        for line in iterator {
            let parts = line.split(":").collect_vec();
            let id = usize::from_str(parts[0]).expect("id not a valid num");
            let rule = Rule::from(parts[1]);
            map.insert(id, rule);
        }

        Self { rules: map }
    }
}

impl From<&str> for Rule {
    /// Expects to be given a string of the form
    /// 4 1 5
    /// (so not 0:, that should be handled from ruleset)
    fn from(line: &str) -> Self {
        let parts = line.trim().split_ascii_whitespace().collect_vec();
        if parts.len() == 1 {
            return if let Ok(x) = usize::from_str(parts[0]) {
                And(vec![x])
            } else {
                Text(parts[0][1..parts[0].len() - 1].to_string())
            };
        }
        let mut iter = parts.iter();
        let before = iter
            .take_while_ref(|x| *x != &"|")

            .map(|x| usize::from_str(x)).collect::<Result<Vec<_>, _>>().unwrap();
        // .collect<Result<Vec<_>, _>>()
        // .unwrap();
        let after: Vec<usize> = iter
            .map(|x| usize::from_str(x))
            .collect::<Result<Vec<_>, _>>().unwrap();
        return if after.len() > 0 {
            Or(before, after)
        } else {
            And(before)
        };
    }
}


#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::error::Error;
    use super::*;

    fn parse_input() -> Result<(RuleSet, Vec<String>), Box<dyn Error>> {
        let input = read_to_string("../../../inputs/year2020/day19.txt")?;
        let mut lines = input.lines();
        let ruleset = lines.take_while_ref(|l| !l.trim().is_empty()).collect::<RuleSet>();
        let messages = lines
            .map(|x| x.to_string()).collect_vec();
        Ok((ruleset, messages))
    }

    #[test]
    fn test_part_one() -> Result<(), Box<dyn Error>> {
        let (set, msgs) = parse_input()?;
        let solution = solve_part_one(set, msgs);
        println!("[day19] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        let (set, msgs) = parse_input()?;
        let solution = solve_part_two(set, msgs);
        println!("[day19] solution part 2: {}", solution);
        Ok(())
    }
}