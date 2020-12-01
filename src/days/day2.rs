fn solve_part_one(expenses: Vec<isize>) -> isize {
    todo!()
}

fn solve_part_two(expenses: Vec<isize>) -> isize {
    todo!()
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::io::Error;
    use super::{solve_part_one, solve_part_two};
    
    #[test]
    fn test_part_one() -> Result<(), Error> {
        let input = read_to_string("inputs/day2.txt")?;
        let lines = input.split_ascii_whitespace();
        // todo transform lines to our input somehow
        let expenses = todo!();
        let solution = solve_part_one(expenses);
        println!("solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let input = read_to_string("inputs/day2.txt")?;
        let lines = input.split_ascii_whitespace();
        // todo transform lines to our input somehow
        let expenses = todo!();
        let solution = solve_part_two(expenses);
        println!("solution part 2: {}", solution);
        Ok(())
    }

}