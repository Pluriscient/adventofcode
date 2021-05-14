use std::error::Error;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use itertools::Itertools;


fn solve_part_one(expressions: &[String]) -> isize {
    expressions.iter().cloned().map(solve_expression).collect::<Result<Vec<_>, _>>().unwrap().iter().sum()
}

fn solve_part_two(expressions: &[String]) -> isize {
    expressions.iter()
        // .map(add_plus_parens)
        // .inspect(|x| println!("testing {}", x))
        .cloned()
        .map(solve_expression_2)
        // .take(1)
        .collect::<Result<Vec<_>, _>>().unwrap().iter()
        .sum()
}


fn solve_expression_2(expression: String) -> Result<isize, Box<dyn Error>> {
    lazy_static! {
    static ref TERM_REGEX: Regex = Regex::new(r"(\([^()\\\r\n]*(?:\\.[^()\\\r\n]*)*\))").unwrap();
    }


    let mut expr = expression.clone();
    while let Some(caps) = TERM_REGEX.captures(&expr) {
        // none-nested values
        let group = caps.get(0).unwrap().as_str();
        let terms = &group[1..group.len() - 1];
        let sol = solve_terms_2(terms)?;
        // println!("replacing {} with {} ", group, sol.to_string());
        expr = expr.replace(group, sol.to_string().as_str());
        // println!("expr: {}", expr);
    }
    solve_terms_2(expr.as_str())
}

enum Operator {
    Plus,
    Times,
}

fn solve_terms_2(expr: &str) -> Result<isize, Box<dyn Error>> {
    let plus_regex = Regex::new(r"(\d+)\s*\+\s*(\d+)").unwrap();
    let mut expr = expr.to_string();
    while let Some(caps) = plus_regex.captures(expr.as_str()) {
        let evaled = isize::from_str(caps.get(1).unwrap().as_str())? + isize::from_str(caps.get(2).unwrap().as_str())?;
        expr = expr.replace(caps.get(0).unwrap().as_str(), evaled.to_string().as_str());
    }
    // println!("evaling {}", expr);
    return solve_terms(expr.as_str());
}


fn solve_expression(expression: String) -> Result<isize, Box<dyn Error>> {
    lazy_static! {
    static ref TERM_REGEX: Regex = Regex::new(r"(\([^()\\\r\n]*(?:\\.[^()\\\r\n]*)*\))").unwrap();
    }


    let mut expr = expression.clone();
    while let Some(caps) = TERM_REGEX.captures(&expr) {
        // none-nested values
        let group = caps.get(0).unwrap().as_str();
        let terms = &group[1..group.len() - 1];
        let sol = solve_terms(terms)?;
        // println!("replacing {} with {} ", group, sol.to_string());
        expr = expr.replace(group, sol.to_string().as_str());
        // println!("expr: {}", expr);
    }
    solve_terms(expr.as_str())
}

fn solve_terms(expr: &str) -> Result<isize, Box<dyn Error>> {
    let mut it = expr.split_ascii_whitespace();
    let mut res = isize::from_str(it.next().ok_or("not a single term")?)?;
    while let Some(operator) = it.next() {
        let next_val = it.next().ok_or("operator should have right side")?;
        let next_val = isize::from_str(next_val)?;
        match operator.as_bytes()[0] {
            b'+' => res += next_val,
            b'-' => res -= next_val,
            b'*' => res *= next_val,
            b'/' => res *= next_val,

            _ => panic!("non-operator at operator")
        }
    }

    Ok(res)
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::error::Error;
    use super::*;

    fn parse_input() -> Result<Vec<String>, Box<dyn Error>> {
        let input = read_to_string("inputs/day18.txt")?;
        let lines = input.lines().map(|s| s.to_string());

        Ok(lines.collect_vec())
    }

    #[test]
    fn test_part_one() -> Result<(), Box<dyn Error>> {
        let expressions = parse_input()?;
        let solution = solve_part_one(&expressions);
        println!("[day18] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        let expressions = parse_input()?;
        let solution = solve_part_two(&expressions);
        println!("[day18] solution part 2: {}", solution);
        Ok(())
    }
}