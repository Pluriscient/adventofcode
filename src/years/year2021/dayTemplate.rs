use super::util::AResult;
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Output = isize;
const DAY: usize = 3;

fn solve_part_one(inputs: &[Input]) -> Output {
    todo!()
}

fn solve_part_two(inputs: &[Input]) -> Output {
    todo!()
}

struct Input {
    size: isize,
    direction: Direction,
}
impl Input {
   
}

impl FromStr for Input {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> AResult<Self> {
        todo!()
        Ok(Self { size, direction })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::super::util::{read_input, read_input_test};
    use super::*;
    use super::{solve_part_one, solve_part_two};
    use std::io::Error;
    use std::str::FromStr;

    fn parse_input() -> std::result::Result<Vec<Input>, Error> {
        let input = read_input(super::DAY)?;
        Ok(input
            .trim()
            .lines()
            .map(Input::from_str)
            .collect::<AResult<Vec<Input>>>()
            .unwrap())
    }

    #[test]
    fn test_part_one() -> Result<(), Error> {
        let input = parse_input()?;
        let solution = solve_part_one(&input);
        println!("solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let input = parse_input()?;
        let solution = solve_part_two(&input);
        println!("solution part 2: {}", solution);
        Ok(())
    }
}
