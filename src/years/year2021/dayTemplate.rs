use super::util::AResult;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Output = isize;
const DAY: usize = 4;

fn solve_part_one(input: Input) -> Output {
    todo!()
}

fn solve_part_two(input: Input) -> Output {
    todo!()
}

struct Input {}
impl Input {}

impl FromStr for Input {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> AResult<Self> {
        Ok(Input {})
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

    fn parse_input() -> std::result::Result<Input, Error> {
        let input = read_input(super::DAY)?;
        Ok(Input::from_str(input.trim()).expect("Could not parse input"))
    }

    #[test]
    fn test_part_one() -> Result<(), Error> {
        let input = parse_input()?;
        let solution = solve_part_one(input);
        println!("solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let input = parse_input()?;
        let solution = solve_part_two(input);
        println!("solution part 2: {}", solution);
        Ok(())
    }
}
