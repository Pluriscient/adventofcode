fn solve_part_one(expenses: Vec<isize>) -> isize {
    for i in 0..expenses.len() {
        for j in (i + 1)..expenses.len() {
            if expenses[i] + expenses[j] == 2020 {
                return expenses[i] * expenses[j];
            }
        }
    }
    unreachable!()
}

fn solve_part_two(expenses: Vec<isize>) -> isize {
    for i in 0..expenses.len() {
        for j in (i + 1)..expenses.len() {
            for k in (j + 1)..expenses.len() {
                if [i, j, k].iter().map(|&x| expenses[x]).sum::<isize>() == 2020 {
                    return [i, j, k].iter().map(|&x| expenses[x]).product::<isize>();
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::error::Error;
    use super::*;
    use itertools::Itertools;
    use std::str::FromStr;
    use crate::years::year2020::util::read_input;

    fn parse_input() -> Result<Vec<isize>, Box<dyn Error>> {
        let input = read_input(1)?;
        let lines = input.lines();
        Ok(lines.filter_map(|x| isize::from_str(x).ok()).collect_vec())
    }

    #[test]
    fn test_part_one() -> Result<(), Box<dyn Error>> {
        let expenses = parse_input()?;
        let solution = solve_part_one(expenses);
        println!("[day1] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        let expenses = parse_input()?;
        let solution = solve_part_two(expenses);
        println!("[day1] solution part 2: {}", solution);
        Ok(())
    }
}