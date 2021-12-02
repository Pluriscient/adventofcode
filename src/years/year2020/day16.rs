use regex::{Regex, Captures, Match};
use std::str::FromStr;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use factorial::Factorial;
use std::hint::unreachable_unchecked;
// use core::num::bignum::*;
// use core::num::dec2flt::rawfp::big_to_fp;

fn solve_part_one(rules: Vec<Rule>, _ticket: Ticket, tickets: Vec<Ticket>) -> usize {
    tickets.iter().flatten().cloned().filter(|field| rules.iter().all(|rule| !rule.is_valid(*field)))
        .sum()
}


fn gen(valid_tickets: &[Ticket], rules: &mut HashSet<Rule>, cur: usize, current_res: &mut Vec<Rule>) -> Option<Vec<Rule>> {
    if cur == valid_tickets[0].len() {
        return Some((*current_res).clone());
    }
    let cur_field = cur;
    let mut pot_rules = rules.clone();
    for ticket in valid_tickets {
        let val = ticket[cur_field];
        pot_rules.drain_filter(|rule| !rule.is_valid(val));
    }
    if pot_rules.len() == 0 {
        return None;
    }

    pot_rules.iter().filter_map(|r| {
        // println!("trying with {:?}", r);
        // println!("{:?}", current_res);
        rules.remove(r);
        current_res.push(r.clone());
        if let Some(x) = gen(valid_tickets, rules, cur + 1, current_res) {
            Some(x)
        } else {
            println!("could not fit rule {:?} at pos {}", r, cur);
            current_res.remove(cur);
            rules.insert(r.clone());
            None
        }
    }).next()
}

fn determine_fields(valid_tickets: &Vec<Ticket>, rules: &Vec<Rule>) -> HashMap<Rule, usize> {
    // first determine if there is any need to continue
    // let rules = fields.keys().cloned().collect_vec();
    println!("{}", rules.len());
    let mut res = vec![];
    let mut rules = rules.iter().cloned().collect::<HashSet<_>>();
    if let Some(x) = gen(valid_tickets, &mut rules, 0, &mut res) {
        println!("{:?}", x);

        todo!()
    } else {
        panic!("couldn't find anything")
    }
}

fn solve_part_two(rules: Vec<Rule>, ticket: Ticket, tickets: Vec<Ticket>) -> usize {
    let valid_tickets = tickets.iter()
        .filter(|t| t.iter().all(|field| rules.iter().any(|rule| rule.is_valid(*field))))
        .cloned()
        .collect_vec();
    println!("valid tickets: {:?}", valid_tickets);
    let mut fields: HashMap<Rule, HashSet<usize>> = rules.iter().map(|rule| (rule.clone(), HashSet::new())).collect::<HashMap<_, _>>();
    let res: HashMap<Rule, usize> = HashMap::new();
    for (rule, locs) in fields.iter_mut() {
        let valid_locs: HashSet<usize> = valid_tickets.iter()
            .flat_map(|t| t.iter().enumerate().filter_map(|(i, p)| if rule.is_valid(*p) { Some(i) } else { None }))
            .collect();
        // println!("rule {:?} is valid at indices {:?}", rule, valid_locs);
        locs.extend(valid_locs);
    }


    let ruleset = determine_fields(&valid_tickets, &rules);
    println!("{:?}", ruleset);
    // println!("found rules {:?}", ruleset);
    // let mut fields = vec![-1; ticket.len()];
    res.iter().filter(|(rule, val)| rule.field.starts_with("cl"))
        .inspect(|(rule, val)| println!("index {:?} associated with {:?}", val, rule))
        .filter_map(|x| Some(*x.1))
        .map(|i| ticket[i])
        .product()
}

type Ticket = Vec<usize>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Rule {
    field: String,
    option_a: (usize, usize),
    option_b: (usize, usize),
}

impl Rule {
    fn read_from(line: &str) -> Option<Self> {
        lazy_static! {
            static ref RULE_REGEX: Regex = Regex::new(r"(^.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        }

        let caps: Captures = RULE_REGEX.captures(line)?;
        let to_num = |cap: Match| usize::from_str(cap.as_str()).ok();
        Some(Self {
            field: caps.get(1)?.as_str().to_string(),
            option_a: (to_num(caps.get(2)?)?, to_num(caps.get(3)?)?),
            option_b: (to_num(caps.get(4)?)?, to_num(caps.get(5)?)?),
        })
    }

    fn fits(rules: &Vec<&Self>, ticket: &Ticket) -> bool {
        for i in 0..ticket.len() {
            let rule = rules[i];
            let val = ticket[i];
            if !rule.is_valid(val) {
                return false;
            }
        }
        true
    }

    fn is_valid(&self, val: usize) -> bool {
        (val >= self.option_a.0 && val <= self.option_a.1) ||
            (val >= self.option_b.0 && val <= self.option_b.1)
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::error::Error;
    use super::*;
    use itertools::Itertools;

    fn parse_input() -> Result<(Vec<Rule>, Ticket, Vec<Ticket>), Box<dyn Error>> {
        let input = read_to_string("../../../inputs/year2020/day16.txt")?;
        let mut lines = input.lines();
        let rules = lines.by_ref().take_while(|line| !line.trim().is_empty())
            .filter_map(|line| Rule::read_from(line)).collect_vec();
        lines.next().ok_or("no next line")?;
        let ticket = lines.next().ok_or("no next")?.split(",")
            .map(|s| usize::from_str(s)).collect::<Result<Ticket, _>>()?;
        lines.next().ok_or("no next line")?;
        lines.next().ok_or("no next line")?;
        let tickets = lines.map(|l| l.split(",")
            .map(|s| usize::from_str(s)).collect::<Result<Ticket, _>>())
            .collect::<Result<Vec<Ticket>, _>>()?;
        Ok((rules, ticket, tickets))
    }

    #[test]
    fn test_part_one() -> Result<(), Box<dyn Error>> {
        let (rules, ticket, tickets) = parse_input()?;
        // println!("rules: {:?}, ticket: {:?}, tickets: {:?}", rules, ticket, tickets);
        let solution = solve_part_one(rules, ticket, tickets);
        println!("[day16] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        let (rules, ticket, tickets) = parse_input()?;
        let solution = solve_part_two(rules, ticket, tickets);
        println!("[day16] solution part 2: {}", solution);
        Ok(())
    }
}