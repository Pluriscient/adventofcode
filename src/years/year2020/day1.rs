fn solve_part_one(expenses: Vec<isize>) -> isize {
    for i in 0..expenses.len() {
        for j in (i + 1)..expenses.len() {
            if expenses[i] + expenses[j] == 2020 {
                return expenses[i] * expenses[j];
            }
        }
    }
    -1
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
    -1
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::io::Error;
    use std::str::FromStr;
    use super::*;
    // use crate::days::day1::{solve_part_one, solve_part_two};

    #[test]
    fn test_day() -> Result<(), Error> {
        // read the input
        let input = read_to_string("inputs/day1.txt")?;
        let lines = input.split_ascii_whitespace()
            .filter(|l| !l.trim().is_empty())
            .collect::<Vec<&str>>();
        let numbers = lines.iter()
            .map(|l| isize::from_str(l).unwrap());
        let numbers: Vec<isize> = numbers.collect();
        println!("input: \n{:?}", numbers);
        let solution_one = solve_part_one(numbers.clone());
        println!("solution_1: {:?}", solution_one);
        let solution_two = solve_part_two(numbers);
        println!("solution_2: {:?}", solution_two);
        Ok(())
    }
}